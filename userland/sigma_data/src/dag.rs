/// Sovereign DAG engine — displaces Apache Airflow and Streamlit execution graphs.
/// Zero external dependencies.
#[derive(Debug, Clone)]
pub struct DagNode {
    pub id: u64,
    pub label: String,
    pub deps: Vec<u64>,
}

#[derive(Debug, Default)]
pub struct Dag {
    pub nodes: Vec<DagNode>,
}

impl Dag {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, id: u64, label: &str, deps: Vec<u64>) {
        self.nodes.push(DagNode { id, label: label.to_string(), deps });
    }

    /// Topological sort via iterative DFS — no recursion limit issues, no stdlib collections beyond Vec.
    pub fn topological_order(&self) -> Result<Vec<u64>, String> {
        let mut in_degree: Vec<(u64, usize)> = self.nodes.iter().map(|n| (n.id, n.deps.len())).collect();
        let mut result: Vec<u64> = Vec::new();
        let mut changed = true;

        while changed {
            changed = false;
            for i in 0..in_degree.len() {
                if in_degree[i].1 == 0 {
                    let node_id = in_degree[i].0;
                    result.push(node_id);
                    in_degree[i].1 = usize::MAX; // mark visited

                    // Reduce degree of dependents
                    for n in &self.nodes {
                        for dep in &n.deps {
                            if *dep == node_id {
                                for j in 0..in_degree.len() {
                                    if in_degree[j].0 == n.id && in_degree[j].1 != usize::MAX {
                                        in_degree[j].1 -= 1;
                                    }
                                }
                            }
                        }
                    }
                    changed = true;
                    break;
                }
            }
        }

        if result.len() == self.nodes.len() {
            Ok(result)
        } else {
            Err("Cycle detected in DAG".to_string())
        }
    }
}
