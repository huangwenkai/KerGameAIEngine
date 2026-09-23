# Windowed Rendering Implementation Status

## ✅ Completed
1. **Base branch**: Now on e3d4c22 which has full M0-M15 SHOWCASE (1860 ticks, 16 chapters)
2. **Shader confirmed**: `crates/engine/shaders/sprite.wgsl` exists and is functional
3. **Headless verified**: SHOWCASE runs successfully in headless mode with all M0-M15 chapters

## ❌ Still Needed
The branch e3d4c22 has NO windowed rendering code yet. Need to add:

1. `WindowedRenderContext` to `render.rs` (window + wgpu surface management)
2. `run_demo_windowed()` to `demo.rs` with **incremental pacing** (1 tick per frame, NOT batch)
3. `render_showcase_frame()` to `demo.rs` with M0-M15 chapter rendering
4. Refactor `run_demo()` to route to headless vs windowed

## Critical Fix Required
The windowed runner MUST advance incrementally:
```rust
// ✅ CORRECT (incremental):
Event::AboutToWait => {
    if engine.tick_count() < target_ticks {
        engine.tick()?; // One tick per frame
    }
}

// ❌ WRONG (batch execution):
Event::AboutToWait => {
    demo.run(&mut engine)?; // Runs ALL ticks at once!
}
```

## Next Steps
1. Add complete windowed code from earlier implementation
2. Test windowed mode (needs Windows machine with display)
3. Verify determinism still works
4. Push and mark PR ready
