//! Demo system for stress testing and validation

use crate::{Engine, EngineConfig};
use crate::report::DemoReport;
use anyhow::Result;
use std::time::Instant;

pub trait Demo {
    fn id(&self) -> &str;
    fn description(&self) -> &str;
    fn run(&self, engine: &mut Engine) -> Result<()>;
}

/// M0 demo: Basic scaffold validation
pub struct M0Demo;

impl Demo for M0Demo {
    fn id(&self) -> &str {
        "M0"
    }

    fn description(&self) -> &str {
        "M0 scaffold validation: CLI + headless harness + report generation"
    }

    fn run(&self, engine: &mut Engine) -> Result<()> {
        log::info!("Running M0 demo: {}", self.description());
        
        // Run 600 ticks (10 seconds at 60 TPS)
        for _ in 0..600 {
            engine.tick()?;
        }
        
        log::info!("M0 demo completed: {} ticks", engine.tick_count());
        Ok(())
    }
}

/// M1 demo: Command execution and replay validation
pub struct M1Demo;

impl Demo for M1Demo {
    fn id(&self) -> &str {
        "M1"
    }

    fn description(&self) -> &str {
        "M1 command replay: 1000 commands with deterministic execution"
    }

    fn run(&self, engine: &mut Engine) -> Result<()> {
        log::info!("Running M1 demo: {}", self.description());
        
        // Phase 1: Spawn 100 entities with random positions
        log::info!("Phase 1: Spawning 100 entities");
        for _ in 0..100 {
            let x = engine.rng.gen_range(0.0..1000.0);
            let y = engine.rng.gen_range(0.0..1000.0);
            engine.queue_command(crate::commands::Command::SpawnEntity { x, y });
        }
        engine.tick()?;
        
        assert_eq!(engine.world.entity_count(), 100, "Should have 100 entities");
        
        // Phase 2: Apply random velocities to entities (900 commands)
        log::info!("Phase 2: Applying velocities");
        for tick in 0..30 {
            for entity_id in 1..=100 {
                if tick % 3 == 0 {
                    let vx = engine.rng.gen_range(-10.0..10.0);
                    let vy = engine.rng.gen_range(-10.0..10.0);
                    engine.queue_command(crate::commands::Command::SetVelocity {
                        entity_id,
                        vx,
                        vy,
                    });
                }
            }
            engine.tick()?;
        }
        
        // Phase 3: Simulate for 10 more seconds (600 ticks)
        log::info!("Phase 3: Simulating movement");
        for _ in 0..600 {
            engine.tick()?;
        }
        
        log::info!("M1 demo completed: {} ticks, {} entities, final state hash: {}",
                   engine.tick_count(),
                   engine.world.entity_count(),
                   engine.world.state_hash());
        
        // Verify state is non-trivial
        assert!(engine.world.entity_count() > 0, "World should have entities");
        
        Ok(())
    }
}

/// M2 demo: ECS stress test with 10k entities
pub struct M2Demo;

impl Demo for M2Demo {
    fn id(&self) -> &str {
        "M2"
    }

    fn description(&self) -> &str {
        "M2 ECS stress: 10,000 entities with movement + collision systems"
    }

    fn run(&self, engine: &mut Engine) -> Result<()> {
        log::info!("Running M2 demo: {}", self.description());
        
        // Spawn 10,000 entities with random positions and velocities
        log::info!("Spawning 10,000 entities...");
        for _ in 0..10_000 {
            let x = engine.rng.gen_range(0.0..1000.0);
            let y = engine.rng.gen_range(0.0..1000.0);
            let vx = engine.rng.gen_range(-50.0..50.0);
            let vy = engine.rng.gen_range(-50.0..50.0);
            let health = engine.rng.gen_range(50.0..200.0);
            
            engine.ecs.spawn_entity(x, y, vx, vy, health);
        }
        
        log::info!("Entities spawned: {}", engine.ecs.entity_count());
        assert_eq!(engine.ecs.entity_count(), 10_000);
        
        // Simulate for 600 ticks (10 seconds @ 60 TPS)
        log::info!("Simulating 600 ticks...");
        let start = std::time::Instant::now();
        
        for tick in 0..600 {
            engine.tick()?;
            
            if (tick + 1) % 100 == 0 {
                log::info!("Tick {}/600 ({}ms elapsed)", 
                           tick + 1, 
                           start.elapsed().as_millis());
            }
        }
        
        let elapsed = start.elapsed();
        let avg_tick_ms = elapsed.as_secs_f64() * 1000.0 / 600.0;
        
        log::info!("M2 demo completed: {} ticks, {} entities",
                   engine.tick_count(),
                   engine.ecs.entity_count());
        log::info!("Performance: {:.3}ms per tick (avg), {:.1} FPS capable",
                   avg_tick_ms,
                   1000.0 / avg_tick_ms);
        log::info!("ECS state hash: {}", engine.ecs.state_hash());
        
        // Verify performance target: < 2ms per tick
        assert!(avg_tick_ms < 2.0, 
                "Performance target not met: {:.3}ms > 2.0ms per tick", 
                avg_tick_ms);
        
        Ok(())
    }
}

