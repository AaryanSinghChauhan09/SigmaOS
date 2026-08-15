#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Task Manager
// OOP-based task management with Kanban boards and reminders

use crate::klib::BTreeMap;
use std::time::{Duration, Instant};

/// Task
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: Option<u64>,
    pub created_at: u64,
    pub completed_at: Option<u64>,
    pub tags: Vec<String>,
    pub subtasks: Vec<Subtask>,
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Review,
    Done,
    Archived,
}

/// Task priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

/// Subtask
#[derive(Debug, Clone)]
pub struct Subtask {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

/// Project
#[derive(Debug, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub color: String,
    pub task_ids: Vec<String>,
}

/// Kanban board
#[derive(Debug, Clone)]
pub struct KanbanBoard {
    pub id: String,
    pub name: String,
    pub columns: Vec<KanbanColumn>,
}

/// Kanban column
#[derive(Debug, Clone)]
pub struct KanbanColumn {
    pub id: String,
    pub name: String,
    pub task_ids: Vec<String>,
    pub wip_limit: Option<usize>,
}

/// Reminder
#[derive(Debug, Clone)]
pub struct Reminder {
    pub id: String,
    pub task_id: String,
    pub reminder_time: u64,
    pub reminder_type: ReminderType,
    pub is_dismissed: bool,
}

/// Reminder type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReminderType {
    DueSoon,
    Overdue,
    Custom,
}

/// OOP trait for task storage strategies
pub trait TaskStorage {
    /// Save task
    fn save_task(&mut self, task: &Task) -> Result<(), TaskError>;
    /// Load task
    fn load_task(&self, task_id: &str) -> Result<Task, TaskError>;
    /// Delete task
    fn delete_task(&mut self, task_id: &str) -> Result<(), TaskError>;
    /// List all tasks
    fn list_tasks(&self) -> Result<Vec<Task>, TaskError>;
    /// Get storage name
    fn name(&self) -> &str;
}

/// In-memory task storage
pub struct InMemoryStorage {
    tasks: BTreeMap<String, Task>,
}

impl InMemoryStorage {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
        }
    }
}

impl TaskStorage for InMemoryStorage {
    fn save_task(&mut self, task: &Task) -> Result<(), TaskError> {
        self.tasks.insert(task.id.clone(), task.clone());
        Ok(())
    }

    fn load_task(&self, task_id: &str) -> Result<Task, TaskError> {
        self.tasks
            .get(task_id)
            .cloned()
            .ok_or_else(|| TaskError::TaskNotFound(task_id.to_string()))
    }

    fn delete_task(&mut self, task_id: &str) -> Result<(), TaskError> {
        self.tasks
            .remove(task_id)
            .ok_or_else(|| TaskError::TaskNotFound(task_id.to_string()))?;
        Ok(())
    }

    fn list_tasks(&self) -> Result<Vec<Task>, TaskError> {
        Ok(self.tasks.values().cloned().collect())
    }

    fn name(&self) -> &str {
        "InMemoryStorage"
    }
}

/// OOP-based Task Manager
pub struct TaskManager {
    storage: Box<dyn TaskStorage>,
    projects: BTreeMap<String, Project>,
    boards: BTreeMap<String, KanbanBoard>,
    reminders: Vec<Reminder>,
    auto_cleanup_enabled: bool,
}

impl TaskManager {
    pub fn new(storage: Box<dyn TaskStorage>) -> Self {
        Self {
            storage,
            projects: BTreeMap::new(),
            boards: BTreeMap::new(),
            reminders: Vec::new(),
            auto_cleanup_enabled: false,
        }
    }

    /// Enable auto-cleanup
    pub fn with_auto_cleanup(mut self, enabled: bool) -> Self {
        self.auto_cleanup_enabled = enabled;
        self
    }

