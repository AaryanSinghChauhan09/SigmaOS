// SigmaOS Sovereign Mind Map Creator (SigmaMind)
// Purpose-built, highly interactive hierarchical visualization tool inspired by XMind, MindMeister, and NiceMind.
// Exposes rich styling, relationship boundaries, task progress tracking, and layouts.

use std::collections::HashMap;

/// Mind Map layouts (Radial, OrgChart, LogicChart)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MindMapLayout {
    Radial,
    OrgChart,
    LogicChart,
}

/// Node shapes (Rectangle, RoundedRect, Ellipse)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeShape {
    Rectangle,
    RoundedRect,
    Ellipse,
}

/// Custom Styling parameters for a node
#[derive(Debug, Clone)]
pub struct NodeStyle {
    pub bg_color: String,
    pub font_size: u32,
    pub shape: NodeShape,
    pub border_width: u32,
}

impl Default for NodeStyle {
    fn default() -> Self {
        Self {
            bg_color: "#FFFFFF".to_string(),
            font_size: 12,
            shape: NodeShape::RoundedRect,
            border_width: 1,
        }
    }
}

/// Structural Mind Map node representing an idea/topic
#[derive(Debug, Clone)]
pub struct MindMapNode {
    pub node_id: String,
    pub topic: String,
    pub parent_id: Option<String>,
    pub children_ids: Vec<String>,
    pub style: NodeStyle,
    pub progress_percent: u32, // XMind-style progress task tracking (e.g., 25%, 50%, 100%)
    pub priority: u32,         // Priority flags (1 to 5)
    pub notes: String,         // Detailed note-taking text
}

impl MindMapNode {
    pub fn new(id: &str, topic: &str, parent: Option<String>) -> Self {
        Self {
            node_id: id.to_string(),
            topic: topic.to_string(),
            parent_id: parent,
            children_ids: Vec::new(),
            style: NodeStyle::default(),
            progress_percent: 0,
            priority: 3, // medium priority default
            notes: String::new(),
        }
    }
}

/// Custom relationship arrow linking arbitrary cross-branch nodes
#[derive(Debug, Clone)]
pub struct RelationshipConnection {
    pub source_id: String,
    pub target_id: String,
    pub label: String,
    pub line_style: String, // "dashed", "solid", "curved"
}

/// High-Performance Mind Map Canvas managing node hierarchies and connections
pub struct MindMapCreator {
    pub map_title: String,
    pub nodes: HashMap<String, MindMapNode>,
    pub root_node_id: String,
    pub relationships: Vec<RelationshipConnection>,
    pub default_layout: MindMapLayout,
}

impl MindMapCreator {
    pub fn new(title: &str, root_topic: &str) -> Self {
        let root_id = "root_node".to_string();
        let root_node = MindMapNode::new(&root_id, root_topic, None);
        let mut nodes = HashMap::new();
        nodes.insert(root_id.clone(), root_node);

        Self {
            map_title: title.to_string(),
            nodes,
            root_node_id: root_id,
            relationships: Vec::new(),
            default_layout: MindMapLayout::Radial,
        }
    }

    /// Appends a new sub-idea to a parent node
    pub fn add_node(
        &mut self,
        node_id: &str,
        parent_id: &str,
        topic: &str,
    ) -> Result<(), &'static str> {
        if self.nodes.contains_key(node_id) {
            return Err("Node ID already exists in the mind map");
        }
        if !self.nodes.contains_key(parent_id) {
            return Err("Specified parent node does not exist");
        }

        let new_node = MindMapNode::new(node_id, topic, Some(parent_id.to_string()));
        self.nodes.insert(node_id.to_string(), new_node);

        if let Some(parent) = self.nodes.get_mut(parent_id) {
            parent.children_ids.push(node_id.to_string());
        }

