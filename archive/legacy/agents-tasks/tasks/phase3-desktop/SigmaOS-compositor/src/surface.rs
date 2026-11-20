//! Surface management for the SigmaOS compositor.

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

/// The role a surface plays in the compositor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceRole {
    None,
    XdgToplevel,
    XdgPopup,
    Subsurface { parent: SurfaceId },
    LayerShell,
    Cursor,
}

/// Pending state applied on the next commit.
#[derive(Debug, Default)]
pub struct SurfacePendingState {
    pub damage: Vec<DamageRect>,
    pub buffer_attached: bool,
}

/// A Wayland surface managed by the compositor.
#[derive(Debug)]
pub struct Surface {
    pub id: SurfaceId,
    pub role: SurfaceRole,
    pub damage: VecDeque<DamageRect>,
    pub current_buffer: Option<u64>,
    pub pending: SurfacePendingState,
    pub subsurfaces: Vec<SurfaceId>,
    pub mapped: bool,
    pub scale: i32,
}

impl Surface {
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

    pub fn assign_role(&mut self, role: SurfaceRole) -> Result<(), String> {
        if self.role != SurfaceRole::None {
            return Err("surface already has a role".to_string());
        }
        self.role = role;
        Ok(())
    }

    pub fn attach_buffer(&mut self, buffer_id: u64) {
        self.pending.buffer_attached = true;
        self.current_buffer = Some(buffer_id);
    }

    pub fn damage_region(&mut self, rect: DamageRect) {
        self.pending.damage.push(rect);
    }

    pub fn commit(&mut self) {
        for rect in self.pending.damage.drain(..) {
            self.damage.push_back(rect);
        }
        if self.pending.buffer_attached {
            self.mapped = true;
            self.pending.buffer_attached = false;
        }
    }

    pub fn clear_damage(&mut self) {
        self.damage.clear();
    }

    pub fn add_subsurface(&mut self, child: SurfaceId) {
        if !self.subsurfaces.contains(&child) {
            self.subsurfaces.push(child);
        }
    }

    pub fn remove_subsurface(&mut self, child: SurfaceId) {
        self.subsurfaces.retain(|id| *id != child);
    }

    pub fn has_damage(&self) -> bool {
        !self.damage.is_empty()
    }
}
