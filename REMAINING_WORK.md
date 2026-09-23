# Remaining Work for Windowed SHOWCASE

## Status
✅ Rebased onto e3d4c22 (full M0-M15 SHOWCASE)
✅ Added WindowedRenderContext to render.rs
✅ Shader confirmed to exist
✅ Headless SHOWCASE works (1860 ticks, M0-M15)

## Still Missing in demo.rs

Need to add 3 functions:

### 1. run_demo_windowed(demo_id: &str, seed: u64) -> Result<DemoReport>
- Create winit event loop
- Initialize WindowedRenderContext
- Create render pipeline with shader
- **CRITICAL**: Event::AboutToWait handler must call `engine.tick()` incrementally, NOT `demo.run()`
- Initialize SHOWCASE state (terrain, entities, lights)
- Target: 1860 ticks for SHOWCASE (not 810)
- Add chapter-specific actions at tick boundaries (sand/water drops, etc.)

### 2. render_showcase_frame(engine: &mut Engine, batch: &mut SpriteBatch)
- Calculate chapter from tick_count (variable chapter lengths!)
- Render content for chapters 0-15 (M0-M15)
  - 0: M0 - Pulse animation
  - 1: M1 - Spawned entities
  - 2: M2 - ECS entities
  - 3: M3 - Colorful patterns
  - 4: M4 - Terrain visualization
  - 5: M5 - Character + tunnel
  - 6: M6 - Falling sand/water
  - 7: M7 - Light sources + glow
  - 8: M8 - Items with bob animation
  - 9: M9 - Combat (player vs enemies)
  - 10: M10 - Magic circles + projectiles
  - 11: M11 - NPCs (Merchant, Guard)
  - 12: M12 - Quest log scroll
  - 13: M13 - Sound waves visualization
  - 14: M14 - Save disk icon
  - 15: M15 - Animated skeleton
- Progress bar: total 1860 ticks

### 3. Refactor run_demo()
```rust
pub fn run_demo(demo_id: &str, seed: u64, headless: bool) -> Result<DemoReport> {
    if headless {
        run_demo_headless(demo_id, seed)
    } else {
        run_demo_windowed(demo_id, seed)
    }
}
```

## Reference Implementation
See commit eb7c205 for the windowed code structure, but with these CRITICAL FIXES:
1. Target ticks: 1860 (not 810)
2. Pacing: `engine.tick()` per frame (NOT `demo.run()`)
3. Rendering: All 16 chapters (M0-M15)

## Testing
After implementation:
```bash
# Headless (already works)
cargo run --release --bin engine -- run-demo SHOWCASE --headless --seed 42

# Windowed (needs Windows + display)
cargo run --release --bin engine -- run-demo SHOWCASE --seed 42
```
