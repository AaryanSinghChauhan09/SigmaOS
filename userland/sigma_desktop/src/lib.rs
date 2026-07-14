pub mod panel;
pub mod dash;
pub mod notification;

pub use panel::Panel;
pub use dash::Dash;
pub use notification::NotificationCenter;

/// SigmaDesktop: The native Rust UI shell.
/// Displaces GNOME/Plasma desktop shells, tightly integrated with `sigma_compositor`.
pub struct SigmaDesktop {
    pub panel: Panel,
    pub dash: Dash,
    pub notification_center: NotificationCenter,
}

impl Default for SigmaDesktop {
    fn default() -> Self {
        Self::new()
    }
}

impl SigmaDesktop {
    pub fn new() -> Self {
        Self {
            panel: Panel::new(),
            dash: Dash::new(),
            notification_center: NotificationCenter::new(),
        }
    }

    /// Renders all desktop components to the compositor.
    pub fn render(&mut self) {
        self.panel.render();
        self.dash.render();
        self.notification_center.render();
    }
}
