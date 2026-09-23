# Skeleton Animation Format

This document describes the JSON schema for 2D skeletal animation data in KerGameAIEngine.

## Overview

The animation system supports:
- Hierarchical bone structures with parent-child relationships
- Local and world-space transforms (position, rotation, scale)
- Sprite attachments (slots) bound to bones
- Keyframe animation with linear interpolation
- Event tracks for gameplay hooks (combat hits, cancellable windows, etc.)
- Looping and one-shot animations

## Transform2D

2D transform with translation, rotation, and scale.

```json
{
  "x": 0.0,
  "y": 0.0,
  "rotation": 0.0,
  "scale_x": 1.0,
  "scale_y": 1.0
}
```

- `x`, `y`: Position in 2D space (pixels)
- `rotation`: Rotation in radians (0 = no rotation)
- `scale_x`, `scale_y`: Scale factors (1.0 = no scale)

## Bone

A bone in the skeleton hierarchy.

```json
{
  "name": "arm_left",
  "parent_index": 1,
  "local_transform": {
    "x": -8.0,
    "y": 0.0,
    "rotation": 0.0,
    "scale_x": 1.0,
    "scale_y": 1.0
  }
}
```

- `name`: Unique bone identifier
- `parent_index`: Index of parent bone (null for root)
- `local_transform`: Transform relative to parent (or world if root)

## Attachment (Slot)

A sprite quad attached to a bone.

```json
{
  "name": "body_sprite",
  "bone_index": 1,
  "offset": {
    "x": 0.0,
    "y": 0.0,
    "rotation": 0.0,
    "scale_x": 1.0,
    "scale_y": 1.0
  },
  "width": 12.0,
  "height": 20.0,
  "color": [0.8, 0.2, 0.2, 1.0]
}
```

- `name`: Attachment identifier
- `bone_index`: Index of bone this attaches to
- `offset`: Local transform relative to bone
- `width`, `height`: Sprite dimensions (pixels)
- `color`: RGBA color [r, g, b, a] (0.0-1.0 range)

## Keyframe

A single keyframe in an animation track.

```json
{
  "time": 0.5,
  "transform": {
    "x": 10.0,
    "y": 5.0,
    "rotation": 0.785,
    "scale_x": 1.2,
    "scale_y": 1.0
  }
}
```

- `time`: Time in seconds
- `transform`: Bone transform at this time

## BoneTrack

Animation track for a single bone.

```json
{
  "bone_index": 2,
  "keyframes": [
    {
      "time": 0.0,
      "transform": { "x": 0.0, "y": 0.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 }
    },
    {
      "time": 0.5,
      "transform": { "x": 5.0, "y": 2.0, "rotation": 0.785, "scale_x": 1.0, "scale_y": 1.0 }
    },
    {
      "time": 1.0,
      "transform": { "x": 0.0, "y": 0.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 }
    }
  ]
}
```

- `bone_index`: Index of bone this track animates
- `keyframes`: Array of keyframes (sorted by time)

Sampling behavior:
- Before first keyframe: use first keyframe value
- Between keyframes: linear interpolation
- After last keyframe: use last keyframe value

## AnimationEvent

A named event that fires at a specific time.

```json
{
  "time": 0.5,
  "name": "hit"
}
```

- `time`: Time in seconds when event fires
- `name`: Event identifier (e.g., "hit", "can_cancel", "footstep")

Common use cases:
- `"hit"`: Combat damage application point
- `"can_cancel"`: Window where action can be interrupted
- `"footstep"`: Audio cue trigger
- `"spawn_vfx"`: Visual effect spawn point

## AnimationClip

A complete animation clip.

```json
{
  "name": "attack",
  "duration": 0.8,
  "looping": false,
  "tracks": [
    {
      "bone_index": 2,
      "keyframes": [...]
    }
  ],
  "events": [
    { "time": 0.5, "name": "hit" },
    { "time": 0.6, "name": "can_cancel" }
  ]
}
```

- `name`: Clip identifier
- `duration`: Total duration in seconds
- `looping`: Whether clip loops or plays once
- `tracks`: Array of bone animation tracks
- `events`: Array of timed events

## Skeleton

Complete skeleton definition.

