# SHOWCASE Demo Guide

## Overview

**SHOWCASE** is the primary **experience and review entrypoint** for KerGameAIEngine. It provides a unified, curated demonstration of all engine features in a single run, designed for human evaluation and understanding.

Unlike individual milestone demos (M0-M8) which focus on isolated feature validation and automated testing, SHOWCASE presents a cohesive journey through the engine's capabilities with emphasis on breadth and flow.

## Purpose

- **Human Review**: Quickly experience all engine systems in ~30-40 seconds
- **Feature Coverage**: Every implemented feature gets a dedicated chapter
- **Integration Testing**: Verify all systems work together harmoniously
- **Documentation**: Living showcase of current engine state

## Running SHOWCASE

### Headless Mode (Automated Testing)

```bash
cargo run --bin engine -- run-demo SHOWCASE --headless --seed 42 --report showcase.json
```

This mode is for CI/automated verification. Runs deterministically with fixed seed.

### Windowed Mode (Human Experience)

```bash
cargo run --bin engine -- run-demo SHOWCASE --seed 42
```

*Note: Windowed mode requires GPU and windowing system. Currently, most testing is done in headless mode. Visual output features use mock rendering where GPU is unavailable.*

## Current Chapters

### Chapter 0: Tick Loop + Commands (M0, M1)
**Duration**: 120 ticks (~2 seconds)

Demonstrates:
- Fixed timestep simulation (60 TPS)
- Command buffer pattern
- Replay hash system
- Deterministic execution

### Chapter 1: ECS Stress Test (M2)
**Duration**: 180 ticks (~3 seconds)

Demonstrates:
- Entity-Component-System architecture
- Spawning 1,000 entities with Transform, Velocity, Health components
- Movement and collision systems
- Performance under load

### Chapter 2: Render Pipeline (M3)
**Duration**: 180 ticks (~3 seconds)

Demonstrates:
- wgpu render context (headless capable)
- Sprite batching and instancing
- Camera system (800×600)
- Headless texture capture

### Chapter 3: Chunks + Terrain (M4)
**Duration**: 180 ticks (~3 seconds)

Demonstrates:
- **4-pixel cell system** (locked design decision)
- Chunk-based world (128×128 cells = 512×512 screen pixels)
- Sparse chunk storage
- Seeded Perlin terrain generation
- get_cell/set_cell APIs

### Chapter 4: Dig/Build (M5)
**Duration**: 180 ticks (~3 seconds)

Demonstrates:
- Digging tunnels through terrain
- Placing blocks (stone construction)
- Material modification via commands
- Terrain destruction and creation

### Chapter 5: Character Motor (M5)
**Duration**: 180 ticks (~3 seconds)

Demonstrates:
- AABB collision detection
- Character physics (walk, jump, friction)
- Terrain interaction
- Ground detection

### Chapter 6: Sand/Fluid Physics (M6)
**Duration**: 300 ticks (~5 seconds)

Demonstrates:
- Cellular automata simulation
- Falling sand physics
- Water flow and spreading
- Material interactions
- Active cell optimization

### Chapter 7: Lighting System (M7)
**Duration**: 300 ticks (~5 seconds)

Demonstrates:
- Tile-based light propagation
- Dynamic point lights (5 moving lights)
- Ambient lighting
- Incremental light updates
- Light occlusion

### Chapter 8: Items/Inventory (M8)
**Duration**: 300 ticks (~5 seconds)

Demonstrates:
- Item definitions and rarity system
- World item drops with physics
- Inventory management (20-slot)
- Pickup mechanics
- Item registry

## Total Duration

**~1,920 ticks (~32 seconds of simulation)**

Average performance target: <1ms per tick for smooth 60 FPS capability.

## Output

SHOWCASE generates:
- Console log with formatted chapter progress
- Final summary with per-chapter metrics
- JSON report (when `--report` flag used) with:
  - Success status
  - Total tick count
  - Real elapsed time
  - Simulated time
  - Replay hash (for determinism verification)

Example output:
```
╔══════════════════════════════════════════════════════════╗
║         KerGameAIEngine SHOWCASE                         ║
║   Fantasy Pixel Engine (Terraria × Noita × Diablo)      ║
╚══════════════════════════════════════════════════════════╝

┌─ Chapter 0: Tick Loop + Commands (M0, M1) ───────────────┐
│ ✓ Fixed timestep: 120 ticks @ 60 TPS
│ ✓ Command buffer + replay hasher
└─ Completed in 15ms

... (more chapters) ...

╔══════════════════════════════════════════════════════════╗
║                   SHOWCASE SUMMARY                       ║
╚══════════════════════════════════════════════════════════╝
  Chapter 0: Tick/Commands - 120 ticks in 15ms
  Chapter 1: ECS - 180 ticks in 22ms
  ...
  
  Total: 1920 ticks in 350ms
  Average: 0.18ms per tick
  Replay hash: abc123...

✓ All features showcased successfully!
```

## Future Chapters

As new milestones are completed, they **MUST** be added to SHOWCASE. Planned additions:

- **Chapter 9: Combat/Loot** (M9) - Diablo-style combat, enemy AI, loot drops
- **Chapter 10: Magic System** (M10) - Noita-style spell combinations
- **Chapter 11: NPC/Dialogue** (M11) - Quest system, NPC interactions
- **Chapter 12: Story/Events** (M12) - Dynamic narrative, boss battles
- **Chapter 13: Audio/UI** (M13) - Sound effects, HUD, menus
- **Chapter 14: Save/Load** (M14) - World persistence, state serialization
- **Chapter 15: Skeletal Animation** (M15) - Character animations, events

## Development Rule

> **When a new feature/milestone ships, it MUST also get a chapter in SHOWCASE.**

See `AGENTS.md` for Agent development guidelines regarding SHOWCASE updates.

## Verification

To verify SHOWCASE determinism:

```bash
# Run twice with same seed
cargo run --bin engine -- run-demo SHOWCASE --headless --seed 42 --report run1.json
cargo run --bin engine -- run-demo SHOWCASE --headless --seed 42 --report run2.json

# Compare hashes
diff <(jq .replay_hash run1.json) <(jq .replay_hash run2.json)
# Should output nothing (hashes match)
```

## Integration with CI

SHOWCASE should be run in CI pipelines to ensure:
1. All features continue to work together
2. Performance remains acceptable
3. Determinism is maintained
4. No regressions across systems

Recommended CI check:
```bash
cargo test
cargo build --release
cargo run --bin engine --release -- run-demo SHOWCASE --headless --seed 42 --report showcase.json
# Assert showcase.json contains "success": true
```

## Comparison to Milestone Demos

| Aspect | SHOWCASE | Milestone Demos (M0-M8) |
|--------|----------|-------------------------|
| Purpose | Human experience/review | Automated feature gates |
| Duration | ~30-40s total | 5-15s each |
| Scope | All features | Single milestone |
| Audience | Humans + CI | Primarily automated testing |
| Updates | Every new feature | When milestone changes |
| Priority | Primary entrypoint | Supporting validation |

Both demo types are important:
- **SHOWCASE**: "Does the engine feel complete and cohesive?"
- **Milestone demos**: "Does feature X work correctly in isolation?"

---

**Remember**: SHOWCASE is for the human. Make it informative, fast, and comprehensive. Individual M* demos remain for automated gates and detailed validation.
