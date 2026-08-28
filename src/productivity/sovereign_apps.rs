// Sovereign Core Apps Shard (SigmaOffice, SigmaTasks, SigmaVault)
// Zero-dependency, #![no_std] compliant


const MAX_NODES: usize = 16;
const MAX_TASKS: usize = 16;
const MAX_SECRETS: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct TextNode {
    pub node_id: u32,
    pub parent_id: u32,
    pub formatting_flags: u8,
    pub content_hash: u32,
}

pub struct SigmaOfficeDocument {
    pub nodes: [Option<TextNode>; MAX_NODES],
    pub next_node_id: u32,
}

impl SigmaOfficeDocument {
    pub fn new() -> Self {
        const EMPTY_NODE: Option<TextNode> = None;
        Self {
            nodes: [EMPTY_NODE; MAX_NODES],
            next_node_id: 1,
        }
    }

    pub fn add_node(
        &mut self,
        parent_id: u32,
        formatting_flags: u8,
        content_hash: u32,
    ) -> Result<u32, &'static str> {
        let id = self.next_node_id;
        let node = TextNode {
            node_id: id,
            parent_id,
            formatting_flags,
            content_hash,
        };

        for slot in self.nodes.iter_mut() {
            if slot.is_none() {
                *slot = Some(node);
                self.next_node_id += 1;
                return Ok(id);
            }
        }

        Err("SigmaOfficeDocument: Document node limit reached")
    }
}

impl Default for SigmaOfficeDocument {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Copy)]
pub struct ProductivityTask {
    pub task_id: u32,
    pub title_hash: u32,
    pub priority: TaskPriority,
    pub is_completed: bool,
}

pub struct SigmaTasksBoard {
    pub tasks: [Option<ProductivityTask>; MAX_TASKS],
    pub next_task_id: u32,
}

impl SigmaTasksBoard {
    pub fn new() -> Self {
        const EMPTY_TASK: Option<ProductivityTask> = None;
        Self {
            tasks: [EMPTY_TASK; MAX_TASKS],
            next_task_id: 1,
        }
    }

    pub fn add_task(
        &mut self,
        title_hash: u32,
        priority: TaskPriority,
    ) -> Result<u32, &'static str> {
        let id = self.next_task_id;
        let task = ProductivityTask {
            task_id: id,
            title_hash,
            priority,
            is_completed: false,
        };

        for slot in self.tasks.iter_mut() {
            if slot.is_none() {
                *slot = Some(task);
                self.next_task_id += 1;
                return Ok(id);
            }
        }

        Err("SigmaTasksBoard: Task board capacity exceeded")
    }
}

impl Default for SigmaTasksBoard {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SigmaVaultContainer {
    pub secrets: [Option<(u32, [u8; 32])>; MAX_SECRETS],
    pub next_secret_id: u32,
}

impl SigmaVaultContainer {
    pub fn new() -> Self {
        const EMPTY_SECRET: Option<(u32, [u8; 32])> = None;
        Self {
            secrets: [EMPTY_SECRET; MAX_SECRETS],
            next_secret_id: 1,
        }
    }

    pub fn store_secret(&mut self, payload: [u8; 32]) -> Result<u32, &'static str> {
        let id = self.next_secret_id;
        for slot in self.secrets.iter_mut() {
            if slot.is_none() {
                *slot = Some((id, payload));
                self.next_secret_id += 1;
                return Ok(id);
            }
        }

        Err("SigmaVaultContainer: Vault capacity exceeded")
    }
}

impl Default for SigmaVaultContainer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_apps() {
        let mut doc = SigmaOfficeDocument::new();
        let node_id = doc.add_node(0, 0x01, 12345).unwrap();
        assert_eq!(node_id, 1);

        let mut board = SigmaTasksBoard::new();
        let task_id = board.add_task(54321, TaskPriority::High).unwrap();
        assert_eq!(task_id, 1);

        let mut vault = SigmaVaultContainer::new();
        let secret_id = vault.store_secret([0xAA; 32]).unwrap();
        assert_eq!(secret_id, 1);
    }
}
