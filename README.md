# KerGameAIEngine

Rust-based game engine with custom architecture: **Terraria × Noita × Diablo**.

## 🔄 Repository Status: HARD RESET COMPLETE

This repository has undergone a complete hard reset. The previous C# MonoGame codebase has been replaced with a fresh Rust foundation. Old commit history has been orphaned from `master`.

## 🎯 Architecture

- **Custom Rust engine** with `wgpu` graphics (not Bevy-as-engine)
- **Fixed timestep**: simulation decoupled from rendering
- **Seeded RNG** for deterministic gameplay
- **Command pattern**: all actions serializable for replay/verification
- **Headless execution**: demos run without display for CI/testing
- **4px cellular automata**: pixel-based world (Noita-inspired) planned for M4+

## 🚀 Quick Start

```bash
# Build
cargo build --release

# Run the playable Terraria-like demo (headless bot mode)
cargo run --release --bin engine -- run-demo TERRARIA --headless --seed 42 --report terraria.json

# List available demos
cargo run --release --bin engine -- list-demos

# Run SHOWCASE (all features in ~30s)
cargo run --release --bin engine -- run-demo SHOWCASE --headless --seed 42 --report showcase.json
```

### Playable Terraria Demo

The **TERRARIA** demo is a vertical slice of a playable Terraria-like game that integrates all engine systems:

**Features:**
- 🌍 Generated overworld with terrain (dirt, stone, air)
- 🏃 Player movement (walk, jump) with physics
- ⛏️ Dig & place blocks → inventory system
- 👾 Enemy spawning, AI (chase), and combat
- 💡 Dynamic lighting system
- ❤️ Health, damage, and survival mechanics

**How to run:**
```bash
# Headless bot mode (automated 10s playthrough)
cargo run --release --bin engine -- run-demo TERRARIA --headless --seed 42 --report terraria.json

# Windowed mode (coming soon - will open interactive window)
# cargo run --release --bin engine -- run-demo TERRARIA --seed 42
```

**Goal:** Bot survives, digs blocks, collects items, fights enemies - proving all systems work together.

## 📊 Milestones

- **M0** ✓ Scaffold + CLI + headless report harness
- **M1** ✓ Time loop + commands + seed + replay hash
- **M2** ✓ ECS stress
- **M3** ✓ Render pipeline
- **M4** ✓ Pixel/chunk world
- **M5** ✓ Dig/build + motor
- **M6** ✓ Sand/fluid/reactions
- **M7** ✓ Lighting
- **M8** ✓ Items
- **M9** ✓ Combat/loot
- **M10** ✓ Magic combos
- **M11** ✓ NPCs
- **M12** ✓ Story/events
- **M13** ✓ Audio/UI
- **M14** ✓ Save/perf/ship
- **M15** ✓ 2D skeletal animation
- **TERRARIA** ✓ Playable vertical slice demo (all systems integrated)

See [`ROADMAP.md`](ROADMAP.md) for detailed milestone breakdown.

## 🤖 AI-First Development

This project is designed for autonomous AI development. See [`AGENTS.md`](AGENTS.md) for agent guidelines.

## 📄 License

MIT
