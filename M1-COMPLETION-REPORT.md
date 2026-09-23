# M1 Milestone Completion Report
## Command Execution & Replay Determinism

**Date**: 2026-09-23  
**Status**: ✓ COMPLETE

---

## Implementation Summary

### M1 Deliverables
✓ Fixed-timestep simulation loop (owned by engine-core)  
✓ Serializable Command queue + execution path  
✓ Seeded RNG streams (`ChaCha8Rng`)  
✓ Record/replay: same seed + same commands → identical hash  
✓ Headless demo M1 (631 ticks, ~10.5s sim time)  
✓ Unit tests covering replay determinism (13/13 passing)  
✓ ROADMAP.md updated  

### New Modules

#### `rng.rs` - Seeded RNG
- `GameRng` wrapper around `ChaCha8Rng`
- Deterministic random generation
- Tests: same seed → same sequence, different seeds → different sequences

#### `state.rs` - World State
- `Entity` struct: position (x, y) + velocity (vx, vy)
- `World`: entity storage + spawning + physics update
- `state_hash()`: SHA256 hash of all entity state for verification
- Tests: spawning, movement, deterministic hashing

#### `commands.rs` - Expanded Command Enum
- `SpawnEntity { x, y }`: Create entity at position
- `SetVelocity { entity_id, vx, vy }`: Set entity velocity
- `ApplyImpulse { entity_id, dx, dy }`: Add to velocity
- `apply()`: Execute command on world
- Tests: command execution correctness

#### `replay_tests.rs` - Determinism Verification
- `replay_determinism_same_seed`: Same seed → identical hash ✓
- `replay_determinism_different_seed`: Different seeds → different hashes ✓
- `command_execution_order`: Command execution + physics integration ✓

---

## Test Results

```bash
cargo test --release
```

**Result**: 13/13 tests PASSED

### Test Breakdown
- `commands` module: 2 tests (spawn, velocity)
- `rng` module: 2 tests (determinism, variation)
- `state` module: 3 tests (spawn, movement, hash)
- `replay_tests` module: 3 tests (replay determinism)
- `lib` module: 3 tests (engine basics, deterministic sim)

---

## M1 Demo Results

### Demo Design
- **Phase 1**: Spawn 100 entities at random positions
- **Phase 2**: Apply random velocities (30 ticks with commands)
- **Phase 3**: Simulate movement (600 ticks)
- **Total**: 631 ticks (~10.5 seconds @ 60 TPS)

### Run 1: Seed 42
```json
{
  "demo_id": "M1",
  "seed": 42,
  "success": true,
  "tick_count": 631,
  "elapsed_real_ms": 0,
  "elapsed_sim_ms": 10516,
  "replay_hash": "96f83e4ef583bdfee20c93a82359217c6e085480ef88c51169a1350b4594995a:261babe18f51bbd3428a17ec0b4b3a042ce8378991f44f2a3b5fadf0eac2237b",
  "message": "Demo completed successfully"
}
```

### Run 2: Seed 42 (Determinism Check)
```
replay_hash: 96f83e4ef583bdfee20c93a82359217c6e085480ef88c51169a1350b4594995a:261babe18f51bbd3428a17ec0b4b3a042ce8378991f44f2a3b5fadf0eac2237b
```
**✓ IDENTICAL** - Determinism verified!

### Run 3: Seed 999 (Variation Check)
```
replay_hash: 671206303b87899c260db2fc4f0fdb510d18fceedf32c85964e748a7573a3e02:03c1c5dff90daa54d015f3b6f60c63e8ef4b152f0093426a42c8519b9fad138d
```
**✓ DIFFERENT** - Seed variation confirmed!

---

## How to Run

### List demos
```bash
cargo run --release --bin engine -- list-demos
```

Output:
```
Available demos:
  M0 - M0 scaffold validation: CLI + headless harness + report generation
  M1 - M1 command replay: 1000 commands with deterministic execution
```

### Run M1 demo
```bash
cargo run --release --bin engine -- run-demo M1 --headless --seed 42 --report m1-report.json
```

