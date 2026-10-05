// SigmaOS Desktop DesktopWorkspace Manager
// Inspired by Linux Mint's workspace management and Omarchy's workspace utilities

use std::collections::HashMap;

/// DesktopWorkspace type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopWorkspaceType {
    Normal,
    Special,
    Scratchpad,
}

impl DesktopWorkspaceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopWorkspaceType::Normal => "Normal",
            DesktopWorkspaceType::Special => "Special",
            DesktopWorkspaceType::Scratchpad => "Scratchpad",
        }
    }
}

/// DesktopWorkspace
#[derive(Debug, Clone)]
pub struct DesktopWorkspace {
    pub id: String,
    pub name: String,
    pub workspace_type: DesktopWorkspaceType,
    pub is_active: bool,
    pub is_visible: bool,
    pub window_count: u32,
}

impl DesktopWorkspace {
    pub fn new(id: String, name: String, workspace_type: DesktopWorkspaceType) -> Self {
        DesktopWorkspace {
            id,
            name,
            workspace_type,
            is_active: false,
            is_visible: true,
            window_count: 0,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    pub fn set_window_count(&mut self, count: u32) {
        self.window_count = count;
    }
}

/// DesktopWorkspace Manager
pub struct DesktopWorkspaceManager {
    workspaces: HashMap<String, DesktopWorkspace>,
    active_workspace: Option<String>,
    num_workspaces: u32,
    next_workspace_id: u32,
}

impl DesktopWorkspaceManager {
    pub fn new() -> Self {
        let mut manager = DesktopWorkspaceManager {
            workspaces: HashMap::new(),
            active_workspace: None,
            num_workspaces: 4,
            next_workspace_id: 1,
        };

        // Add default workspaces
        manager.add_default_workspaces();

        manager
    }

    fn add_default_workspaces(&mut self) {
        for i in 1..=self.num_workspaces {
            let mut workspace = DesktopWorkspace::new(
                format!("ws_{}", self.next_workspace_id),
                format!("DesktopWorkspace {}", i),
                DesktopWorkspaceType::Normal,
            );
            if i == 1 {
                workspace.set_active(true);
                self.active_workspace = Some(workspace.id.clone());
            }
            self.workspaces.insert(workspace.id.clone(), workspace);
            self.next_workspace_id += 1;
        }
    }

    pub fn add_workspace(&mut self, name: String, workspace_type: DesktopWorkspaceType) -> String {
        let id = format!("ws_{}", self.next_workspace_id);
        let workspace = DesktopWorkspace::new(id.clone(), name, workspace_type);
        self.workspaces.insert(id.clone(), workspace);
        self.next_workspace_id += 1;
        self.num_workspaces += 1;
        id
    }

    pub fn remove_workspace(&mut self, id: &str) -> bool {
        if let Some(workspace) = self.workspaces.get(id) {
            if workspace.is_active {
                return false; // Cannot remove active workspace
            }
            if workspace.workspace_type == DesktopWorkspaceType::Normal && self.num_workspaces <= 1 {
                return false; // Must have at least one normal workspace
            }
        }

        let removed = self.workspaces.remove(id).is_some();

        if removed {
            self.num_workspaces -= 1;
        }

        removed
    }

    pub fn get_workspace(&self, id: &str) -> Option<&DesktopWorkspace> {
        self.workspaces.get(id)
    }

    pub fn get_workspaces(&self) -> Vec<&DesktopWorkspace> {
        self.workspaces.values().collect()
    }

    pub fn get_visible_workspaces(&self) -> Vec<&DesktopWorkspace> {
        self.workspaces
            .values()
            .filter(|w| w.is_visible)
            .collect()
    }

    pub fn get_workspaces_by_type(&self, workspace_type: DesktopWorkspaceType) -> Vec<&DesktopWorkspace> {
        self.workspaces
            .values()
            .filter(|w| w.workspace_type == workspace_type)
            .collect()
    }

    pub fn get_active_workspace(&self) -> Option<&DesktopWorkspace> {
        if let Some(active_id) = &self.active_workspace {
            self.workspaces.get(active_id)
        } else {
            None
        }
    }

    pub fn set_active_workspace(&mut self, id: &str) -> bool {
        if !self.workspaces.contains_key(id) {
            return false;
        }

        // Deactivate current active workspace
        if let Some(current_id) = &self.active_workspace {
            if let Some(workspace) = self.workspaces.get_mut(current_id) {
                workspace.set_active(false);
            }
        }

        // Activate new workspace
        if let Some(workspace) = self.workspaces.get_mut(id) {
            workspace.set_active(true);
            self.active_workspace = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn switch_to_next(&mut self) -> bool {
        let workspace_ids: Vec<String> = self.workspaces.keys().cloned().collect();
        if workspace_ids.is_empty() {
            return false;
        }

        if let Some(current_id) = &self.active_workspace {
            if let Some(pos) = workspace_ids.iter().position(|id| id == current_id) {
                let next_pos = (pos + 1) % workspace_ids.len();
                self.set_active_workspace(&workspace_ids[next_pos])
            } else {
                false
            }
        } else {
            self.set_active_workspace(&workspace_ids[0])
        }
    }

    pub fn switch_to_previous(&mut self) -> bool {
        let workspace_ids: Vec<String> = self.workspaces.keys().cloned().collect();
        if workspace_ids.is_empty() {
            return false;
        }

        if let Some(current_id) = &self.active_workspace {
            if let Some(pos) = workspace_ids.iter().position(|id| id == current_id) {
                let prev_pos = if pos == 0 {
                    workspace_ids.len() - 1
                } else {
                    pos - 1
                };
                self.set_active_workspace(&workspace_ids[prev_pos])
            } else {
                false
            }
        } else {
            self.set_active_workspace(&workspace_ids[0])
        }
    }

    pub fn switch_to_index(&mut self, index: usize) -> bool {
        let workspace_ids: Vec<String> = self.workspaces.keys().cloned().collect();
        if index < workspace_ids.len() {
            self.set_active_workspace(&workspace_ids[index])
        } else {
            false
        }
    }

    pub fn set_workspace_visible(&mut self, id: &str, visible: bool) -> bool {
        if let Some(workspace) = self.workspaces.get_mut(id) {
            workspace.set_visible(visible);
            true
        } else {
            false
        }
    }

    pub fn set_workspace_window_count(&mut self, id: &str, count: u32) -> bool {
        if let Some(workspace) = self.workspaces.get_mut(id) {
            workspace.set_window_count(count);
            true
        } else {
            false
        }
    }

    pub fn set_num_workspaces(&mut self, num: u32) -> bool {
        if num < 1 {
            return false;
        }

        let current_normal = self.get_workspaces_by_type(DesktopWorkspaceType::Normal).len() as u32;

        if num > current_normal {
            // Add workspaces
            for i in (current_normal + 1)..=num {
                let id = format!("ws_{}", self.next_workspace_id);
                let workspace = DesktopWorkspace::new(
                    id.clone(),
                    format!("DesktopWorkspace {}", i),
                    DesktopWorkspaceType::Normal,
                );
                self.workspaces.insert(id.clone(), workspace);
                self.next_workspace_id += 1;
            }
        } else if num < current_normal {
            // Remove workspaces (but not active)
            let mut to_remove: Vec<String> = Vec::new();
            for workspace in self.workspaces.values() {
                if workspace.workspace_type == DesktopWorkspaceType::Normal && !workspace.is_active {
                    to_remove.push(workspace.id.clone());
                }
            }

            while to_remove.len() as u32 > current_normal - num {
                if let Some(id) = to_remove.pop() {
                    self.remove_workspace(&id);
                }
            }
        }

        self.num_workspaces = num;
        true
    }

    pub fn get_statistics(&self) -> DesktopWorkspaceStatistics {
        DesktopWorkspaceStatistics {
            total_workspaces: self.workspaces.len(),
            active_workspace: self.active_workspace.clone(),
            normal_workspaces: self.get_workspaces_by_type(DesktopWorkspaceType::Normal).len(),
            visible_workspaces: self.get_visible_workspaces().len(),
        }
    }
}

impl Default for DesktopWorkspaceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// DesktopWorkspace statistics
#[derive(Debug, Clone)]
pub struct DesktopWorkspaceStatistics {
    pub total_workspaces: usize,
    pub active_workspace: Option<String>,
    pub normal_workspaces: usize,
    pub visible_workspaces: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_manager_initialization() {
        let manager = DesktopWorkspaceManager::new();
        assert_eq!(manager.get_workspaces().len(), 4);
        assert!(manager.get_active_workspace().is_some());
    }

    #[test]
    fn test_add_workspace() {
        let mut manager = DesktopWorkspaceManager::new();
        let id = manager.add_workspace("Scratchpad".to_string(), DesktopWorkspaceType::Scratchpad);
        assert!(manager.get_workspace(&id).is_some());
        assert_eq!(manager.get_workspaces().len(), 5);
    }

    #[test]
    fn test_remove_workspace() {
        let mut manager = DesktopWorkspaceManager::new();
        let id = manager.add_workspace("Scratchpad".to_string(), DesktopWorkspaceType::Scratchpad);
        assert!(manager.remove_workspace(&id));
        assert!(!manager.get_workspace(&id).is_some());
        assert_eq!(manager.get_workspaces().len(), 4);
    }

    #[test]
    fn test_remove_active_workspace() {
        let mut manager = DesktopWorkspaceManager::new();
        let active_id = manager.get_active_workspace().unwrap().id.clone();
        assert!(!manager.remove_workspace(&active_id));
    }

    #[test]
    fn test_set_active_workspace() {
        let mut manager = DesktopWorkspaceManager::new();
        let id = manager.add_workspace("Scratchpad".to_string(), DesktopWorkspaceType::Scratchpad);
        assert!(manager.set_active_workspace(&id));
        assert_eq!(manager.get_active_workspace().unwrap().id, id);
    }

    #[test]
    fn test_switch_to_next() {
        let mut manager = DesktopWorkspaceManager::new();
        let current_id = manager.get_active_workspace().unwrap().id.clone();
        assert!(manager.switch_to_next());
        assert_ne!(manager.get_active_workspace().unwrap().id, current_id);
    }

    #[test]
    fn test_switch_to_previous() {
        let mut manager = DesktopWorkspaceManager::new();
        manager.switch_to_next();
        let current_id = manager.get_active_workspace().unwrap().id.clone();
        assert!(manager.switch_to_previous());
        assert_ne!(manager.get_active_workspace().unwrap().id, current_id);
    }

    #[test]
    fn test_switch_to_index() {
        let mut manager = DesktopWorkspaceManager::new();
        assert!(manager.switch_to_index(2));
    }

    #[test]
    fn test_set_num_workspaces() {
        let mut manager = DesktopWorkspaceManager::new();
        assert!(manager.set_num_workspaces(6));
        assert_eq!(manager.get_workspaces().len(), 6);
    }

    #[test]
    fn test_statistics() {
        let manager = DesktopWorkspaceManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.total_workspaces, 4);
        assert_eq!(stats.normal_workspaces, 4);
    }
}
