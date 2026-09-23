//! Humanoid character setup with locomotion and combat animations

use crate::animation::{Animator, AnimationClip, Bone, BoneTrack, Keyframe, Skeleton, Transform2D};
use std::collections::HashMap;

/// Create humanoid skeleton (head, torso, arms, legs)
pub fn create_humanoid_skeleton() -> Skeleton {
    let mut skeleton = Skeleton::new("humanoid".to_string());
    
    // Bone 0: Root (hip)
    skeleton.add_bone(Bone::new(
        "root".to_string(),
        None,
        Transform2D::new(0.0, 0.0),
    ));
    
    // Bone 1: Torso
    skeleton.add_bone(Bone::new(
        "torso".to_string(),
        Some(0),
        Transform2D::new(0.0, -8.0),
    ));
    
    // Bone 2: Head
    skeleton.add_bone(Bone::new(
        "head".to_string(),
        Some(1),
        Transform2D::new(0.0, -6.0),
    ));
    
    // Bone 3: Left Upper Arm
    skeleton.add_bone(Bone::new(
        "left_upper_arm".to_string(),
        Some(1),
        Transform2D::new(-4.0, -6.0),
    ));
    
    // Bone 4: Left Lower Arm
    skeleton.add_bone(Bone::new(
        "left_lower_arm".to_string(),
        Some(3),
        Transform2D::new(0.0, -5.0),
    ));
    
    // Bone 5: Right Upper Arm
    skeleton.add_bone(Bone::new(
        "right_upper_arm".to_string(),
        Some(1),
        Transform2D::new(4.0, -6.0),
    ));
    
    // Bone 6: Right Lower Arm
    skeleton.add_bone(Bone::new(
        "right_lower_arm".to_string(),
        Some(5),
        Transform2D::new(0.0, -5.0),
    ));
    
    // Bone 7: Left Upper Leg
    skeleton.add_bone(Bone::new(
        "left_upper_leg".to_string(),
        Some(0),
        Transform2D::new(-2.0, 4.0),
    ));
    
    // Bone 8: Left Lower Leg
    skeleton.add_bone(Bone::new(
        "left_lower_leg".to_string(),
        Some(7),
        Transform2D::new(0.0, 6.0),
    ));
    
    // Bone 9: Right Upper Leg
    skeleton.add_bone(Bone::new(
        "right_upper_leg".to_string(),
        Some(0),
        Transform2D::new(2.0, 4.0),
    ));
    
    // Bone 10: Right Lower Leg
    skeleton.add_bone(Bone::new(
        "right_lower_leg".to_string(),
        Some(9),
        Transform2D::new(0.0, 6.0),
    ));
    
    skeleton
}

/// Create walk animation clip
fn create_walk_clip() -> AnimationClip {
    let mut clip = AnimationClip::new("walk".to_string(), 1.0, true);
    
    // Left leg swing
    let mut left_leg = BoneTrack::new(7);
    left_leg.add_keyframe(0.0, Transform2D { x: -2.0, y: 4.0, rotation: 0.3, scale_x: 1.0, scale_y: 1.0 });
    left_leg.add_keyframe(0.5, Transform2D { x: -2.0, y: 4.0, rotation: -0.3, scale_x: 1.0, scale_y: 1.0 });
    left_leg.add_keyframe(1.0, Transform2D { x: -2.0, y: 4.0, rotation: 0.3, scale_x: 1.0, scale_y: 1.0 });
    clip.add_track(left_leg);
    
    // Right leg swing (opposite phase)
    let mut right_leg = BoneTrack::new(9);
    right_leg.add_keyframe(0.0, Transform2D { x: 2.0, y: 4.0, rotation: -0.3, scale_x: 1.0, scale_y: 1.0 });
    right_leg.add_keyframe(0.5, Transform2D { x: 2.0, y: 4.0, rotation: 0.3, scale_x: 1.0, scale_y: 1.0 });
    right_leg.add_keyframe(1.0, Transform2D { x: 2.0, y: 4.0, rotation: -0.3, scale_x: 1.0, scale_y: 1.0 });
    clip.add_track(right_leg);
    
    // Arm swing
    let mut left_arm = BoneTrack::new(3);
    left_arm.add_keyframe(0.0, Transform2D { x: -4.0, y: -6.0, rotation: -0.2, scale_x: 1.0, scale_y: 1.0 });
    left_arm.add_keyframe(0.5, Transform2D { x: -4.0, y: -6.0, rotation: 0.2, scale_x: 1.0, scale_y: 1.0 });
    left_arm.add_keyframe(1.0, Transform2D { x: -4.0, y: -6.0, rotation: -0.2, scale_x: 1.0, scale_y: 1.0 });
    clip.add_track(left_arm);
    
    clip.add_event(0.5, "footstep".to_string());
    clip.add_event(1.0, "footstep".to_string());
    
    clip
}

