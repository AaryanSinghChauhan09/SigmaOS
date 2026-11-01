//! Output (display) management for the SigmaOS compositor.
//!
//! Each `Output` represents a physical or virtual display. The compositor
//! maintains a layout of outputs with positions, modes, and HiDPI scale.

/// Unique identifier for an output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutputId(pub u32);

/// A display mode (resolution + refresh rate).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputMode {
    pub width: u32,
    pub height: u32,
    /// Refresh rate in mHz (e.g. 60000 = 60 Hz).
    pub refresh_mhz: u32,
    pub preferred: bool,
}

/// Physical orientation of the display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputTransform {
    Normal,
    Rotate90,
    Rotate180,
    Rotate270,
    Flipped,
    FlippedRotate90,
    FlippedRotate180,
    FlippedRotate270,
}

impl Default for OutputTransform {
    fn default() -> Self {
        Self::Normal
    }
}

/// A physical or virtual display output.
#[derive(Debug)]
pub struct Output {
    pub id: OutputId,
    pub name: String,
    pub make: String,
    pub model: String,
    /// Position in the global compositor coordinate space (pixels).
    pub x: i32,
    pub y: i32,
    pub modes: Vec<OutputMode>,
    pub current_mode: Option<usize>,
    pub scale: f64,
    pub transform: OutputTransform,
    pub enabled: bool,
}

impl Output {
    /// Create a new output with sensible defaults.
    pub fn new(id: OutputId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            make: String::new(),
            model: String::new(),
            x: 0,
            y: 0,
            modes: Vec::new(),
            current_mode: None,
            scale: 1.0,
            transform: OutputTransform::Normal,
            enabled: true,
        }
    }

    /// Add a mode to this output. Marks the first added mode as current.
    pub fn add_mode(&mut self, mode: OutputMode) {
        if self.current_mode.is_none() {
            self.current_mode = Some(0);
        }
        self.modes.push(mode);
    }

    /// Set current mode by index.
    pub fn set_mode(&mut self, index: usize) -> crate::Result<()> {
        if index >= self.modes.len() {
            return Err(crate::CompositorError::OutputConfig(format!(
                "mode index {} out of range",
                index
            )));
        }
        self.current_mode = Some(index);
        Ok(())
    }

    /// Returns the currently active mode, if any.
    pub fn active_mode(&self) -> Option<&OutputMode> {
        self.current_mode.map(|i| &self.modes[i])
    }

    /// Set the output's pixel scale factor (HiDPI).
    pub fn set_scale(&mut self, scale: f64) {
        assert!(scale > 0.0, "scale must be positive");
        self.scale = scale;
    }

    /// Position the output in the global layout.
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    /// Returns the logical (scaled) width of the output.
    pub fn logical_width(&self) -> u32 {
        self.active_mode()
            .map(|m| (m.width as f64 / self.scale) as u32)
            .unwrap_or(0)
    }

    /// Returns the logical (scaled) height of the output.
    pub fn logical_height(&self) -> u32 {
        self.active_mode()
            .map(|m| (m.height as f64 / self.scale) as u32)
            .unwrap_or(0)
    }
}

/// Manages the collection of all active outputs.
#[derive(Debug, Default)]
pub struct OutputLayout {
    pub outputs: Vec<Output>,
    next_id: u32,
}

impl OutputLayout {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an output to the layout and return its id.
    pub fn add_output(&mut self, name: impl Into<String>) -> OutputId {
        let id = OutputId(self.next_id);
        self.next_id += 1;
        self.outputs.push(Output::new(id, name));
        id
    }

    /// Remove an output by id.
    pub fn remove_output(&mut self, id: OutputId) {
        self.outputs.retain(|o| o.id != id);
    }

    /// Find an output by id.
    pub fn find_mut(&mut self, id: OutputId) -> Option<&mut Output> {
        self.outputs.iter_mut().find(|o| o.id == id)
    }

    /// Find an output by id (immutable).
    pub fn find(&self, id: OutputId) -> Option<&Output> {
        self.outputs.iter().find(|o| o.id == id)
    }

    /// Arrange outputs left-to-right, no gaps.
    pub fn auto_arrange(&mut self) {
        let mut x = 0i32;
        for output in &mut self.outputs {
            output.x = x;
            output.y = 0;
            if let Some(mode) = output.active_mode() {
                x += mode.width as i32;
            }
        }
    }
}
