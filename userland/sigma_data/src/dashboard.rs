/// Sovereign Dashboard — displaces Apache Superset and Gradio.
#[derive(Debug, Default)]
pub struct Dashboard {
    pub widgets: Vec<String>,
    pub title: String,
}

impl Dashboard {
    pub fn new(title: &str) -> Self {
        Self { widgets: Vec::new(), title: title.to_string() }
    }

    pub fn add_widget(&mut self, widget: &str) {
        self.widgets.push(widget.to_string());
    }
}