/// M3 demo: Render pipeline with headless wgpu
pub struct M3Demo;

impl Demo for M3Demo {
    fn id(&self) -> &str {
        "M3"
    }

    fn description(&self) -> &str {
        "M3 render pipeline: sprite batching + camera + headless capture"
    }

    fn run(&self, engine: &mut Engine) -> Result<()> {
        log::info!("Running M3 demo: {}", self.description());
        
        // Initialize headless render context
        log::info!("Initializing wgpu render context (headless)...");
        
        // Try to create real render context, fall back to mock if unavailable
        let use_mock = match pollster::block_on(crate::render::RenderContext::new_headless(800, 600)) {
            Ok(ctx) => {
                log::info!("Real GPU rendering available");
                drop(ctx); // We'll recreate it in the actual render loop
                false
            }
            Err(e) => {
                log::warn!("GPU not available ({}), using mock renderer for CI", e);
                true
            }
        };
        
        if use_mock {
            // Mock renderer path (for CI/headless environments without GPU)
            log::info!("Using mock render path (architecture validation only)");
            
            let mut camera = crate::render::Camera::new(800, 600);
            
            log::info!("Simulating 600 ticks with mock rendering...");
            let start = std::time::Instant::now();
            
            for tick in 0..600 {
                engine.tick()?;
                
                if tick % 60 == 0 {
                    camera.move_by(10, 5);
                }
                
                // Mock render (just count batches)
                if tick % 100 == 0 {
                    let mut batch = crate::render::SpriteBatch::new();
                    
                    // Same scene as real renderer
                    for i in 0..10 {
                        let y = i as f32 * 60.0;
                        let brightness = 0.2 + (i as f32 * 0.05);
                        batch.add_quad(0.0, y, 800.0, 60.0, [0.0, 0.0, brightness, 1.0]);
                    }
                    
                    for i in 0..20 {
                        let x = i as f32 * 40.0 - (camera.x % 40) as f32;
                        batch.add_quad(x, 0.0, 2.0, 600.0, [0.5, 0.5, 0.5, 0.3]);
                    }
                    
                    for i in 0..20 {
                        let x = (i * 40) as f32 + (tick % 800) as f32;
                        let y = 250.0 + ((tick + i * 30) as f32 * 0.1).sin() * 100.0;
                        let color = [
                            ((i * 13) % 256) as f32 / 255.0,
                            ((i * 27) % 256) as f32 / 255.0,
                            ((i * 41) % 256) as f32 / 255.0,
                            1.0,
                        ];
                        batch.add_quad(x, y, 20.0, 20.0, color);
                    }
                    
                    // Generate mock frame (1x1 colored pixel based on tick)
                    if tick == 0 || tick == 200 || tick == 400 || tick == 599 {
                        let path = format!("/workspace/m3-frame-{:04}.png", tick);
                        let r = ((tick * 13) % 256) as u8;
                        let g = ((tick * 27) % 256) as u8;
                        let b = ((tick * 41) % 256) as u8;
                        image::save_buffer(
                            &path,
                            &[r, g, b, 255],
                            1,
                            1,
                            image::ColorType::Rgba8,
                        )?;
                        log::info!("Mock frame {} saved ({}×{} quad batch, camera: {},{}) to {}",
                                   tick, batch.vertex_count(), batch.index_count(),
                                   camera.x, camera.y, path);
                    }
                }
                
                if (tick + 1) % 100 == 0 {
                    log::info!("Tick {}/600 ({}ms elapsed)", 
                               tick + 1, 
                               start.elapsed().as_millis());
                }
            }
            
            let elapsed = start.elapsed();
            log::info!("M3 demo completed (mock): {} ticks", engine.tick_count());
            log::info!("Camera position: ({}, {})", camera.x, camera.y);
            log::info!("Elapsed: {}ms", elapsed.as_millis());
            log::info!("Frames captured: 4 (mock 1x1 pixel deterministic placeholders)");
            
            return Ok(());
        }
        
        // Real GPU render path (when GPU available)
        log::info!("Initializing wgpu render context (headless)...");
        let render_ctx = pollster::block_on(crate::render::RenderContext::new_headless(800, 600))?;
        
        // Load shader
        let shader_source = include_str!("../shaders/sprite.wgsl");
        let shader = render_ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Sprite Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });
        
        // Create render pipeline
        let pipeline_layout = render_ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Sprite Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        
        let pipeline = render_ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Sprite Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<crate::render::SpriteVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: render_ctx.texture_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        
        log::info!("Render pipeline created");
        
        // Create camera
        let mut camera = crate::render::Camera::new(800, 600);
        
        // Simulate with rendering (600 ticks = 10 seconds)
        log::info!("Simulating 600 ticks with rendering...");
        let start = std::time::Instant::now();
        
        for tick in 0..600 {
            // Tick simulation
            engine.tick()?;
            
            // Move camera (simple pattern)
            if tick % 60 == 0 {
                camera.move_by(10, 5);
            }
            
            // Render frame (every 10 ticks for demo, not every tick)
            if tick % 100 == 0 {
                let mut batch = crate::render::SpriteBatch::new();
                
                // Layer 0: Background (blue gradient)
                for i in 0..10 {
                    let y = i as f32 * 60.0;
                    let brightness = 0.2 + (i as f32 * 0.05);
                    batch.add_quad(0.0, y, 800.0, 60.0, [0.0, 0.0, brightness, 1.0]);
                }
                
                // Layer 1: Grid pattern (white lines)
                for i in 0..20 {
                    let x = i as f32 * 40.0 - (camera.x % 40) as f32;
                    batch.add_quad(x, 0.0, 2.0, 600.0, [0.5, 0.5, 0.5, 0.3]);
                }
                
                // Layer 2: Moving sprites (colored squares)
                for i in 0..20 {
                    let x = (i * 40) as f32 + (tick % 800) as f32;
                    let y = 250.0 + ((tick + i * 30) as f32 * 0.1).sin() * 100.0;
                    let color = [
                        ((i * 13) % 256) as f32 / 255.0,
                        ((i * 27) % 256) as f32 / 255.0,
                        ((i * 41) % 256) as f32 / 255.0,
                        1.0,
                    ];
                    batch.add_quad(x, y, 20.0, 20.0, color);
                }
                
                // Create buffers and render
                let (vertex_buffer, index_buffer) = batch.create_buffers(&render_ctx.device);
                let render_texture = render_ctx.create_render_texture();
                let view = render_texture.create_view(&wgpu::TextureViewDescriptor::default());
                
                let mut encoder = render_ctx.device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    }
                );
                
                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.1,
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                    
                    render_pass.set_pipeline(&pipeline);
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    render_pass.draw_indexed(0..batch.index_count() as u32, 0, 0..1);
                }
                
                render_ctx.queue.submit(Some(encoder.finish()));
                
                // Capture frame (at tick 0, 200, 400, 599)
                if tick == 0 || tick == 200 || tick == 400 || tick == 599 {
                    let path = format!("/workspace/m3-frame-{:04}.png", tick);
                    render_ctx.capture_texture(&render_texture, &path)?;
                }
            }
            
            if (tick + 1) % 100 == 0 {
                log::info!("Tick {}/600 ({}ms elapsed)", 
                           tick + 1, 
                           start.elapsed().as_millis());
            }
        }
        
        let elapsed = start.elapsed();
        
        log::info!("M3 demo completed: {} ticks", engine.tick_count());
        log::info!("Camera position: ({}, {})", camera.x, camera.y);
        log::info!("Elapsed: {}ms", elapsed.as_millis());
        log::info!("Frames captured: 4 (tick 0, 200, 400, 599)");
        
        Ok(())
    }
}

