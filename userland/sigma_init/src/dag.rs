use std::collections::{HashMap, HashSet};

/// Directed Acyclic Graph for resolving service dependencies.
pub struct DependencyGraph {
    edges: HashMap<String, Vec<String>>,
    nodes: HashSet<String>,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            nodes: HashSet::new(),
        }
    }

    /// Add a dependency edge: `from` must start before `to`.
    pub fn add_edge(&mut self, from: String, to: String) {
        self.nodes.insert(from.clone());
        self.nodes.insert(to.clone());
        self.edges.entry(from).or_insert_with(Vec::new).push(to);
    }

    /// Perform a topological sort to resolve the start order.
    pub fn resolve_order(&self) -> Result<Vec<String>, String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        for node in &self.nodes {
            in_degree.insert(node.clone(), 0);
        }

        for targets in self.edges.values() {
            for target in targets {
                *in_degree.entry(target.clone()).or_insert(0) += 1;
            }
        }

        let mut queue: Vec<String> = in_degree
            .iter()
            .filter(|&(_, &deg)| deg == 0)
            .map(|(node, _)| node.clone())
            .collect();

        let mut order = Vec::new();

        while let Some(node) = queue.pop() {
            order.push(node.clone());

            if let Some(targets) = self.edges.get(&node) {
                for target in targets {
                    let deg = in_degree.get_mut(target).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push(target.clone());
                    }
                }
            }
        }

        if order.len() != self.nodes.len() {
            return Err("Dependency cycle detected in service DAG".to_string());
        }

        Ok(order)
    }
}
