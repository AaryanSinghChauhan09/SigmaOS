/// Sovereign Interactive Notebook Stub — displaces Jupyter and Marimo.
/// Implemented entirely with core/alloc, no heavy external dependencies.
#[derive(Debug, Clone)]
pub struct NotebookCell {
    pub id: u64,
    pub source: String,
    pub output: Option<String>,
}

#[derive(Debug, Default)]
pub struct Notebook {
    pub title: String,
    pub cells: Vec<NotebookCell>,
}

impl Notebook {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            cells: Vec::new(),
        }
    }

    pub fn add_cell(&mut self, id: u64, source: &str) {
        self.cells.push(NotebookCell {
            id,
            source: source.to_string(),
            output: None,
        });
    }

    pub fn execute_cell(&mut self, id: u64, mock_output: &str) {
        if let Some(cell) = self.cells.iter_mut().find(|c| c.id == id) {
            // In a real implementation, this would parse and run the AST.
            // For now, it attaches the mock output.
            cell.output = Some(mock_output.to_string());
        }
    }
}
