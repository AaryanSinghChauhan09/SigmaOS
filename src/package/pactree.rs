// SigmaOS Pactree Engine
// Implements Arch Linux's pactree functionality
// Renders ASCII dependency trees for installed packages and recipes

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Dependency tree node
#[derive(Debug, Clone)]
pub struct DependencyNode {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub level: usize,
}

/// Pactree engine
pub struct PactreeEngine {
    pub package_db: BTreeMap<String, DependencyNode>,
}

impl PactreeEngine {
    pub fn new() -> Self {
        Self {
            package_db: BTreeMap::new(),
        }
    }

    /// Add package to database
    pub fn add_package(&mut self, node: DependencyNode) {
        self.package_db.insert(node.name.clone(), node);
    }

    /// Generate dependency tree
    pub fn generate_tree(&self, package_name: &str) -> Option<String> {
        if !self.package_db.contains_key(package_name) {
            return None;
        }

        let mut tree = String::new();
        self._build_tree(package_name, 0, &mut tree, "");
        Some(tree)
    }

    /// Build tree recursively
    fn _build_tree(&self, package_name: &str, level: usize, tree: &mut String, prefix: &str) {
        if let Some(node) = self.package_db.get(package_name) {
            tree.push_str(prefix);
            tree.push_str(&node.name);
            tree.push_str(" ");
            tree.push_str(&node.version);
            tree.push('\n');

            let new_prefix = format!("{}  ", prefix);
            for (i, dep) in node.dependencies.iter().enumerate() {
                let is_last = i == node.dependencies.len() - 1;
                let connector = if is_last { "└─" } else { "├─" };
                let next_prefix = if is_last {
                    format!("{}    ", prefix)
                } else {
                    format!("{}│   ", prefix)
                };

                tree.push_str(&new_prefix);
                tree.push_str(connector);

                if let Some(dep_node) = self.package_db.get(dep) {
                    tree.push_str(&dep_node.name);
                    tree.push('\n');
                    self._build_tree(dep, level + 1, tree, &next_prefix);
                } else {
                    tree.push_str(dep);
                    tree.push('\n');
                }
            }
        }
    }

    /// Generate reverse dependency tree
    pub fn generate_reverse_tree(&self, package_name: &str) -> Option<String> {
        let mut dependents = Vec::new();

        for (name, node) in &self.package_db {
            if node.dependencies.contains(&package_name.to_string()) {
                dependents.push(name.clone());
            }
        }

        if dependents.is_empty() {
            return None;
        }

        let mut tree = String::new();
        tree.push_str(package_name);
        tree.push_str(" is required by:\n");

        for (i, dep) in dependents.iter().enumerate() {
            let prefix = if i == dependents.len() - 1 {
                "└─"
            } else {
                "├─"
            };
            tree.push_str(prefix);
            tree.push_str(dep);
            tree.push('\n');
        }

        Some(tree)
    }

    /// Get dependency statistics
    pub fn get_dependency_stats(&self, package_name: &str) -> Option<String> {
        if let Some(node) = self.package_db.get(package_name) {
            let direct_deps = node.dependencies.len();
            let total_deps = self._count_total_deps(package_name);

            Some(format!(
                "{} dependencies for {}:\nDirect: {}\nTotal (including transitive): {}",
                package_name, package_name, direct_deps, total_deps
            ))
        } else {
            None
        }
    }

    /// Count total dependencies recursively
    fn _count_total_deps(&self, package_name: &str) -> usize {
        if let Some(node) = self.package_db.get(package_name) {
            let mut count = node.dependencies.len();
            for dep in &node.dependencies {
                count += self._count_total_deps(dep);
            }
            count
        } else {
            0
        }
    }
}

impl Default for PactreeEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_pactree() {
        let mut engine = PactreeEngine::new();

        let node = DependencyNode {
            name: "example-pkg".to_string(),
            version: "1.0.0".to_string(),
            dependencies: vec!["dep1".to_string(), "dep2".to_string()],
            level: 0,
        };

        engine.add_package(node);
        let tree = engine.generate_tree("example-pkg");
        assert!(tree.is_some());
    }

    #[test]
    fn test_dependency_stats() {
        let mut engine = PactreeEngine::new();

        let node = DependencyNode {
            name: "example-pkg".to_string(),
            version: "1.0.0".to_string(),
            dependencies: vec!["dep1".to_string()],
            level: 0,
        };

        engine.add_package(node);
        let stats = engine.get_dependency_stats("example-pkg");
        assert!(stats.is_some());
    }
}
