#[derive(Debug, Clone)]
pub struct WebView {
    pub current_url: String,
    pub title: String,
    pub dom_root: String,
}

pub struct SigmaBrowser {
    pub views: Vec<WebView>,
}

impl Default for SigmaBrowser {
    fn default() -> Self {
        Self::new()
    }
}

impl SigmaBrowser {
    pub fn new() -> Self {
        Self { views: Vec::new() }
    }

    pub fn navigate(&mut self, url: &str) -> Result<WebView, String> {
        let view = WebView {
            current_url: url.to_string(),
            title: "SigmaOS Sovereign Web Page".to_string(),
            dom_root: "<html><body><h1>Welcome to SigmaOS Sovereign Web</h1></body></html>".to_string(),
        };
        self.views.push(view.clone());
        Ok(view)
    }
}
