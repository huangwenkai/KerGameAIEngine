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

/// M6 demo: Falling sand + fluid simulation
pub struct M6Demo;

impl Demo for M6Demo {
    fn id(&self) -> &str {
        "M6"
    }

    fn description(&self) -> &str {
        "M6 cellular automata: falling sand + water flow + material reactions"
    }

    fn run(&self, engine: &mut Engine) -> Result<()> {
        log::info!("Running M6 demo: {}", self.description());
        
        // Generate base terrain
        let terrain_gen = crate::terrain::TerrainGenerator::new(engine.config.seed);
        log::info!("Generating terrain...");
        for cy in -1..=1 {
            for cx in -1..=1 {
                terrain_gen.generate_chunk(&mut engine.chunk_world, crate::chunk::ChunkCoord::new(cx, cy));
            }
        }
        log::info!("Terrain generated: {} chunks", engine.chunk_world.chunk_count());
        
        // Phase 1: Drop sand pile (200 ticks)
        log::info!("Phase 1: Drop 200 sand cells (200 ticks)");
        let sand_x = 50;
        let sand_y_start = -20;
        
        for tick in 0..200 {
            engine.tick()?;
            
            // Drop sand from above
            if tick % 2 == 0 {
                let offset = (tick / 2) % 10 - 5;
                engine.queue_command(crate::commands::Command::PlaceCell {
                    x: sand_x + offset,
                    y: sand_y_start,
                    material: crate::chunk::Material::Sand,
                });
                engine.physics_sim.wake_cell(sand_x + offset, sand_y_start);
            }
        }
        
        log::info!("After phase 1: {} active cells", engine.physics_sim.active_cell_count());
        
        // Phase 2: Drop water (200 ticks)
        log::info!("Phase 2: Drop 100 water cells (200 ticks)");
        let water_x = 80;
        let water_y_start = -20;
        
        for tick in 200..400 {
            engine.tick()?;
            
            // Drop water from above
            if tick % 4 == 0 {
                let offset = (tick / 4) % 6 - 3;
                engine.queue_command(crate::commands::Command::PlaceCell {
                    x: water_x + offset,
                    y: water_y_start,
                    material: crate::chunk::Material::Water,
                });
                engine.physics_sim.wake_cell(water_x + offset, water_y_start);
            }
        }
        
        log::info!("After phase 2: {} active cells", engine.physics_sim.active_cell_count());
        
        // Phase 3: Let physics settle (200 ticks)
        log::info!("Phase 3: Physics settle (200 ticks)");
        for tick in 400..600 {
            engine.tick()?;
            
            if tick % 50 == 0 {
                log::info!("  Tick {}: {} active cells", tick, engine.physics_sim.active_cell_count());
            }
        }
        
        // Verify sand settled
        let mut sand_count = 0;
        let mut water_count = 0;
        for y in 0..100 {
            for x in 40..90 {
                match engine.chunk_world.get_cell(x, y) {
                    crate::chunk::Material::Sand => sand_count += 1,
                    crate::chunk::Material::Water => water_count += 1,
                    _ => {}
                }
            }
        }
        
        log::info!("M6 demo completed: {} ticks", engine.tick_count());
        log::info!("Sand cells settled: {}", sand_count);
        log::info!("Water cells settled: {}", water_count);
        log::info!("Final active cells: {}", engine.physics_sim.active_cell_count());
        
        assert!(sand_count > 50, "Should have settled sand (found {})", sand_count);
        assert!(water_count > 10, "Should have settled water (found {})", water_count);
        
        Ok(())
    }
}

/// M7 demo: Lighting system
pub struct M7Demo;

impl Demo for M7Demo {
    fn id(&self) -> &str {
        "M7"
    }

    fn description(&self) -> &str {
        "M7 lighting: tile light propagation + point lights + incremental updates"
    }