    /// Create task
    pub fn create_task(
        &mut self,
        title: String,
        description: String,
        priority: TaskPriority,
    ) -> Result<String, TaskError> {
        let task_id = format!(
            "task_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );

        let task = Task {
            id: task_id.clone(),
            title,
            description,
            status: TaskStatus::Todo,
            priority,
            due_date: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            completed_at: None,
            tags: Vec::new(),
            subtasks: Vec::new(),
        };

        self.storage.save_task(&task)?;
        Ok(task_id)
    }

    /// Update task
    pub fn update_task(&mut self, task: Task) -> Result<(), TaskError> {
        self.storage.save_task(&task)
    }

    /// Get task
    pub fn get_task(&self, task_id: &str) -> Result<Task, TaskError> {
        self.storage.load_task(task_id)
    }

    /// Delete task
    pub fn delete_task(&mut self, task_id: &str) -> Result<(), TaskError> {
        self.storage.delete_task(task_id)
    }

    /// List all tasks
    pub fn list_tasks(&self) -> Result<Vec<Task>, TaskError> {
        self.storage.list_tasks()
    }

    /// Filter tasks by status
    pub fn filter_by_status(&self, status: TaskStatus) -> Result<Vec<Task>, TaskError> {
        let tasks = self.storage.list_tasks()?;
        Ok(tasks.into_iter().filter(|t| t.status == status).collect())
    }

    /// Filter tasks by priority
    pub fn filter_by_priority(&self, priority: TaskPriority) -> Result<Vec<Task>, TaskError> {
        let tasks = self.storage.list_tasks()?;
        Ok(tasks
            .into_iter()
            .filter(|t| t.priority == priority)
            .collect())
    }

    /// Search tasks
    pub fn search_tasks(&self, query: &str) -> Result<Vec<Task>, TaskError> {
        let tasks = self.storage.list_tasks()?;
        let query_lower = query.to_lowercase();
        Ok(tasks
            .into_iter()
            .filter(|t| {
                t.title.to_lowercase().contains(&query_lower)
                    || t.description.to_lowercase().contains(&query_lower)
            })
            .collect())
    }

    /// Complete task
    pub fn complete_task(&mut self, task_id: &str) -> Result<(), TaskError> {
        let mut task = self.storage.load_task(task_id)?;
        task.status = TaskStatus::Done;
        task.completed_at = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        self.storage.save_task(&task)
    }

    /// Add subtask
    pub fn add_subtask(&mut self, task_id: &str, subtask_title: String) -> Result<(), TaskError> {
        let mut task = self.storage.load_task(task_id)?;
        let subtask_id = format!("subtask_{}", task.subtasks.len());
        task.subtasks.push(Subtask {
            id: subtask_id,
            title: subtask_title,
            completed: false,
        });
        self.storage.save_task(&task)
    }

    /// Toggle subtask
    pub fn toggle_subtask(&mut self, task_id: &str, subtask_id: &str) -> Result<(), TaskError> {
        let mut task = self.storage.load_task(task_id)?;
        if let Some(subtask) = task.subtasks.iter_mut().find(|s| s.id == subtask_id) {
            subtask.completed = !subtask.completed;
            self.storage.save_task(&task)
        } else {
            Err(TaskError::SubtaskNotFound(subtask_id.to_string()))
        }
    }

    /// Create project
    pub fn create_project(&mut self, name: String, description: String, color: String) -> String {
        let project_id = format!("project_{}", self.projects.len());
        let project = Project {
            id: project_id.clone(),
            name,
            description,
            color,
            task_ids: Vec::new(),
        };
        self.projects.insert(project_id.clone(), project);
        project_id
    }

    /// Add task to project
    pub fn add_task_to_project(
        &mut self,
        project_id: &str,
        task_id: &str,
    ) -> Result<(), TaskError> {
        if let Some(project) = self.projects.get_mut(project_id) {
            project.task_ids.push(task_id.to_string());
            Ok(())
        } else {
            Err(TaskError::ProjectNotFound(project_id.to_string()))
        }
    }

    /// Create Kanban board
    pub fn create_kanban_board(&mut self, name: String) -> String {
        let board_id = format!("board_{}", self.boards.len());
        let board = KanbanBoard {
            id: board_id.clone(),
            name,
            columns: vec![
                KanbanColumn {
                    id: "todo".to_string(),
                    name: "To Do".to_string(),
                    task_ids: Vec::new(),
                    wip_limit: None,
                },
                KanbanColumn {
                    id: "in_progress".to_string(),
                    name: "In Progress".to_string(),
                    task_ids: Vec::new(),
                    wip_limit: Some(3),
                },
                KanbanColumn {
                    id: "review".to_string(),
                    name: "Review".to_string(),
                    task_ids: Vec::new(),
                    wip_limit: None,
                },
                KanbanColumn {
                    id: "done".to_string(),
                    name: "Done".to_string(),
                    task_ids: Vec::new(),
                    wip_limit: None,
                },
            ],
        };
        self.boards.insert(board_id.clone(), board);
        board_id
    }

    /// Add task to Kanban column
    pub fn add_to_kanban(
        &mut self,
        board_id: &str,
        column_id: &str,
        task_id: &str,
    ) -> Result<(), TaskError> {
        if let Some(board) = self.boards.get_mut(board_id) {
            if let Some(column) = board.columns.iter_mut().find(|c| c.id == column_id) {
                column.task_ids.push(task_id.to_string());
                Ok(())
            } else {
                Err(TaskError::ColumnNotFound(column_id.to_string()))
            }
        } else {
            Err(TaskError::BoardNotFound(board_id.to_string()))
        }
    }

    /// Set reminder
    pub fn set_reminder(&mut self, task_id: &str, reminder_time: u64, reminder_type: ReminderType) {
        let reminder_id = format!("reminder_{}", self.reminders.len());
        self.reminders.push(Reminder {
            id: reminder_id,
            task_id: task_id.to_string(),
            reminder_time,
            reminder_type,
            is_dismissed: false,
        });
    }

    /// Check reminders
    pub fn check_reminders(&mut self) -> Vec<Reminder> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let due_reminders: Vec<Reminder> = self
            .reminders
            .iter()
            .filter(|r| !r.is_dismissed && r.reminder_time <= now)
            .cloned()
            .collect();

        due_reminders
    }