/// M4 demo: Chunk streaming + terrain generation
pub struct M4Demo;

impl Demo for M4Demo {
    fn id(&self) -> &str {
        "M4"
    }

    fn description(&self) -> &str {
        "M4 pixel/chunk world: 4px cells + terrain generation + chunk streaming"
    }

    fn run(&self, engine: &mut Engine) -> Result<()> {
        log::info!("Running M4 demo: {}", self.description());
        log::info!("🔒 Cell size: 4px (locked decision)");
        log::info!("Chunk size: 128×128 cells = 512×512 screen pixels");
        
        let terrain_gen = crate::terrain::TerrainGenerator::new(engine.config.seed);
        
        log::info!("Phase 1: Generating initial terrain (5×5 chunks around origin)");
        let start = std::time::Instant::now();
        
        for cy in -2..=2 {
            for cx in -2..=2 {
                let coord = crate::chunk::ChunkCoord::new(cx, cy);
                terrain_gen.generate_chunk(&mut engine.chunk_world, coord);
            }
        }
        
        log::info!("Generated {} chunks in {}ms", 
                   engine.chunk_world.chunk_count(),
                   start.elapsed().as_millis());
        
        log::info!("Phase 2: Simulating 600 ticks with chunk streaming");
        let mut focus_x = 0;
        let mut focus_y = 0;
        let start_sim = std::time::Instant::now();
        
        for tick in 0..600 {
            engine.tick()?;
            
            if tick % 10 == 0 {
                focus_x += 20;
                if tick % 100 == 0 {
                    focus_y += 10;
                }
                
                let chunk_radius = 2;
                for cy in (focus_y / (crate::chunk::CHUNK_SIZE as i32)) - chunk_radius
                    ..=(focus_y / (crate::chunk::CHUNK_SIZE as i32)) + chunk_radius
                {
                    for cx in (focus_x / (crate::chunk::CHUNK_SIZE as i32)) - chunk_radius
                        ..=(focus_x / (crate::chunk::CHUNK_SIZE as i32)) + chunk_radius
                    {
                        let coord = crate::chunk::ChunkCoord::new(cx, cy);
                        terrain_gen.generate_chunk(&mut engine.chunk_world, coord);
                    }
                }
            }
            
            if (tick + 1) % 100 == 0 {
                log::info!("Tick {}/600 - Focus: ({}, {}), Chunks: {}",
                           tick + 1, focus_x, focus_y, engine.chunk_world.chunk_count());
            }
        }
        
        let elapsed = start_sim.elapsed();
        
        log::info!("Phase 3: Sampling terrain for verification");
        let mut sample_hash = 0u64;
        for (x, y) in [(0, 0), (100, 50), (200, 100), (500, 200), (1000, 300)].iter() {
            let material = engine.chunk_world.get_cell(*x, *y);
            sample_hash = sample_hash.wrapping_mul(31).wrapping_add(material as u64);
        }
        
        log::info!("M4 demo completed: {} ticks", engine.tick_count());
        log::info!("Final focus: ({}, {})", focus_x, focus_y);
        log::info!("Total chunks loaded: {}", engine.chunk_world.chunk_count());
        log::info!("Terrain sample hash: {:x}", sample_hash);
        log::info!("Elapsed: {}ms", elapsed.as_millis());
        
        let test_x = 100;
        let test_y = 100;
        let original = engine.chunk_world.get_cell(test_x, test_y);
        engine.chunk_world.dig(test_x, test_y);
        assert_eq!(engine.chunk_world.get_cell(test_x, test_y), crate::chunk::Material::Air);
        engine.chunk_world.place(test_x, test_y, original);
        assert_eq!(engine.chunk_world.get_cell(test_x, test_y), original);
        log::info!("Dig/place verified at ({}, {})", test_x, test_y);
        
        Ok(())
    }
}

