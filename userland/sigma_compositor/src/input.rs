#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    KeyDown(u32),
    KeyUp(u32),
    MouseMove { dx: i32, dy: i32 },
    MouseClick { button: u8, state: bool },
}

/// The InputRouter routes physical hardware events to the currently focused logical surface.
/// Absorbs libinput/Wayland seat concepts.
pub struct InputRouter {
    pub focused_surface: Option<u64>,
}

impl Default for InputRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl InputRouter {
    pub fn new() -> Self {
        Self {
            focused_surface: None,
        }
    }

    pub fn set_focus(&mut self, surface_id: u64) {
        self.focused_surface = Some(surface_id);
    }

    pub fn dispatch(&self, event: InputEvent) -> Result<(), String> {
        if let Some(surface) = self.focused_surface {
            // In a real implementation, this would send an IPC message (via sigma_ipc)
            // to the application owning the surface.
            // println!("Dispatching event {:?} to surface {}", event, surface);
            Ok(())
        } else {
            Err("No surface focused".into())
        }
    }
}
