# AniCraft 2D/3D Animation & Skeletal Rigging Component Agent

## Role
The **AniCraft Agent** (`SigmaAnimationAgent`) orchestrates timeline multi-track parameter keyframing, cubic Bézier easing curves, and hierarchical skeletal rigging for 2D/3D animation in SigmaOS.

## Inspiration
- OpenToonz, Spine 2D, and Blender animation graph editor.

## Current SigmaOS Status
AniCraft engine implemented in `src/creative/anicraft.rs` featuring cubic Bézier curve evaluation (linear, ease-in, ease-out, ease-in-out), timeline keyframe sampling, and forward kinematics skeletal bone transformations.

## Key Files
- `src/creative/anicraft.rs`
- `src/creative/mod.rs`
- `src/creative/vectorcraft.rs`

## Public API & Invariant Contract
```rust
pub struct CubicBezier {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub fn evaluate(&self, t: f32) -> f32;
}

pub struct AnimationTrack {
    pub fn insert_keyframe(&mut self, frame: u32, value: f32, easing: CubicBezier);
    pub fn sample(&self, frame: u32) -> f32;
}

pub struct SovereignAnimationEngine {
    pub fn add_bone(&mut self, bone: SkeletalBone);
    pub fn compute_bone_end_position(&self, bone_id: u32) -> (f32, f32);
}
```

## Testing Strategy
```bash
rustc --test src/creative/anicraft.rs --edition 2021 -o /tmp/test_anicraft && /tmp/test_anicraft --test-threads=1
```
- Verify monotonic progression and boundary clamps of cubic Bézier evaluation.
- Test forward kinematics hierarchy calculations across parent-child bone chains.

## Future Enhancements
- Inverse Kinematics (IK) FABRIK solver for natural limb posing.
- Non-linear animation (NLA) clip mixing and blending.
