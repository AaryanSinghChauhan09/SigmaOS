use sigma_design_system::TilingNode;

use crate::input::InputRouter;
use crate::render::{DamageTracker, RenderBackend};

#[derive(Debug, Clone)]
pub struct Surface {
    pub id: u64,
    pub width: u32,
    pub height: u32,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct Layout {
    pub name: String,
    pub surfaces: Vec<Surface>,
    pub root: TilingNode,
}

pub struct ZenithCompositor {
    pub layout: Layout,
    pub input_router: InputRouter,
    pub damage_tracker: DamageTracker,
    pub render_backend: RenderBackend,
}

impl Default for ZenithCompositor {
    fn default() -> Self {
        Self::new()
    }
}

impl ZenithCompositor {
    pub fn new() -> Self {
        Self {
            layout: Layout {
                name: "Tiling".to_string(),
                surfaces: Vec::new(),
                root: TilingNode::Horizontal(Vec::new()),
            },
            input_router: InputRouter::new(),
            damage_tracker: DamageTracker::new(),
            render_backend: RenderBackend,
        }
    }

    pub fn create_surface(&mut self, id: u64, w: u32, h: u32) -> Surface {
        let surface = Surface {
            id,
            width: w,
            height: h,
            active: true,
        };
        self.layout.surfaces.push(surface.clone());
        
        // Add to tiling tree
        if let TilingNode::Horizontal(ref mut children) = self.layout.root {
            children.push(TilingNode::Window(id));
        }

        surface
    }

    pub fn destroy_surface(&mut self, id: u64) -> Result<(), String> {
        let len_before = self.layout.surfaces.len();
        self.layout.surfaces.retain(|s| s.id != id);
        
        // Remove from tiling tree
        if let TilingNode::Horizontal(ref mut children) = self.layout.root {
            children.retain(|node| match node {
                TilingNode::Window(wid) => *wid != id,
                _ => true,
            });
        }

        if self.layout.surfaces.len() < len_before {
            Ok(())
        } else {
            Err("Surface not found".to_string())
        }
    }
}
