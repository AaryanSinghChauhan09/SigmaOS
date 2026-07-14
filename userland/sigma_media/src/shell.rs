pub struct SigmaShell {
    pub history_limit: usize,
}

impl Default for SigmaShell {
    fn default() -> Self {
        Self::new()
    }
}

impl SigmaShell {
    pub fn new() -> Self {
        Self { history_limit: 1000 }
    }

    pub fn complete_cmd(&self, input: &str) -> Vec<String> {
        let commands = vec!["sigma-shard", "sigma-container", "sigma-compose", "sigma-security"];
        commands
            .into_iter()
            .filter(|c| c.starts_with(input))
            .map(|c| c.to_string())
            .collect()
    }
}