/// M5 demo: Character motor + collision + dig/build
pub struct M5Demo;

impl Demo for M5Demo {
    fn id(&self) -> &str {
        "M5"
    }

    fn description(&self) -> &str {
        "M5 character motor: AABB collision + walk/jump + dig tunnel + place blocks"
    }

    fn run(&self, engine: &mut Engine) -> Result<()> {
        log::info!("Running M5 demo: {}", self.description());
        
        // Generate terrain
        let terrain_gen = crate::terrain::TerrainGenerator::new(engine.config.seed);
        log::info!("Generating terrain (3×3 chunks)...");
        for cy in -1..=1 {
            for cx in -1..=1 {
                terrain_gen.generate_chunk(&mut engine.chunk_world, crate::chunk::ChunkCoord::new(cx, cy));
            }
        }
        log::info!("Terrain generated: {} chunks", engine.chunk_world.chunk_count());
        
        // Create character at spawn point (air above ground)
        let mut character = crate::physics::CharacterMotor::new(100.0, -50.0);
        log::info!("Character spawned at ({}, {})", character.aabb.x, character.aabb.y);
        
        // Phase 1: Walk and jump (200 ticks)
        log::info!("Phase 1: Walk and jump (200 ticks)");
        for tick in 0..200 {
            engine.tick()?;
            
            let dt = engine.config.fixed_timestep.as_secs_f32();
            
            // Walk right
            character.move_input(1.0, dt);
            character.apply_friction(dt);
            
            // Jump every 50 ticks
            if tick % 50 == 0 {
                character.jump();
            }
            
            character.update(dt, &mut engine.chunk_world);
        }
        
        log::info!("After phase 1: pos=({:.1}, {:.1}), on_ground={}", 
                   character.aabb.x, character.aabb.y, character.on_ground);
        
        // Phase 2: Dig horizontal tunnel (200 ticks)
        log::info!("Phase 2: Dig horizontal tunnel (200 ticks)");
        let dig_y = 50; // Underground level
        for tick in 200..400 {
            engine.tick()?;
            
            let dt = engine.config.fixed_timestep.as_secs_f32();
            
            // Dig ahead
            if tick % 5 == 0 {
                let dig_x = ((character.aabb.center_x() / 4.0) as i32) + 3;
                engine.queue_command(crate::commands::Command::DigCell {
                    x: dig_x,
                    y: dig_y,
                });
                engine.queue_command(crate::commands::Command::DigCell {
                    x: dig_x,
                    y: dig_y - 1,
                });
                engine.queue_command(crate::commands::Command::DigCell {
                    x: dig_x,
                    y: dig_y + 1,
                });
            }
            
            // Move character
            character.move_input(0.5, dt);
            character.apply_friction(dt);
            character.update(dt, &mut engine.chunk_world);
        }
        
        log::info!("After phase 2: pos=({:.1}, {:.1}), tunnel dug", 
                   character.aabb.x, character.aabb.y);
        
        // Phase 3: Place blocks to build structure (200 ticks)
        log::info!("Phase 3: Place blocks (200 ticks)");
        let build_start_x = (character.aabb.center_x() / 4.0) as i32;
        for tick in 400..600 {
            engine.tick()?;
            
            let dt = engine.config.fixed_timestep.as_secs_f32();
            
            // Build a small platform
            if tick % 10 == 0 {
                let offset = (tick - 400) / 10;
                engine.queue_command(crate::commands::Command::PlaceCell {
                    x: build_start_x + offset,
                    y: dig_y + 5,
                    material: crate::chunk::Material::Stone,
                });
            }
            
            character.update(dt, &mut engine.chunk_world);
        }
        
        log::info!("After phase 3: pos=({:.1}, {:.1}), platform built", 
                   character.aabb.x, character.aabb.y);
        
        // Verify
        let cells_dug = (0..20).filter(|&i| {
            let x = 120 + i * 2;
            engine.chunk_world.get_cell(x, dig_y) == crate::chunk::Material::Air
        }).count();
        
        let blocks_placed = (0..20).filter(|&i| {
            let x = build_start_x + i;
            engine.chunk_world.get_cell(x, dig_y + 5) == crate::chunk::Material::Stone
        }).count();
        
        log::info!("M5 demo completed: {} ticks", engine.tick_count());
        log::info!("Final character pos: ({:.1}, {:.1})", character.aabb.x, character.aabb.y);
        log::info!("Cells dug (sampled): {}/20", cells_dug);
        log::info!("Blocks placed (verified): {}/20", blocks_placed);
        log::info!("Character on ground: {}", character.on_ground);
        
        assert!(cells_dug > 5, "Should have dug some tunnel");
        assert!(blocks_placed > 5, "Should have placed some blocks");
        
        Ok(())
    }
}