/// Create run animation clip (faster walk with more exaggeration)
fn create_run_clip() -> AnimationClip {
    let mut clip = AnimationClip::new("run".to_string(), 0.6, true);
    
    // Left leg - more aggressive swing
    let mut left_leg = BoneTrack::new(7);
    left_leg.add_keyframe(0.0, Transform2D { x: -2.0, y: 4.0, rotation: 0.5, scale_x: 1.0, scale_y: 1.0 });
    left_leg.add_keyframe(0.3, Transform2D { x: -2.0, y: 4.0, rotation: -0.5, scale_x: 1.0, scale_y: 1.0 });
    left_leg.add_keyframe(0.6, Transform2D { x: -2.0, y: 4.0, rotation: 0.5, scale_x: 1.0, scale_y: 1.0 });
    clip.add_track(left_leg);
    
    // Right leg
    let mut right_leg = BoneTrack::new(9);
    right_leg.add_keyframe(0.0, Transform2D { x: 2.0, y: 4.0, rotation: -0.5, scale_x: 1.0, scale_y: 1.0 });
    right_leg.add_keyframe(0.3, Transform2D { x: 2.0, y: 4.0, rotation: 0.5, scale_x: 1.0, scale_y: 1.0 });
    right_leg.add_keyframe(0.6, Transform2D { x: 2.0, y: 4.0, rotation: -0.5, scale_x: 1.0, scale_y: 1.0 });
    clip.add_track(right_leg);
    
    // Torso lean
    let mut torso = BoneTrack::new(1);
    torso.add_keyframe(0.0, Transform2D { x: 0.0, y: -8.0, rotation: 0.1, scale_x: 1.0, scale_y: 1.0 });
    clip.add_track(torso);
    
    clip.add_event(0.3, "footstep".to_string());
    clip.add_event(0.6, "footstep".to_string());
    
    clip
}

