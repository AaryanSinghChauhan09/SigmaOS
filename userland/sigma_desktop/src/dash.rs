/// Represents the application launcher overlay (similar to GNOME Dash).
pub struct Dash {
    pub is_open: bool,
    pub search_query: String,
}

impl Default for Dash {
    fn default() -> Self {
        Self::new()
    }
}

impl Dash {
    pub fn new() -> Self {
        Self {
            is_open: false,
            search_query: String::new(),
        }
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn render(&self) {
        if self.is_open {
            // Render logic here: draw search bar and app grid
        }
    }
}