### Verify determinism
```bash
# Run twice with same seed
engine run-demo M1 --headless --seed 42 --report run1.json
engine run-demo M1 --headless --seed 42 --report run2.json

# Compare (should be identical)
diff <(jq .replay_hash run1.json) <(jq .replay_hash run2.json)
```

---

## Architecture Details

### Replay Hash Format
```
<command_hash>:<state_hash>
```

- **Command hash**: SHA256 of all serialized commands executed
- **State hash**: SHA256 of final world state (all entity positions/velocities)
- Both must match for true determinism

### Fixed Timestep Loop
```rust
pub fn tick(&mut self) -> Result<()> {
    self.time.tick(self.config.fixed_timestep);
    
    // 1. Execute commands
    for cmd in self.command_buffer.drain() {
        self.replay_hasher.hash_command(&cmd);
        cmd.apply(&mut self.world);
    }
    
    // 2. Update physics
    let dt = self.config.fixed_timestep.as_secs_f32();
    self.world.update_all(dt);
    
    Ok(())
}
```

### Command Flow
```
User/Demo → queue_command()
    ↓
CommandBuffer (per tick)
    ↓
tick() → drain commands
    ↓
hash_command() + cmd.apply(world)
    ↓
Physics update (world.update_all)
    ↓
State hash for verification
```

---

## Performance Metrics

- **Tick rate**: 60 TPS (16.67ms budget)
- **M1 actual**: < 1ms per tick
- **631 ticks**: 0ms real time (below measurement threshold)
- **Entity count**: 100 entities with physics
- **Commands processed**: ~3100 commands total

---

## Key Learnings

### Determinism Requirements Met
1. ✓ Seeded RNG (ChaCha8 with explicit seed)
2. ✓ Fixed timestep (no wall-clock dependencies)
3. ✓ Command serialization (all actions recorded)
4. ✓ Deterministic physics (f32 operations with fixed dt)
5. ✓ State hashing (full world state verification)

### Hash Stability
- Same seed + same commands → **100% identical hash**
- Different seeds → **100% different hashes**
- Replay system validated across multiple runs

---

## Next Milestone: M2

**Goal**: ECS stress test with 100k entities

**Key additions**:
- Proper ECS architecture (likely `hecs` or custom)
- Component types: Transform, Velocity, Health
- System execution loop
- M2 Demo: 100k entities, 600 ticks, performance benchmark

**Estimated complexity**: Moderate (need ECS library integration)

---

## Changes from M0 → M1

### New Files
- `crates/engine/src/rng.rs` (58 lines)
- `crates/engine/src/state.rs` (110 lines)
- `crates/engine/src/replay_tests.rs` (120 lines)

### Modified Files
- `crates/engine/src/lib.rs`: Added RNG + World, expanded Engine
- `crates/engine/src/commands.rs`: Real commands (SpawnEntity, SetVelocity, ApplyImpulse)
- `crates/engine/src/demo.rs`: Added M1Demo
- `ROADMAP.md`: Marked M1 complete

### Lines of Code
- M0: ~500 LOC
- M1: ~800 LOC (+60%)
- All tests passing

---

## Final Checklist

- [x] Fixed timestep simulation loop
- [x] Seeded RNG (deterministic)
- [x] Command queue + apply
- [x] Replay hash (command + state)
- [x] M1 demo runs headless (5-15s range: 10.5s ✓)
- [x] JSON report generation
- [x] Unit tests for replay determinism
- [x] Same seed → same hash verified
- [x] Different seed → different hash verified
- [x] ROADMAP.md updated
- [x] Self-tested (no human playtest)

---

## Conclusion

M1 milestone **COMPLETE**. The engine now has a fully deterministic command-replay architecture with verified hash consistency. Same seed + same commands produce identical world state, validated by both unit tests and demo runs.

**Repository**: https://github.com/huangwenkai/KerGameAIEngine  
**Branch**: `master`  
**Status**: Ready for M2 (ECS stress test)
