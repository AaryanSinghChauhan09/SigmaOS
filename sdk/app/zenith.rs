// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Zenith UI App SDK bindings (Rust, no_std)
//! =========================================================================

pub trait Widget {
    fn render(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
}

pub struct Window {
    title: &'static str,
    visible: bool,
}

impl Window {
    pub const fn new(title: &'static str) -> Self {
        Self { title, visible: false }
    }
}

impl Widget for Window {
    fn render(&self) -> bool {
        self.visible
    }
    
    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}
