#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub payload: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Actor {
    pub id: u64,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct ObjectStore {
    objects: std::collections::HashMap<u64, String>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            objects: std::collections::HashMap::new(),
        }
    }

    pub fn put(&mut self, id: u64, data: &str) {
        self.objects.insert(id, data.to_string());
    }

    pub fn get(&self, id: u64) -> Option<String> {
        self.objects.get(&id).cloned()
    }
}

pub struct DistributedEngine {
    pub tasks: Vec<Task>,
    pub actors: Vec<Actor>,
    pub object_store: ObjectStore,
}

impl Default for DistributedEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DistributedEngine {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            actors: Vec::new(),
            object_store: ObjectStore::new(),
        }
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

    pub fn spawn_actor(&mut self, id: u64) -> Actor {
        let actor = Actor {
            id,
            state: "Initialized".to_string(),
        };
        self.actors.push(actor.clone());
        actor
    }
}
