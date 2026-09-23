//! 2D skeletal animation system
//!
//! Features:
//! - Bone hierarchy with local/world transforms
//! - Sprite slots attached to bones
//! - Keyframe animation with interpolation
//! - Event tracks for combat/gameplay hooks
//! - JSON serialization for AI authoring
//! - Deterministic sampling (fixed timestep)

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 2D Transform (position, rotation, scale)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform2D {
    pub x: f32,
    pub y: f32,
    pub rotation: f32, // radians
    pub scale_x: f32,
    pub scale_y: f32,
}

impl Transform2D {
    pub fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }

    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }

    /// Combine transforms: child relative to parent
    pub fn combine(&self, parent: &Transform2D) -> Transform2D {
        let cos = parent.rotation.cos();
        let sin = parent.rotation.sin();
        
        let scaled_x = self.x * parent.scale_x;
        let scaled_y = self.y * parent.scale_y;
        
        let rotated_x = scaled_x * cos - scaled_y * sin;
        let rotated_y = scaled_x * sin + scaled_y * cos;
        
        Transform2D {
            x: parent.x + rotated_x,
            y: parent.y + rotated_y,
            rotation: parent.rotation + self.rotation,
            scale_x: parent.scale_x * self.scale_x,
            scale_y: parent.scale_y * self.scale_y,
        }
    }

    /// Linear interpolation
    pub fn lerp(&self, other: &Transform2D, t: f32) -> Transform2D {
        Transform2D {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            rotation: self.rotation + (other.rotation - self.rotation) * t,
            scale_x: self.scale_x + (other.scale_x - self.scale_x) * t,
            scale_y: self.scale_y + (other.scale_y - self.scale_y) * t,
        }
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::identity()
    }
}

/// Bone in skeleton hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bone {
    pub name: String,
    pub parent_index: Option<usize>,
    pub local_transform: Transform2D,
    #[serde(skip)]
    pub world_transform: Transform2D,
}

impl Bone {
    pub fn new(name: String, parent_index: Option<usize>, local_transform: Transform2D) -> Self {
        Self {
            name,
            parent_index,
            local_transform,
            world_transform: local_transform,
        }
    }
}

/// Sprite attachment (slot) bound to a bone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub name: String,
    pub bone_index: usize,
    pub offset: Transform2D,
    pub width: f32,
    pub height: f32,
    pub color: [f32; 4],
}

impl Attachment {
    pub fn new(name: String, bone_index: usize, width: f32, height: f32, color: [f32; 4]) -> Self {
        Self {
            name,
            bone_index,
            offset: Transform2D::identity(),
            width,
            height,
            color,
        }
    }

    pub fn with_offset(mut self, offset: Transform2D) -> Self {
        self.offset = offset;
        self
    }
}

/// Keyframe for a single bone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub time: f32,
    pub transform: Transform2D,
}

/// Animation track for a single bone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneTrack {
    pub bone_index: usize,
    pub keyframes: Vec<Keyframe>,
}

impl BoneTrack {
    pub fn new(bone_index: usize) -> Self {
        Self {
            bone_index,
            keyframes: Vec::new(),
        }
    }

    pub fn add_keyframe(&mut self, time: f32, transform: Transform2D) {
        self.keyframes.push(Keyframe { time, transform });
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    pub fn sample(&self, time: f32) -> Transform2D {
        if self.keyframes.is_empty() {
            return Transform2D::identity();
        }

        if self.keyframes.len() == 1 {
            return self.keyframes[0].transform;
        }

        if time <= self.keyframes[0].time {
            return self.keyframes[0].transform;
        }

        if time >= self.keyframes.last().unwrap().time {
            return self.keyframes.last().unwrap().transform;
        }

        for i in 0..self.keyframes.len() - 1 {
            let kf0 = &self.keyframes[i];
            let kf1 = &self.keyframes[i + 1];

            if time >= kf0.time && time <= kf1.time {
                let t = (time - kf0.time) / (kf1.time - kf0.time);
                return kf0.transform.lerp(&kf1.transform, t);
            }
        }

        self.keyframes.last().unwrap().transform
    }
}

/// Animation event (e.g., "hit", "can_cancel")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationEvent {
    pub time: f32,
    pub name: String,
}

/// Animation clip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationClip {
    pub name: String,
    pub duration: f32,
    pub looping: bool,
    pub tracks: Vec<BoneTrack>,
    pub events: Vec<AnimationEvent>,
}

