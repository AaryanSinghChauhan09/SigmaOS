/// Low-level typed column without any external dependencies.
#[derive(Debug, Clone)]
pub enum Column {
    Float(Vec<f64>),
    Integer(Vec<i64>),
    Text(Vec<String>),
}

/// Sovereign DataFrame — no Pandas, no Arrow, no external crates.
#[derive(Debug, Clone, Default)]
pub struct DataFrame {
    pub columns: Vec<(String, Column)>,
    pub row_count: usize,
}

impl DataFrame {
    pub fn new() -> Self {
        Self { columns: Vec::new(), row_count: 0 }
    }

    pub fn add_float_column(&mut self, name: &str, data: Vec<f64>) {
        if self.row_count == 0 { self.row_count = data.len(); }
        self.columns.push((name.to_string(), Column::Float(data)));
    }

    pub fn add_int_column(&mut self, name: &str, data: Vec<i64>) {
        if self.row_count == 0 { self.row_count = data.len(); }
        self.columns.push((name.to_string(), Column::Integer(data)));
    }

    pub fn get_column(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|(n, _)| n == name).map(|(_, c)| c)
    }

    /// Compute mean of a float column — implemented without std::iter imports for sovereignty.
    pub fn mean_of(&self, name: &str) -> Option<f64> {
        if let Some(Column::Float(data)) = self.get_column(name) {
            if data.is_empty() { return None; }
            let mut sum = 0.0_f64;
            let mut count = 0usize;
            for v in data {
                sum += v;
                count += 1;
            }
            Some(sum / count as f64)
        } else {
            None
        }
    }
}