/// Create jump animation clip
fn create_jump_clip() -> AnimationClip {
    let mut clip = AnimationClip::new("jump".to_string(), 0.4, false);
    
    // Crouch start
    let mut root = BoneTrack::new(0);
    root.add_keyframe(0.0, Transform2D { x: 0.0, y: 0.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
    root.add_keyframe(0.1, Transform2D { x: 0.0, y: 2.0, rotation: 0.0, scale_x: 1.0, scale_y: 0.9 });
    root.add_keyframe(0.4, Transform2D { x: 0.0, y: -3.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.1 });
    clip.add_track(root);
    
    // Arms up
    let mut left_arm = BoneTrack::new(3);
    left_arm.add_keyframe(0.0, Transform2D { x: -4.0, y: -6.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
    left_arm.add_keyframe(0.4, Transform2D { x: -4.0, y: -6.0, rotation: -0.5, scale_x: 1.0, scale_y: 1.0 });
    clip.add_track(left_arm);
    
    let mut right_arm = BoneTrack::new(5);
    right_arm.add_keyframe(0.0, Transform2D { x: 4.0, y: -6.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
    right_arm.add_keyframe(0.4, Transform2D { x: 4.0, y: -6.0, rotation: 0.5, scale_x: 1.0, scale_y: 1.0 });
    clip.add_track(right_arm);
    
    clip.add_event(0.1, "jump_start".to_string());
    
    clip
}

/// Create climb animation clip
fn create_climb_clip() -> AnimationClip {
    let mut clip = AnimationClip::new("climb".to_string(), 1.2, true);
    
    // Alternating arm reach
    let mut left_arm = BoneTrack::new(3);
    left_arm.add_keyframe(0.0, Transform2D { x: -4.0, y: -6.0, rotation: -0.8, scale_x: 1.0, scale_y: 1.0 });
    left_arm.add_keyframe(0.6, Transform2D { x: -4.0, y: -6.0, rotation: -0.3, scale_x: 1.0, scale_y: 1.0 });
    left_arm.add_keyframe(1.2, Transform2D { x: -4.0, y: -6.0, rotation: -0.8, scale_x: 1.0, scale_y: 1.0 });
    clip.add_track(left_arm);
    
    let mut right_arm = BoneTrack::new(5);
    right_arm.add_keyframe(0.0, Transform2D { x: 4.0, y: -6.0, rotation: 0.3, scale_x: 1.0, scale_y: 1.0 });
    right_arm.add_keyframe(0.6, Transform2D { x: 4.0, y: -6.0, rotation: 0.8, scale_x: 1.0, scale_y: 1.0 });
    right_arm.add_keyframe(1.2, Transform2D { x: 4.0, y: -6.0, rotation: 0.3, scale_x: 1.0, scale_y: 1.0 });
    clip.add_track(right_arm);
    
    clip.add_event(0.6, "climb_step".to_string());
    clip.add_event(1.2, "climb_step".to_string());
    
    clip
}

/// Create attack animation clip
fn create_attack_clip() -> AnimationClip {
    let mut clip = AnimationClip::new("attack".to_string(), 0.5, false);
    
    // Wind up
    let mut right_arm = BoneTrack::new(5);
    right_arm.add_keyframe(0.0, Transform2D { x: 4.0, y: -6.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
    right_arm.add_keyframe(0.2, Transform2D { x: 4.0, y: -6.0, rotation: -1.2, scale_x: 1.0, scale_y: 1.0 });
    right_arm.add_keyframe(0.3, Transform2D { x: 4.0, y: -6.0, rotation: 0.8, scale_x: 1.0, scale_y: 1.0 });
    right_arm.add_keyframe(0.5, Transform2D { x: 4.0, y: -6.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
    clip.add_track(right_arm);
    
    // Torso rotation
    let mut torso = BoneTrack::new(1);
    torso.add_keyframe(0.0, Transform2D { x: 0.0, y: -8.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
    torso.add_keyframe(0.2, Transform2D { x: 0.0, y: -8.0, rotation: -0.3, scale_x: 1.0, scale_y: 1.0 });
    torso.add_keyframe(0.3, Transform2D { x: 0.0, y: -8.0, rotation: 0.2, scale_x: 1.0, scale_y: 1.0 });
    torso.add_keyframe(0.5, Transform2D { x: 0.0, y: -8.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
    clip.add_track(torso);
    
    clip.add_event(0.3, "hit".to_string());
    
    clip
}

/// Create dash animation clip
fn create_dash_clip() -> AnimationClip {
    let mut clip = AnimationClip::new("dash".to_string(), 0.3, false);
    
    // Lean forward
    let mut torso = BoneTrack::new(1);
    torso.add_keyframe(0.0, Transform2D { x: 0.0, y: -8.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
    torso.add_keyframe(0.1, Transform2D { x: 2.0, y: -8.0, rotation: 0.3, scale_x: 1.1, scale_y: 0.9 });
    torso.add_keyframe(0.3, Transform2D { x: 0.0, y: -8.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
    clip.add_track(torso);
    
    // Legs tuck
    let mut left_leg = BoneTrack::new(7);
    left_leg.add_keyframe(0.0, Transform2D { x: -2.0, y: 4.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
    left_leg.add_keyframe(0.1, Transform2D { x: -2.0, y: 4.0, rotation: 0.6, scale_x: 1.0, scale_y: 1.0 });
    left_leg.add_keyframe(0.3, Transform2D { x: -2.0, y: 4.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
    clip.add_track(left_leg);
    
    clip.add_event(0.1, "dash_burst".to_string());
    
    clip
}

/// Create idle animation clip
fn create_idle_clip() -> AnimationClip {
    let mut clip = AnimationClip::new("idle".to_string(), 2.0, true);
    
    // Subtle breathing
    let mut torso = BoneTrack::new(1);
    torso.add_keyframe(0.0, Transform2D { x: 0.0, y: -8.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
    torso.add_keyframe(1.0, Transform2D { x: 0.0, y: -8.2, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
    torso.add_keyframe(2.0, Transform2D { x: 0.0, y: -8.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
    clip.add_track(torso);
    
    clip
}

/// Create humanoid character animator with all clips
pub fn create_humanoid_animator() -> Animator {
    let skeleton = create_humanoid_skeleton();
    let mut clips = HashMap::new();
    
    clips.insert("idle".to_string(), create_idle_clip());
    clips.insert("walk".to_string(), create_walk_clip());
    clips.insert("run".to_string(), create_run_clip());
    clips.insert("jump".to_string(), create_jump_clip());
    clips.insert("climb".to_string(), create_climb_clip());
    clips.insert("attack".to_string(), create_attack_clip());
    clips.insert("dash".to_string(), create_dash_clip());
    
    Animator {
        skeleton,
        clips,
        current_state: None,
        fired_events: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn humanoid_skeleton_structure() {
        let skeleton = create_humanoid_skeleton();
        assert_eq!(skeleton.bones.len(), 11); // root + torso + head + 4 limbs (2 bones each)
        assert_eq!(skeleton.bones[0].name, "root");
        assert_eq!(skeleton.bones[1].name, "torso");
        assert_eq!(skeleton.bones[2].name, "head");
    }
    
    #[test]
    fn all_animation_clips_present() {
        let animator = create_humanoid_animator();
        assert!(animator.clips.contains_key("idle"));
        assert!(animator.clips.contains_key("walk"));
        assert!(animator.clips.contains_key("run"));
        assert!(animator.clips.contains_key("jump"));
        assert!(animator.clips.contains_key("climb"));
        assert!(animator.clips.contains_key("attack"));
        assert!(animator.clips.contains_key("dash"));
    }
    
    #[test]
    fn animation_events_fire() {
        let animator = create_humanoid_animator();
        
        // Walk has footstep events
        let walk = animator.clips.get("walk").unwrap();
        assert!(!walk.events.is_empty());
        assert!(walk.events.iter().any(|e| e.name == "footstep"));
        
        // Attack has hit event
        let attack = animator.clips.get("attack").unwrap();
        assert!(attack.events.iter().any(|e| e.name == "hit"));
    }
}