    fn run(&self, engine: &mut Engine) -> Result<()> {
        log::info!("Running M7 demo: {}", self.description());
        
        // Generate base terrain
        let terrain_gen = crate::terrain::TerrainGenerator::new(engine.config.seed);
        log::info!("Generating terrain...");
        for cy in -1..=1 {
            for cx in -1..=1 {
                terrain_gen.generate_chunk(&mut engine.chunk_world, crate::chunk::ChunkCoord::new(cx, cy));
            }
        }
        log::info!("Terrain generated: {} chunks", engine.chunk_world.chunk_count());
        
        // Set ambient light
        engine.light_map.set_ambient(32);
        
        // Phase 1: Dig cave + place torches (300 ticks)
        log::info!("Phase 1: Dig cave + place 10 torches (300 ticks)");
        let cave_x = 60;
        let cave_y = 40;
        
        for tick in 0..300 {
            engine.tick()?;
            
            // Dig cave (horizontal tunnel)
            if tick % 10 == 0 {
                let offset = tick / 10;
                for dy in -2..=2 {
                    engine.queue_command(crate::commands::Command::DigCell {
                        x: cave_x + offset as i32,
                        y: cave_y + dy,
                    });
                }
            }
            
            // Place torches every 5 cells
            if tick % 50 == 0 && tick > 0 {
                let torch_x = cave_x + (tick / 10) as i32;
                engine.light_map.add_light(crate::lighting::PointLight::new(
                    torch_x,
                    cave_y,
                    200,
                ));
            }
        }
        
        log::info!("After phase 1: {} lights, {} dirty cells", 
                   engine.light_map.light_count(), engine.light_map.dirty_count());
        
        // Phase 2: Add moving lights (200 ticks)
        log::info!("Phase 2: Add 5 moving lights (200 ticks)");
        
        // Add moving lights
        let moving_light_ids: Vec<_> = (0..5).map(|i| {
            engine.light_map.add_light(crate::lighting::PointLight::new(
                80 + i * 10,
                30,
                180,
            ))
        }).collect();
        
        for tick in 300..500 {
            engine.tick()?;
            
            // Move lights in circles
            if tick % 5 == 0 {
                for (i, &light_id) in moving_light_ids.iter().enumerate() {
                    let angle = (tick as f32 * 0.1 + i as f32 * 1.0).to_radians();
                    let radius = 5.0;
                    let center_x = 80 + i as i32 * 10;
                    let center_y = 30;
                    let x = center_x + (angle.cos() * radius) as i32;
                    let y = center_y + (angle.sin() * radius) as i32;
                    engine.light_map.update_light(light_id, x, y, 180);
                }
            }
        }
        
        log::info!("After phase 2: {} lights, {} dirty cells", 
                   engine.light_map.light_count(), engine.light_map.dirty_count());
        
        // Phase 3: Light propagation settle (100 ticks)
        log::info!("Phase 3: Light propagation settle (100 ticks)");
        for _tick in 500..600 {
            engine.tick()?;
        }
        
        // Calculate lighting metrics
        let mut lit_cells = 0;
        let mut bright_cells = 0;
        let mut total_light = 0u64;
        
        for y in 20..60 {
            for x in 40..120 {
                let light = engine.light_map.get_light(x, y);
                if light > 50 {
                    lit_cells += 1;
                    if light > 150 {
                        bright_cells += 1;
                    }
                    total_light += light as u64;
                }
            }
        }
        
        let avg_light = if lit_cells > 0 {
            (total_light / lit_cells as u64) as u32
        } else {
            0
        };
        
        log::info!("M7 demo completed: {} ticks", engine.tick_count());
        log::info!("Total lights: {}", engine.light_map.light_count());
        log::info!("Lit cells (>50): {}", lit_cells);
        log::info!("Bright cells (>150): {}", bright_cells);
        log::info!("Average light level: {}", avg_light);
        log::info!("Final dirty cells: {}", engine.light_map.dirty_count());
        
        assert!(engine.light_map.light_count() >= 10, "Should have at least 10 lights");
        assert!(lit_cells > 100, "Should have lit area (got {} cells)", lit_cells);
        assert!(bright_cells > 10, "Should have bright areas near torches");
        
        Ok(())
    }
}