impl AnimationClip {
    pub fn new(name: String, duration: f32, looping: bool) -> Self {
        Self {
            name,
            duration,
            looping,
            tracks: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn add_track(&mut self, track: BoneTrack) {
        self.tracks.push(track);
    }

    pub fn add_event(&mut self, time: f32, name: String) {
        self.events.push(AnimationEvent { time, name });
        self.events.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    pub fn get_track(&self, bone_index: usize) -> Option<&BoneTrack> {
        self.tracks.iter().find(|t| t.bone_index == bone_index)
    }

    /// Get events that fired between last_time and current_time
    pub fn get_events_in_range(&self, last_time: f32, current_time: f32) -> Vec<&AnimationEvent> {
        self.events
            .iter()
            .filter(|e| e.time > last_time && e.time <= current_time)
            .collect()
    }
}

/// Skeleton definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skeleton {
    pub name: String,
    pub bones: Vec<Bone>,
    pub attachments: Vec<Attachment>,
}

impl Skeleton {
    pub fn new(name: String) -> Self {
        Self {
            name,
            bones: Vec::new(),
            attachments: Vec::new(),
        }
    }

    pub fn add_bone(&mut self, bone: Bone) -> usize {
        let index = self.bones.len();
        self.bones.push(bone);
        index
    }

    pub fn add_attachment(&mut self, attachment: Attachment) {
        self.attachments.push(attachment);
    }

    /// Solve world transforms for all bones (forward kinematics)
    pub fn update_world_transforms(&mut self) {
        for i in 0..self.bones.len() {
            if let Some(parent_index) = self.bones[i].parent_index {
                let parent_world = self.bones[parent_index].world_transform;
                self.bones[i].world_transform = self.bones[i].local_transform.combine(&parent_world);
            } else {
                self.bones[i].world_transform = self.bones[i].local_transform;
            }
        }
    }

    pub fn find_bone_index(&self, name: &str) -> Option<usize> {
        self.bones.iter().position(|b| b.name == name)
    }
}

/// Animation state
#[derive(Debug, Clone)]
pub struct AnimationState {
    pub clip_name: String,
    pub time: f32,
    pub last_time: f32,
    pub speed: f32,
    pub looping: bool,
    pub finished: bool,
}

impl AnimationState {
    pub fn new(clip_name: String, looping: bool) -> Self {
        Self {
            clip_name,
            time: 0.0,
            last_time: 0.0,
            speed: 1.0,
            looping,
            finished: false,
        }
    }
}

/// Animator (state machine)
pub struct Animator {
    pub skeleton: Skeleton,
    pub clips: HashMap<String, AnimationClip>,
    pub current_state: Option<AnimationState>,
    pub fired_events: Vec<String>,
}

impl Animator {
    pub fn new(skeleton: Skeleton) -> Self {
        Self {
            skeleton,
            clips: HashMap::new(),
            current_state: None,
            fired_events: Vec::new(),
        }
    }

    pub fn add_clip(&mut self, clip: AnimationClip) {
        self.clips.insert(clip.name.clone(), clip);
    }

    /// Play animation clip
    pub fn play(&mut self, clip_name: &str) -> Result<()> {
        let clip = self.clips.get(clip_name)
            .ok_or_else(|| anyhow::anyhow!("Clip '{}' not found", clip_name))?;

        self.current_state = Some(AnimationState::new(clip_name.to_string(), clip.looping));
        self.fired_events.clear();
        Ok(())
    }

    /// Update animation (fixed timestep)
    pub fn update(&mut self, dt: f32) {
        let sample_time = if let Some(ref mut state) = self.current_state {
            if state.finished {
                return;
            }

            let clip = match self.clips.get(&state.clip_name) {
                Some(c) => c,
                None => return,
            };

            state.last_time = state.time;
            state.time += dt * state.speed;

            if state.time >= clip.duration {
                if state.looping {
                    state.time = state.time % clip.duration;
                } else {
                    state.time = clip.duration;
                    state.finished = true;
                }
            }

            let events = clip.get_events_in_range(state.last_time, state.time);
            for event in events {
                self.fired_events.push(event.name.clone());
            }

            state.time
        } else {
            return;
        };

        self.sample(sample_time);
    }

    /// Sample animation at time and apply to skeleton
    fn sample(&mut self, time: f32) {
        let state = match &self.current_state {
            Some(s) => s,
            None => return,
        };

        let clip = match self.clips.get(&state.clip_name) {
            Some(c) => c,
            None => return,
        };

        for (i, bone) in self.skeleton.bones.iter_mut().enumerate() {
            if let Some(track) = clip.get_track(i) {
                bone.local_transform = track.sample(time);
            }
        }

        self.skeleton.update_world_transforms();
    }

    /// Get world transform of attachment
    pub fn get_attachment_world_transform(&self, attachment_index: usize) -> Option<Transform2D> {
        let attachment = self.skeleton.attachments.get(attachment_index)?;
        let bone = self.skeleton.bones.get(attachment.bone_index)?;
        Some(attachment.offset.combine(&bone.world_transform))
    }

    /// Check if animation is finished
    pub fn is_finished(&self) -> bool {
        self.current_state.as_ref().map(|s| s.finished).unwrap_or(true)
    }

    /// Get current time
    pub fn current_time(&self) -> f32 {
        self.current_state.as_ref().map(|s| s.time).unwrap_or(0.0)
    }

    /// Pop fired events (consume them)
    pub fn take_events(&mut self) -> Vec<String> {
        std::mem::take(&mut self.fired_events)
    }
}

/// Sprite data for rendering
#[derive(Debug, Clone)]
pub struct AnimatedSprite {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub color: [f32; 4],
}

impl Animator {
    /// Emit sprites for rendering
    pub fn emit_sprites(&self) -> Vec<AnimatedSprite> {
        let mut sprites = Vec::new();

        for attachment in &self.skeleton.attachments {
            if let Some(transform) = self.get_attachment_world_transform(
                self.skeleton.attachments.iter().position(|a| a.name == attachment.name).unwrap()
            ) {
                sprites.push(AnimatedSprite {
                    x: transform.x,
                    y: transform.y,
                    width: attachment.width,
                    height: attachment.height,
                    rotation: transform.rotation,
                    scale_x: transform.scale_x,
                    scale_y: transform.scale_y,
                    color: attachment.color,
                });
            }
        }

        sprites
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_identity() {
        let t = Transform2D::identity();
        assert_eq!(t.x, 0.0);
        assert_eq!(t.y, 0.0);
        assert_eq!(t.rotation, 0.0);
        assert_eq!(t.scale_x, 1.0);
        assert_eq!(t.scale_y, 1.0);
    }

    #[test]
    fn transform_combine() {
        let parent = Transform2D::new(10.0, 20.0);
        let child = Transform2D::new(5.0, 0.0);
        
        let world = child.combine(&parent);
        assert!((world.x - 15.0).abs() < 0.001);
        assert!((world.y - 20.0).abs() < 0.001);
    }

    #[test]
    fn transform_lerp() {
        let a = Transform2D::new(0.0, 0.0);
        let b = Transform2D::new(10.0, 20.0);
        
        let mid = a.lerp(&b, 0.5);
        assert!((mid.x - 5.0).abs() < 0.001);
        assert!((mid.y - 10.0).abs() < 0.001);
    }

    #[test]
    fn skeleton_hierarchy() {
        let mut skeleton = Skeleton::new("test".to_string());
        
        let root_idx = skeleton.add_bone(Bone::new(
            "root".to_string(),
            None,
            Transform2D::new(10.0, 20.0),
        ));
        
        skeleton.add_bone(Bone::new(
            "child".to_string(),
            Some(root_idx),
            Transform2D::new(5.0, 0.0),
        ));
        
        skeleton.update_world_transforms();
        
        assert_eq!(skeleton.bones[0].world_transform.x, 10.0);
        assert!((skeleton.bones[1].world_transform.x - 15.0).abs() < 0.001);
    }

    #[test]
    fn bone_track_sample() {
        let mut track = BoneTrack::new(0);
        track.add_keyframe(0.0, Transform2D::new(0.0, 0.0));
        track.add_keyframe(1.0, Transform2D::new(10.0, 20.0));
        
        let mid = track.sample(0.5);
        assert!((mid.x - 5.0).abs() < 0.001);
        assert!((mid.y - 10.0).abs() < 0.001);
        
        let start = track.sample(0.0);
        assert_eq!(start.x, 0.0);
        
        let end = track.sample(1.0);
        assert_eq!(end.x, 10.0);
    }

    #[test]
    fn animation_events() {
        let mut clip = AnimationClip::new("test".to_string(), 2.0, false);
        clip.add_event(0.5, "event1".to_string());
        clip.add_event(1.5, "event2".to_string());
        
        let events = clip.get_events_in_range(0.0, 0.6);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "event1");
        
        let events = clip.get_events_in_range(0.6, 1.6);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "event2");
    }

    #[test]
    fn animator_playback() {
        let skeleton = Skeleton::new("test".to_string());
        let mut animator = Animator::new(skeleton);
        
        let mut clip = AnimationClip::new("idle".to_string(), 1.0, true);
        let mut track = BoneTrack::new(0);
        track.add_keyframe(0.0, Transform2D::new(0.0, 0.0));
        track.add_keyframe(1.0, Transform2D::new(10.0, 0.0));
        clip.add_track(track);
        
        animator.add_clip(clip);
        animator.play("idle").unwrap();
        
        animator.update(0.5);
        assert!((animator.current_time() - 0.5).abs() < 0.001);
        
        animator.update(0.6);
        assert!((animator.current_time() - 0.1).abs() < 0.1);
    }

    #[test]
    fn animator_events_fire() {
        let skeleton = Skeleton::new("test".to_string());
        let mut animator = Animator::new(skeleton);
        
        let mut clip = AnimationClip::new("attack".to_string(), 1.0, false);
        clip.add_event(0.5, "hit".to_string());
        
        animator.add_clip(clip);
        animator.play("attack").unwrap();
        
        animator.update(0.6);
        let events = animator.take_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], "hit");
    }
}