    /// Get projects
    pub fn projects(&self) -> Vec<&Project> {
        self.projects.values().collect()
    }

    /// Get boards
    pub fn boards(&self) -> Vec<&KanbanBoard> {
        self.boards.values().collect()
    }

    /// Auto-cleanup completed tasks
    pub fn auto_cleanup(&mut self) -> Result<usize, TaskError> {
        if !self.auto_cleanup_enabled {
            return Ok(0);
        }

        let mut tasks = self.storage.list_tasks()?;
        let mut cleaned = 0;

        tasks.retain(|task| {
            if task.status == TaskStatus::Done {
                if let Some(completed_at) = task.completed_at {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    // Archive tasks completed more than 30 days ago
                    if now - completed_at > 30 * 24 * 3600 {
                        cleaned += 1;
                        return false;
                    }
                }
            }
            true
        });

        // Save filtered tasks
        for task in &tasks {
            self.storage.save_task(task)?;
        }

        Ok(cleaned)
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new(Box::new(InMemoryStorage::new())).with_auto_cleanup(true)
    }
}

/// Task errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskError {
    TaskNotFound(String),
    ProjectNotFound(String),
    BoardNotFound(String),
    ColumnNotFound(String),
    SubtaskNotFound(String),
    StorageError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let task = Task {
            id: "test".to_string(),
            title: "Test Task".to_string(),
            description: "Test".to_string(),
            status: TaskStatus::Todo,
            priority: TaskPriority::Medium,
            due_date: None,
            created_at: 1234567890,
            completed_at: None,
            tags: Vec::new(),
            subtasks: Vec::new(),
        };
        assert_eq!(task.title, "Test Task");
    }

    #[test]
    fn test_in_memory_storage() {
        let storage = InMemoryStorage::new();
        assert_eq!(storage.name(), "InMemoryStorage");
    }

    #[test]
    fn test_task_manager() {
        let manager = TaskManager::default();
        assert!(manager.auto_cleanup_enabled);
    }

    #[test]
    fn test_create_task() {
        let mut manager = TaskManager::default();
        let task_id = manager
            .create_task(
                "Test".to_string(),
                "Description".to_string(),
                TaskPriority::Medium,
            )
            .unwrap();
        assert!(!task_id.is_empty());
    }

    #[test]
    fn test_complete_task() {
        let mut manager = TaskManager::default();
        let task_id = manager
            .create_task(
                "Test".to_string(),
                "Description".to_string(),
                TaskPriority::Medium,
            )
            .unwrap();
        manager.complete_task(&task_id).unwrap();
        let task = manager.get_task(&task_id).unwrap();
        assert_eq!(task.status, TaskStatus::Done);
    }
}