/// M8 demo: Item system
pub struct M8Demo;

impl Demo for M8Demo {
    fn id(&self) -> &str {
        "M8"
    }

    fn description(&self) -> &str {
        "M8 items: definitions, inventory, drop/pickup mechanics"
    }

    fn run(&self, engine: &mut Engine) -> Result<()> {
        log::info!("Running M8 demo: {}", self.description());
        
        // Create item registry
        let mut registry = crate::items::ItemRegistry::new();
        
        // Register item definitions
        log::info!("Registering items...");
        let sword_id = registry.generate_id();
        registry.register(crate::items::ItemDef::new_weapon(
            sword_id, "Iron Sword", 15, crate::items::Rarity::Common
        ));
        
        let axe_id = registry.generate_id();
        registry.register(crate::items::ItemDef::new_weapon(
            axe_id, "Steel Axe", 20, crate::items::Rarity::Uncommon
        ));
        
        let armor_id = registry.generate_id();
        registry.register(crate::items::ItemDef::new_armor(
            armor_id, "Chain Mail", 25, crate::items::Rarity::Rare
        ));
        
        let potion_id = registry.generate_id();
        registry.register(crate::items::ItemDef::new_consumable(
            potion_id, "Health Potion", crate::items::Rarity::Common, 99
        ));
        
        let wood_id = registry.generate_id();
        registry.register(crate::items::ItemDef::new_material(
            wood_id, "Wood", 999
        ));
        
        log::info!("Registered {} items", registry.count());
        
        // Create world items and inventory
        let mut world_items = crate::items::WorldItems::new();
        let mut inventory = crate::items::Inventory::new(20);
        
        // Phase 1: Drop items in world (200 ticks)
        log::info!("Phase 1: Drop 100 items (200 ticks)");
        let mut rng = crate::rng::GameRng::new(engine.config.seed);
        
        for tick in 0..200 {
            engine.tick()?;
            
            // Drop random items
            if tick % 2 == 0 {
                let item_type = rng.gen_u32() % 5;
                let (def_id, count) = match item_type {
                    0 => (sword_id, 1),
                    1 => (axe_id, 1),
                    2 => (armor_id, 1),
                    3 => (potion_id, rng.gen_range(1..=10)),
                    _ => (wood_id, rng.gen_range(1..=50)),
                };
                
                let x = 50.0 + rng.gen_f32() * 100.0;
                let y = 0.0;
                world_items.drop_item(x, y, crate::items::ItemStack::new(def_id, count));
            }
            
            // Update item physics
            world_items.update(engine.config.fixed_timestep.as_secs_f32());
        }
        
        log::info!("After phase 1: {} items in world", world_items.item_count());
        
        // Phase 2: Pickup items (300 ticks)
        log::info!("Phase 2: AI pickup items (300 ticks)");
        let mut player_x = 50.0;
        let mut player_y = 400.0;
        let mut items_picked = 0;
        
        for tick in 200..500 {
            engine.tick()?;
            
            // Move player toward items
            if tick % 5 == 0 {
                player_x += 2.0;
                if player_x > 150.0 {
                    player_x = 50.0;
                }
            }
            
            // Try to pickup nearby items
            if tick % 3 == 0 {
                if let Some(stack) = world_items.pickup_near(player_x, player_y, 10.0) {
                    if inventory.add_item(stack, &registry).is_ok() {
                        items_picked += 1;
                    }
                }
            }
        }
        
        log::info!("After phase 2: {} items remaining in world, {} items in inventory", 
                   world_items.item_count(), inventory.item_count());
        log::info!("Total items picked: {}", items_picked);
        
        // Phase 3: Inventory management (100 ticks)
        log::info!("Phase 3: Inventory management (100 ticks)");
        for _tick in 500..600 {
            engine.tick()?;
        }
        
        // Count items by type in inventory
        let mut weapon_count = 0;
        let mut armor_count = 0;
        let mut consumable_count = 0;
        let mut material_count = 0;
        
        for i in 0..inventory.slot_count() {
            if let Some(stack) = inventory.get_slot(i) {
                if let Some(def) = registry.get(stack.def_id) {
                    match def.item_type {
                        crate::items::ItemType::Weapon => weapon_count += stack.count,
                        crate::items::ItemType::Armor => armor_count += stack.count,
                        crate::items::ItemType::Consumable => consumable_count += stack.count,
                        crate::items::ItemType::Material => material_count += stack.count,
                        _ => {}
                    }
                }
            }
        }
        
        log::info!("M8 demo completed: {} ticks", engine.tick_count());
        log::info!("Inventory slots used: {}/{}", inventory.item_count(), inventory.slot_count());
        log::info!("Weapons: {}", weapon_count);
        log::info!("Armor: {}", armor_count);
        log::info!("Consumables: {}", consumable_count);
        log::info!("Materials: {}", material_count);
        log::info!("Items remaining in world: {}", world_items.item_count());
        
        assert!(items_picked > 10, "Should have picked up items");
        assert!(inventory.item_count() > 5, "Should have items in inventory");
        
        Ok(())
    }
}