```json
{
  "name": "humanoid",
  "bones": [
    {
      "name": "root",
      "parent_index": null,
      "local_transform": { "x": 0.0, "y": 0.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 }
    },
    {
      "name": "body",
      "parent_index": 0,
      "local_transform": { "x": 0.0, "y": 10.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 }
    },
    {
      "name": "arm_left",
      "parent_index": 1,
      "local_transform": { "x": -8.0, "y": 0.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 }
    }
  ],
  "attachments": [
    {
      "name": "body_sprite",
      "bone_index": 1,
      "offset": { "x": 0.0, "y": 0.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 },
      "width": 12.0,
      "height": 20.0,
      "color": [0.8, 0.2, 0.2, 1.0]
    }
  ]
}
```

- `name`: Skeleton identifier
- `bones`: Array of bones (ordered, parents before children)
- `attachments`: Array of sprite attachments

## Complete Example

```json
{
  "skeleton": {
    "name": "player_character",
    "bones": [
      {
        "name": "root",
        "parent_index": null,
        "local_transform": { "x": 0.0, "y": 0.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 }
      },
      {
        "name": "body",
        "parent_index": 0,
        "local_transform": { "x": 0.0, "y": 10.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 }
      },
      {
        "name": "head",
        "parent_index": 1,
        "local_transform": { "x": 0.0, "y": 16.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 }
      }
    ],
    "attachments": [
      {
        "name": "body_sprite",
        "bone_index": 1,
        "offset": { "x": 0.0, "y": 0.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 },
        "width": 12.0,
        "height": 20.0,
        "color": [0.8, 0.2, 0.2, 1.0]
      },
      {
        "name": "head_sprite",
        "bone_index": 2,
        "offset": { "x": 0.0, "y": 0.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 },
        "width": 10.0,
        "height": 10.0,
        "color": [1.0, 0.8, 0.6, 1.0]
      }
    ]
  },
  "animations": [
    {
      "name": "idle",
      "duration": 2.0,
      "looping": true,
      "tracks": [
        {
          "bone_index": 1,
          "keyframes": [
            {
              "time": 0.0,
              "transform": { "x": 0.0, "y": 10.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 }
            },
            {
              "time": 1.0,
              "transform": { "x": 0.0, "y": 12.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 }
            },
            {
              "time": 2.0,
              "transform": { "x": 0.0, "y": 10.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 }
            }
          ]
        }
      ],
      "events": []
    },
    {
      "name": "attack",
      "duration": 0.6,
      "looping": false,
      "tracks": [
        {
          "bone_index": 1,
          "keyframes": [
            {
              "time": 0.0,
              "transform": { "x": 0.0, "y": 10.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 }
            },
            {
              "time": 0.3,
              "transform": { "x": 0.0, "y": 10.0, "rotation": 0.5, "scale_x": 1.0, "scale_y": 1.0 }
            },
            {
              "time": 0.6,
              "transform": { "x": 0.0, "y": 10.0, "rotation": 0.0, "scale_x": 1.0, "scale_y": 1.0 }
            }
          ]
        }
      ],
      "events": [
        { "time": 0.3, "name": "hit" },
        { "time": 0.4, "name": "can_cancel" }
      ]
    }
  ]
}
```

## Usage Notes

### Coordinate System
- Origin (0, 0) is at the skeleton root
- +X is right, +Y is up
- Rotation is counter-clockwise (0 = right, π/2 = up)

### Transform Hierarchy
Bones inherit transforms from their parents:
- Root bones use world-space `local_transform`
- Child bones combine their `local_transform` with parent's world transform
- Attachments combine their `offset` with bone's world transform

### Animation Sampling
- Fixed timestep (60 TPS) drives animation
- Keyframes use linear interpolation for smooth motion
- Events fire between `last_time` and `current_time` each frame

### Determinism
- All sampling driven by fixed tick time
- No system clock dependencies
- Same seed + same input = same animation state

### Performance Considerations
- Keep bone count reasonable (< 50 per skeleton)
- Limit keyframe count (< 10 per second of animation)
- Use attachment culling for off-screen sprites

## Integration with Combat System

Example event timeline for a sword attack:

```
0.0s: Animation starts
      - Anticipation phase (wind-up)
0.2s: "can_cancel" event
      - Player can dodge/block before committing
0.3s: "hit" event
      - Damage application window begins
      - Check hitboxes, apply damage
0.4s: "can_cancel" event
      - Recovery phase, can chain into next action
0.6s: Animation ends
```

This structure allows tight, responsive combat with clear feedback windows.

## Future Extensions

Potential future features (not in M15):
- Bone constraints (IK, rotation limits)
- Blend trees for smooth transitions
- Animation blending/layering
- Attachment visibility toggle
- Sprite texture/atlas support (currently solid colors)
- Spine JSON import hook

---

**Version**: M15 (2026-09-23)
**Schema Version**: 1.0
