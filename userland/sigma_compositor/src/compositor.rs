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
}

pub struct ZenithCompositor {
    pub layout: Layout,
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
                name: "Floating".to_string(),
                surfaces: Vec::new(),
            },
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
        surface
    }

    pub fn destroy_surface(&mut self, id: u64) -> Result<(), String> {
        let len_before = self.layout.surfaces.len();
        self.layout.surfaces.retain(|s| s.id != id);
        if self.layout.surfaces.len() < len_before {
            Ok(())
        } else {
            Err("Surface not found".to_string())
        }
    }
}
