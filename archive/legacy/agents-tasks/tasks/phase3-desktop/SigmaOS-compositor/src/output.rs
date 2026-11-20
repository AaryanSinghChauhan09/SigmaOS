//! Output (display) management for the SigmaOS compositor.

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputTransform {
    #[default]
    Normal,
    Rotate90,
    Rotate180,
    Rotate270,
}

/// A physical or virtual display output.
#[derive(Debug)]
pub struct CompositorOutput {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub modes: Vec<OutputMode>,
    pub current_mode: Option<usize>,
    pub scale: f64,
    pub transform: OutputTransform,
    pub enabled: bool,
}

impl CompositorOutput {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            x: 0,
            y: 0,
            modes: Vec::new(),
            current_mode: None,
            scale: 1.0,
            transform: OutputTransform::Normal,
            enabled: true,
        }
    }

    pub fn add_mode(&mut self, mode: OutputMode) {
        if self.current_mode.is_none() {
            self.current_mode = Some(0);
        }
        self.modes.push(mode);
    }

    pub fn active_mode(&self) -> Option<&OutputMode> {
        self.current_mode.map(|i| &self.modes[i])
    }

    pub fn logical_width(&self) -> u32 {
        self.active_mode()
            .map(|m| (m.width as f64 / self.scale) as u32)
            .unwrap_or(0)
    }

    pub fn logical_height(&self) -> u32 {
        self.active_mode()
            .map(|m| (m.height as f64 / self.scale) as u32)
            .unwrap_or(0)
    }
}

/// Layout of all active outputs.
#[derive(Debug, Default)]
pub struct OutputLayout {
    pub outputs: Vec<CompositorOutput>,
}

impl OutputLayout {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, output: CompositorOutput) {
        self.outputs.push(output);
    }

    pub fn remove(&mut self, name: &str) {
        self.outputs.retain(|o| o.name != name);
    }

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
