#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub payload: String,
    pub status: String,
}

pub struct DistributedEngine {
    pub tasks: Vec<Task>,
}

impl Default for DistributedEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DistributedEngine {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn submit_task(&mut self, id: u64, payload: &str) -> Task {
        let task = Task {
            id,
            payload: payload.to_string(),
            status: "Pending".to_string(),
        };
        self.tasks.push(task.clone());
        task
    }
}
