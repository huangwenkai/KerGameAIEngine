//! Rendering system with wgpu
//! Supports both windowed and headless rendering

use anyhow::Result;
use wgpu::util::DeviceExt;

/// Render context (headless or windowed)
pub struct RenderContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub texture_format: wgpu::TextureFormat,
    pub width: u32,
    pub height: u32,
}

impl RenderContext {
    /// Create headless render context (renders to texture)
    pub async fn new_headless(width: u32, height: u32) -> Result<Self> {
        log::info!("Initializing headless render context ({}×{})", width, height);
        
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN | wgpu::Backends::GL,
            ..Default::default()
        });
        
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: true, // Enable fallback for CI
                compatible_surface: None,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("Failed to find suitable adapter"))?;
        
        log::info!("Using adapter: {:?}", adapter.get_info());
        
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Headless Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await?;
        
        Ok(Self {
            device,
            queue,
            texture_format: wgpu::TextureFormat::Rgba8UnormSrgb,
            width,
            height,
        })
    }
    
    /// Create render texture
    pub fn create_render_texture(&self) -> wgpu::Texture {
        self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Render Texture"),
            size: wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.texture_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        })
    }
    
    /// Capture texture to PNG file
    pub fn capture_texture(&self, texture: &wgpu::Texture, path: &str) -> Result<()> {
        let width = self.width;
        let height = self.height;
        
        // Create buffer to copy texture data
        let bytes_per_pixel = 4; // RGBA8
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padded_bytes_per_row = (unpadded_bytes_per_row + align - 1) / align * align;
        
        let buffer_size = (padded_bytes_per_row * height) as u64;
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Capture Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        
        // Copy texture to buffer
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Capture Encoder"),
        });
        
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            texture.size(),
        );
        
        self.queue.submit(Some(encoder.finish()));
        
        // Map buffer and read data
        let buffer_slice = buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        
        self.device.poll(wgpu::Maintain::Wait);
        rx.recv()??;
        
        let data = buffer_slice.get_mapped_range();
        
        // Copy to unpadded buffer
        let mut pixels = vec![0u8; (width * height * 4) as usize];
        for y in 0..height {
            let src_offset = (y * padded_bytes_per_row) as usize;
            let dst_offset = (y * unpadded_bytes_per_row) as usize;
            pixels[dst_offset..dst_offset + unpadded_bytes_per_row as usize]
                .copy_from_slice(&data[src_offset..src_offset + unpadded_bytes_per_row as usize]);
        }
        
        drop(data);
        buffer.unmap();
        
        // Save as PNG
        image::save_buffer(
            path,
            &pixels,
            width,
            height,
            image::ColorType::Rgba8,
        )?;
        
        log::info!("Screenshot saved to {}", path);
        Ok(())
    }
}

/// Pixel-perfect camera (integer coordinates)
#[derive(Debug, Clone)]
pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub viewport_width: u32,
    pub viewport_height: u32,
}

impl Camera {
    pub fn new(viewport_width: u32, viewport_height: u32) -> Self {
        Self {
            x: 0,
            y: 0,
            viewport_width,
            viewport_height,
        }
    }
    
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

/// Sprite vertex
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

/// Sprite batch renderer
pub struct SpriteBatch {
    vertices: Vec<SpriteVertex>,
    indices: Vec<u16>,
}

impl SpriteBatch {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
    
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
    
    /// Add a colored quad (sprite)
    pub fn add_quad(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        let base_idx = self.vertices.len() as u16;
        
        // Add vertices (CCW winding)
        self.vertices.push(SpriteVertex {
            position: [x, y],
            color,
        });
        self.vertices.push(SpriteVertex {
            position: [x + w, y],
            color,
        });
        self.vertices.push(SpriteVertex {
            position: [x + w, y + h],
            color,
        });
        self.vertices.push(SpriteVertex {
            position: [x, y + h],
            color,
        });
        
        // Add indices (two triangles)
        self.indices.extend_from_slice(&[
            base_idx,
            base_idx + 1,
            base_idx + 2,
            base_idx,
            base_idx + 2,
            base_idx + 3,
        ]);
    }
    
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
    
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
    
    /// Create buffers for rendering
    pub fn create_buffers(&self, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer) {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sprite Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sprite Index Buffer"),
            contents: bytemuck::cast_slice(&self.indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        
        (vertex_buffer, index_buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_movement() {
        let mut camera = Camera::new(800, 600);
        assert_eq!(camera.x, 0);
        assert_eq!(camera.y, 0);
        
        camera.move_by(10, 20);
        assert_eq!(camera.x, 10);
        assert_eq!(camera.y, 20);
    }

    #[test]
    fn sprite_batch_quads() {
        let mut batch = SpriteBatch::new();
        assert_eq!(batch.vertex_count(), 0);
        assert_eq!(batch.index_count(), 0);
        
        batch.add_quad(0.0, 0.0, 10.0, 10.0, [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(batch.vertex_count(), 4);
        assert_eq!(batch.index_count(), 6);
        
        batch.add_quad(10.0, 10.0, 5.0, 5.0, [0.0, 1.0, 0.0, 1.0]);
        assert_eq!(batch.vertex_count(), 8);
        assert_eq!(batch.index_count(), 12);
    }
}
