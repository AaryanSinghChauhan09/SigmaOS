//! Surface management for the SigmaOS compositor.
//!
//! A `Surface` maps to a `wl_surface` Wayland object. It tracks the
//! committed buffer, damage regions, subsurface hierarchy, and input
//! focus state.

use std::collections::VecDeque;

/// Unique identifier for a compositor surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceId(pub u64);

/// Damage region — a rectangle that needs to be redrawn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DamageRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// The role a surface plays in the compositor (xdg-toplevel, subsurface, cursor, etc.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceRole {
    None,
    XdgToplevel,
    XdgPopup,
    Subsurface { parent: SurfaceId },
    LayerShell,
    Cursor,
}

/// Pending state that will become current on the next commit.
#[derive(Debug, Default)]
pub struct SurfacePendingState {
    pub damage: Vec<DamageRect>,
    pub buffer_attached: bool,
    pub input_region: Option<(i32, i32, u32, u32)>,
    pub opaque_region: Option<(i32, i32, u32, u32)>,
}

/// A Wayland surface managed by the compositor.
#[derive(Debug)]
pub struct Surface {
    pub id: SurfaceId,
    pub role: SurfaceRole,
    pub damage: VecDeque<DamageRect>,
    pub current_buffer: Option<u64>, // buffer object ID
    pub pending: SurfacePendingState,
    pub subsurfaces: Vec<SurfaceId>,
    pub mapped: bool,
    pub scale: i32,
}

impl Surface {
    /// Create a new unmapped surface with the given id.
    pub fn new(id: SurfaceId) -> Self {
        Self {
            id,
            role: SurfaceRole::None,
            damage: VecDeque::new(),
            current_buffer: None,
            pending: SurfacePendingState::default(),
            subsurfaces: Vec::new(),
            mapped: false,
            scale: 1,
        }
    }

    /// Assign a role to this surface (may only be done once).
    pub fn assign_role(&mut self, role: SurfaceRole) -> crate::Result<()> {
        if self.role != SurfaceRole::None {
            return Err(crate::CompositorError::Protocol(
                "surface already has a role".to_string(),
            ));
        }
        self.role = role;
        Ok(())
    }

    /// Attach a buffer to the pending state.
    pub fn attach_buffer(&mut self, buffer_id: u64) {
        self.pending.buffer_attached = true;
        self.current_buffer = Some(buffer_id);
    }

    /// Mark a region as damaged in pending state.
    pub fn damage_region(&mut self, rect: DamageRect) {
        self.pending.damage.push(rect);
    }

    /// Commit pending state to current state.
    pub fn commit(&mut self) {
        // Move pending damage into the active damage queue.
        for rect in self.pending.damage.drain(..) {
            self.damage.push_back(rect);
        }
        if self.pending.buffer_attached {
            self.mapped = true;
            self.pending.buffer_attached = false;
        }
    }

    /// Consume all damage regions (called after a frame is rendered).
    pub fn clear_damage(&mut self) {
        self.damage.clear();
    }

    /// Add a subsurface as a child of this surface.
    pub fn add_subsurface(&mut self, child: SurfaceId) {
        if !self.subsurfaces.contains(&child) {
            self.subsurfaces.push(child);
        }
    }

    /// Remove a subsurface child.
    pub fn remove_subsurface(&mut self, child: SurfaceId) {
        self.subsurfaces.retain(|id| *id != child);
    }

    /// Returns true when there is pending damage to render.
    pub fn has_damage(&self) -> bool {
        !self.damage.is_empty()
    }
}