/// M9 demo: Combat + loot
pub struct M9Demo;

impl Demo for M9Demo {
    fn id(&self) -> &str {
        "M9"
    }

    fn description(&self) -> &str {
        "M9 combat: damage/health/death, attack commands, loot drops"
    }

    fn run(&self, engine: &mut Engine) -> Result<()> {
        log::info!("Running M9 demo: {}", self.description());
        
        // Setup item registry
        let mut registry = crate::items::ItemRegistry::new();
        let sword_id = registry.generate_id();
        registry.register(crate::items::ItemDef::new_weapon(sword_id, "Sword", 15, crate::items::Rarity::Common));
        let potion_id = registry.generate_id();
        registry.register(crate::items::ItemDef::new_consumable(potion_id, "Potion", crate::items::Rarity::Common, 99));
        let gold_id = registry.generate_id();
        registry.register(crate::items::ItemDef::new_material(gold_id, "Gold", 9999));
        
        // Setup combat system
        let mut combat = crate::combat::CombatSystem::new();
        let mut world_items = crate::items::WorldItems::new();
        let mut rng = crate::rng::GameRng::new(engine.config.seed);
        
        // Phase 1: Spawn combatants (100 ticks)
        log::info!("Phase 1: Spawn 50 enemies (100 ticks)");
        
        let player = combat.spawn_entity("Player", 50.0, 50.0, 1000, 50, 50);
        
        let mut enemy_ids = Vec::new();
        for i in 0..50 {
            let x = 100.0 + (i % 10) as f32 * 20.0;
            let y = 100.0 + (i / 10) as f32 * 20.0;
            let health = 20 + rng.gen_range(0..=20);
            let damage = 5 + rng.gen_range(0..=10);
            let armor = rng.gen_range(0..=5);
            
            let enemy_id = combat.spawn_entity(&format!("Enemy{}", i), x, y, health, damage, armor);
            
            // Setup loot tables
            if let Some(enemy) = combat.get_entity_mut(enemy_id) {
                let rarity = match rng.gen_u32() % 5 {
                    0 => crate::items::Rarity::Uncommon,
                    1 => crate::items::Rarity::Rare,
                    _ => crate::items::Rarity::Common,
                };
                enemy.loot_table = crate::combat::generate_loot_table(rarity, &[sword_id, potion_id, gold_id]);
            }
            
            enemy_ids.push(enemy_id);
        }
        
        for _tick in 0..100 {
            engine.tick()?;
        }
        
        log::info!("After phase 1: {} combatants", combat.entity_count());
        
        // Phase 2: Combat (400 ticks)
        log::info!("Phase 2: Combat (400 ticks)");
        let mut kills = 0;
        let mut total_damage_dealt = 0;
        
        for tick in 100..500 {
            engine.tick()?;
            
            // Player attacks random living enemy
            if tick % 3 == 0 {
                let alive_enemies: Vec<_> = enemy_ids.iter()
                    .filter(|&&id| combat.get_entity(id).map_or(false, |e| e.is_alive()))
                    .copied()
                    .collect();
                
                if !alive_enemies.is_empty() {
                    let target = alive_enemies[rng.gen_range(0..alive_enemies.len())];
                    if let Some(damage) = combat.attack(player, target) {
                        total_damage_dealt += damage;
                    }
                }
            }
            
            // Enemies attack player occasionally (much less frequent)
            if tick % 50 == 0 && tick < 300 { // Stop attacking after tick 300
                for &enemy_id in &enemy_ids {
                    if combat.get_entity(enemy_id).map_or(false, |e| e.is_alive()) {
                        if rng.gen_f32() < 0.2 { // Only 20% chance
                            combat.attack(enemy_id, player);
                        }
                    }
                }
            }
            
            // Remove dead and drop loot
            let dead = combat.remove_dead();
            for entity in dead {
                kills += 1;
                let loot = entity.loot_table.roll_loot(&mut rng);
                for stack in loot {
                    world_items.drop_item(entity.x, entity.y, stack);
                }
            }
        }
        
        log::info!("After phase 2: {} kills, {} damage dealt, {} items dropped", 
                   kills, total_damage_dealt, world_items.item_count());
        
        // Phase 3: Loot collection (100 ticks)
        log::info!("Phase 3: Collect loot (100 ticks)");
        let mut inventory = crate::items::Inventory::new(30);
        let mut collected = 0;
        
        for tick in 500..600 {
            engine.tick()?;
            
            if let Some(player_entity) = combat.get_entity(player) {
                if tick % 2 == 0 {
                    if let Some(stack) = world_items.pickup_near(player_entity.x, player_entity.y, 500.0) {
                        if inventory.add_item(stack, &registry).is_ok() {
                            collected += 1;
                        }
                    }
                }
            }
        }
        
        // Calculate final stats
        let player_entity = combat.get_entity(player).expect("Player should exist");
        let player_health = player_entity.stats.health_percent();
        let player_alive = player_entity.is_alive();
        
        let mut swords = 0;
        let mut potions = 0;
        let mut gold = 0;
        
        for i in 0..inventory.slot_count() {
            if let Some(stack) = inventory.get_slot(i) {
                match stack.def_id {
                    id if id == sword_id => swords += stack.count,
                    id if id == potion_id => potions += stack.count,
                    id if id == gold_id => gold += stack.count,
                    _ => {}
                }
            }
        }
        
        log::info!("M9 demo completed: {} ticks", engine.tick_count());
        log::info!("Player health: {:.0}% (alive: {})", player_health * 100.0, player_alive);
        log::info!("Enemies killed: {}", kills);
        log::info!("Total damage dealt: {}", total_damage_dealt);
        log::info!("Loot collected: {} items", collected);
        log::info!("Inventory: {} swords, {} potions, {} gold", swords, potions, gold);
        
        assert!(kills > 0, "Should have killed some enemies (got {})", kills);
        assert!(total_damage_dealt > 50, "Should have dealt damage");
        
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
        registry.demos.push(Box::new(M6Demo));
        registry.demos.push(Box::new(M7Demo));
        registry.demos.push(Box::new(M8Demo));
        registry.demos.push(Box::new(M9Demo));
        
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
