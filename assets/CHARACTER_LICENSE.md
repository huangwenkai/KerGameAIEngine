# Character Assets License

## Current Implementation
**Procedural colored rectangles** - No external assets required  
License: Public Domain (generated code)

The current character is rendered using procedural geometry (colored rectangles for limbs) attached to the M15 skeletal animation system. This allows the engine to work immediately without external dependencies.

## Documented CC0 Sources for Future Integration

### Option 1: Adventurer and Slime (Recommended)
- **Author**: Segel
- **Source**: https://opengameart.org/content/adventurer-and-slime-game-sprites
- **License**: CC0 1.0 Universal (Public Domain)
- **Date**: June 13, 2023
- **Animations**: Idle, Run, Attack, Jump/Fall, Air Attack, Hurt, Dead
- **Format**: PNG sprite sheets
- **Notes**: Most complete set, suitable for side-scrolling/platformer

### Option 2: Ninja [Animated]
- **Author**: morgan3d and DezrasDragons
- **Source**: https://opengameart.org/content/ninja-animated-0
- **License**: CC0 1.0 Universal (Public Domain)
- **Date**: June 1, 2019
- **Size**: 32×32 pixels
- **Animations**: Idle, Run, Jump, Attack, Swim, Climb/Crawl
- **Weapons**: Sword and Bow variants included
- **Notes**: Excellent climb animation, multiple weapon options

### Option 3: Pixel Character 01 - Onur
- **Author**: ImogiaGames (derivative of Frozenen's work)
- **Source**: https://opengameart.org/content/pixel-character-01-onur
- **License**: CC0 1.0 Universal (Public Domain)
- **Date**: December 3, 2021
- **Animations**: Idle, Walk (4-dir), Jump, Fall, Climb, Attack, Hit, Death
- **Format**: Sprite sheet (h16 v8)
- **Notes**: Directional animations, full animation set

### Option 4: RPG Asset Character 'Soldier' SMS
- **Author**: Chasersgaming
- **Source**: https://opengameart.org/content/rpg-asset-character-soldier-sms
- **License**: CC0 1.0 Universal (Public Domain)
- **Date**: March 28, 2020
- **Animations**: Idle, Walk, Run, Climb, Push/Pull, Punch, Kick, Hit, Jump
- **Directions**: 4-way (N, E, S, W)
- **Format**: Master System palette, sprite sheets + individual strips
- **Notes**: Retro style, extensive animation set

### Option 5: Trainee Knight
- **Author**: Segel
- **Source**: https://opengameart.org/content/trainee-knight-character
- **License**: CC0 1.0 Universal (Public Domain)
- **Date**: March 23, 2022
- **Animations**: Idle, Run, Attack, Jump Attack, Jump, Fall, Hurt, Dead
- **Notes**: Chibi style, suitable for platformers

### Option 6: Kenney Platformer Characters 1
- **Author**: Kenney
- **Source**: https://opengameart.org/content/platformer-characters-1-5-characters
- **License**: CC0 1.0 Universal (Public Domain)
- **Date**: January 10, 2017
- **Characters**: 5 characters × 24 poses
- **Format**: PNG + vector source + separated limbs for Spine/Spriter
- **Notes**: Modular limbs perfect for skeletal animation

## Integration Instructions

To replace procedural character with sprite assets:

1. Download one of the above packs
2. Place sprite sheets in `assets/character/`
3. Update `CharacterAnimator` in `crates/engine/src/animation.rs` to:
   - Load sprite texture
   - Map bone transforms to sprite positions/rotations
   - Render sprites instead of colored quads
4. Adjust bone lengths/positions to match sprite proportions

## License Requirements

All listed sources are CC0 1.0 Universal:
- ✓ Commercial use allowed
- ✓ Attribution not required (but appreciated)
- ✓ No permission needed
- ✓ Can modify freely

Credit format (optional): "Character sprites by [Author] (CC0)" or "www.kenney.nl"