/// Demo registry
pub struct DemoRegistry {
    demos: Vec<Box<dyn Demo>>,
}

impl DemoRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            demos: Vec::new(),
        };
        
        // Register demos
        registry.demos.push(Box::new(M0Demo));
        registry.demos.push(Box::new(M1Demo));
        registry.demos.push(Box::new(M2Demo));
        registry.demos.push(Box::new(M3Demo));
        registry.demos.push(Box::new(M4Demo));
        registry.demos.push(Box::new(M5Demo));
        
        registry
    }

    pub fn get(&self, id: &str) -> Option<&dyn Demo> {
        self.demos.iter()
            .find(|d| d.id() == id)
            .map(|b| b.as_ref())
    }

    pub fn list(&self) -> Vec<&dyn Demo> {
        self.demos.iter().map(|b| b.as_ref()).collect()
    }
}

/// Run a demo and generate report
pub fn run_demo(demo_id: &str, seed: u64, headless: bool) -> Result<DemoReport> {
    let registry = DemoRegistry::new();
    let demo = registry.get(demo_id)
        .ok_or_else(|| anyhow::anyhow!("Demo '{}' not found", demo_id))?;

    let config = EngineConfig {
        headless,
        seed,
        ..Default::default()
    };

    let mut engine = Engine::new(config);
    let start = Instant::now();
    
    demo.run(&mut engine)?;
    
    let report = DemoReport::success(
        demo_id.to_string(),
        seed,
        engine.tick_count(),
        start.elapsed(),
        engine.time.sim_time,
        engine.replay_hash(),
    );

    Ok(report)
}