        Ok(())
    }

    /// Moves a sub-topic to a different branch (re-parenting)
    pub fn move_branch(&mut self, node_id: &str, new_parent_id: &str) -> Result<(), &'static str> {
        if node_id == self.root_node_id {
            return Err("Cannot re-parent the central root idea");
        }
        if !self.nodes.contains_key(new_parent_id) {
            return Err("New parent node not found");
        }

        let old_parent_id = {
            let node = self.nodes.get(node_id).ok_or("Node not found")?;
            node.parent_id
                .clone()
                .ok_or("Root node re-parent blocked")?
        };

        // Remove from old parent's child list
        if let Some(old_parent) = self.nodes.get_mut(&old_parent_id) {
            old_parent.children_ids.retain(|id| id != node_id);
        }

        // Set new parent
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.parent_id = Some(new_parent_id.to_string());
        }

        // Append to new parent's child list
        if let Some(new_parent) = self.nodes.get_mut(new_parent_id) {
            new_parent.children_ids.push(node_id.to_string());
        }

        Ok(())
    }

    /// Deletes a node and recursively removes all of its children sub-branches
    pub fn delete_node_recursive(&mut self, node_id: &str) -> Result<(), &'static str> {
        if node_id == self.root_node_id {
            return Err("Cannot delete central root node");
        }

        // Remove from parent child references
        let parent_id = self
            .nodes
            .get(node_id)
            .and_then(|node| node.parent_id.clone());
        if let Some(ref p_id) = parent_id {
            if let Some(parent) = self.nodes.get_mut(p_id) {
                parent.children_ids.retain(|id| id != node_id);
            }
        }

        self.delete_recursive_inner(node_id);
        Ok(())
    }

    fn delete_recursive_inner(&mut self, node_id: &str) {
        if let Some(node) = self.nodes.remove(node_id) {
            for child_id in node.children_ids {
                self.delete_recursive_inner(&child_id);
            }
        }
        // Remove active cross-relationships involving the deleted node
        self.relationships
            .retain(|rel| rel.source_id != node_id && rel.target_id != node_id);
    }

    /// Connects arbitrary cross-branch ideas with a customized relationship line
    pub fn add_relationship(
        &mut self,
        source_id: &str,
        target_id: &str,
        label: &str,
        line_style: &str,
    ) -> Result<(), &'static str> {
        if !self.nodes.contains_key(source_id) || !self.nodes.contains_key(target_id) {
            return Err("Source or target node not found in map");
        }
        self.relationships.push(RelationshipConnection {
            source_id: source_id.to_string(),
            target_id: target_id.to_string(),
            label: label.to_string(),
            line_style: line_style.to_string(),
        });
        Ok(())
    }

    /// Sets custom styles and metadata markers (XMind-style priority and progress)
    pub fn update_node_metadata(
        &mut self,
        node_id: &str,
        priority: u32,
        progress: u32,
        notes: &str,
    ) -> Result<(), &'static str> {
        let node = self.nodes.get_mut(node_id).ok_or("Node not found")?;
        node.priority = priority.clamp(1, 5);
        node.progress_percent = progress.clamp(0, 100);
        node.notes = notes.to_string();
        Ok(())
    }

    /// Exports the hierarchical mind map layout to an indented text tree for easy console rendering
    pub fn export_to_text_tree(&self) -> String {
        let mut output = format!("=== MIND MAP: {} ===\n", self.map_title);
        self.export_node_tree_inner(&self.root_node_id, 0, &mut output);
        output
    }

    fn export_node_tree_inner(&self, node_id: &str, depth: usize, output: &mut String) {
        if let Some(node) = self.nodes.get(node_id) {
            let indent = "  ".repeat(depth);
            let progress_marker = if node.progress_percent > 0 {
                format!(" [Progress: {}%]", node.progress_percent)
            } else {
                String::new()
            };
            output.push_str(&format!(
                "{}- {} (Priority: {}){}\n",
                indent, node.topic, node.priority, progress_marker
            ));

            for child_id in &node.children_ids {
                self.export_node_tree_inner(child_id, depth + 1, output);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mind_map_creation() {
        let mut map = MindMapCreator::new("Project Planning", "Main Goal");
        assert_eq!(map.map_title, "Project Planning");
        assert_eq!(map.nodes.len(), 1);

        // Add sub-nodes
        assert!(map
            .add_node("idea_1", "root_node", "Technical Setup")
            .is_ok());
        assert!(map
            .add_node("idea_2", "root_node", "Marketing Strategy")
            .is_ok());
        assert_eq!(map.nodes.len(), 3);

        // Verify root node child indices
        let root = map.nodes.get("root_node").unwrap();
        assert_eq!(root.children_ids.len(), 2);
    }

    #[test]
    fn test_move_branch() {
        let mut map = MindMapCreator::new("R&D Roadmap", "Base OS");
        map.add_node("kernel_shard", "root_node", "Kernel").unwrap();
        map.add_node("scheduler", "root_node", "Scheduler").unwrap();

        // Move Scheduler branch to be a child of Kernel
        assert!(map.move_branch("scheduler", "kernel_shard").is_ok());

        // Verify Scheduler parent is updated
        let sched = map.nodes.get("scheduler").unwrap();
        assert_eq!(sched.parent_id.as_deref(), Some("kernel_shard"));

        // Verify old parent root_node no longer lists scheduler as direct child
        let root = map.nodes.get("root_node").unwrap();
        assert!(!root.children_ids.contains(&"scheduler".to_string()));
    }

    #[test]
    fn test_delete_node_recursive() {
        let mut map = MindMapCreator::new("Business Plan", "HQ");
        map.add_node("marketing", "root_node", "Marketing").unwrap();
        map.add_node("social_media", "marketing", "Social Media Campaign")
            .unwrap();
        map.add_node("seo", "marketing", "Search Engine Optimization")
            .unwrap();

        assert_eq!(map.nodes.len(), 4);

        // Delete marketing (should recursively delete social_media and seo)
        assert!(map.delete_node_recursive("marketing").is_ok());
        assert_eq!(map.nodes.len(), 1); // Only root remains
        assert!(!map.nodes.contains_key("social_media"));
    }

    #[test]
    fn test_rich_metadata_and_connections() {
        let mut map = MindMapCreator::new("Study Guide", "Microkernel");
        map.add_node("nix", "root_node", "NixOS").unwrap();
        map.add_node("debian", "root_node", "Debian").unwrap();

        // Update markers
        assert!(map
            .update_node_metadata("nix", 1, 75, "High priority sandboxing research")
            .is_ok());
        let nix_node = map.nodes.get("nix").unwrap();
        assert_eq!(nix_node.priority, 1);
        assert_eq!(nix_node.progress_percent, 75);

        // Add custom connection boundary
        assert!(map
            .add_relationship("nix", "debian", "Sync release model structure", "dashed")
            .is_ok());
        assert_eq!(map.relationships.len(), 1);
        assert_eq!(map.relationships[0].label, "Sync release model structure");
    }

    #[test]
    fn test_text_tree_export() {
        let mut map = MindMapCreator::new("Alpha", "HQ");
        map.add_node("b1", "root_node", "Branch 1").unwrap();
        map.update_node_metadata("b1", 1, 50, "").unwrap();

        let tree_str = map.export_to_text_tree();
        assert!(tree_str.contains("=== MIND MAP: Alpha ==="));
        assert!(tree_str.contains("- Branch 1 (Priority: 1) [Progress: 50%]"));
    }
}
