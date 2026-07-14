pub struct Panel {
    pub position: String,
    pub height: u32,
    pub widgets: Vec<String>,
}

pub struct SigmaDesktop {
    pub panel: Panel,
    pub background_uri: String,
}

impl Default for SigmaDesktop {
    fn default() -> Self {
        Self::new()
    }
}

impl SigmaDesktop {
    pub fn new() -> Self {
        Self {
            panel: Panel {
                position: "Top".to_string(),
                height: 32,
                widgets: vec!["Clock".to_string(), "StatusIndicator".to_string(), "Launcher".to_string()],
            },
            background_uri: "file:///usr/share/backgrounds/sigma_default.png".to_string(),
        }
    }

    pub fn set_background(&mut self, uri: &str) {
        self.background_uri = uri.to_string();
    }
}
