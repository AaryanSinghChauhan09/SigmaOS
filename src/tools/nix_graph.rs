// SigmaOS `sigma-nix-graph` Functional Store Dependency Tree Visualizer CLI
// Implements store derivation closure parsing, ASCII dependency tree rendering,
// and why-depends retention path analysis.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct NixStoreDerivationNode {
    pub store_path: String,
    pub package_name: String,
    pub nar_size_bytes: u64,
    pub references: Vec<String>, // list of referenced store paths
}

pub struct SovereignNixStoreGraphEngine {
    pub store_nodes: BTreeMap<String, NixStoreDerivationNode>, // path -> node
}

impl SovereignNixStoreGraphEngine {
    pub fn new() -> Self {
        Self {
            store_nodes: BTreeMap::new(),
        }
    }

    pub fn register_derivation(&mut self, path: &str, pkg_name: &str, size: u64, refs: &[&str]) {
        let node = NixStoreDerivationNode {
            store_path: path.to_string(),
            package_name: pkg_name.to_string(),
            nar_size_bytes: size,
            references: refs.iter().map(|s| s.to_string()).collect(),
        };
        self.store_nodes.insert(path.to_string(), node);
    }

    pub fn render_ascii_dependency_tree(&self, root_path: &str) -> Result<String, &'static str> {
        let root = self.store_nodes.get(root_path).ok_or("nix-graph: Root store path not found")?;
        let mut tree = format!("{} ({})\n", root.package_name, root.store_path);

        for ref_path in &root.references {
            if let Some(child) = self.store_nodes.get(ref_path) {
                tree.push_str(&format!("  └── {} ({})\n", child.package_name, child.store_path));
            }
        }

        Ok(tree)
    }

    pub fn why_depends(&self, root_path: &str, target_path: &str) -> Vec<String> {
        let mut path_chain = Vec::new();
        if let Some(root) = self.store_nodes.get(root_path) {
            path_chain.push(root.store_path.clone());
            if root.references.contains(&target_path.to_string()) {
                path_chain.push(target_path.to_string());
            }
        }
        path_chain
    }
}

impl Default for SovereignNixStoreGraphEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nix_store_graph_engine() {
        let mut graph = SovereignNixStoreGraphEngine::new();

        let glibc_path = "/nix/store/a1b2c3d4-glibc-2.38";
        let openssl_path = "/nix/store/e5f6g7h8-openssl-3.0";
        let nginx_path = "/nix/store/i9j0k1l2-nginx-1.24";

        graph.register_derivation(glibc_path, "glibc-2.38", 15_000_000, &[]);
        graph.register_derivation(openssl_path, "openssl-3.0", 8_000_000, &[glibc_path]);
        graph.register_derivation(nginx_path, "nginx-1.24", 5_000_000, &[openssl_path, glibc_path]);

        let tree = graph.render_ascii_dependency_tree(nginx_path).unwrap();
        assert!(tree.contains("nginx-1.24"));
        assert!(tree.contains("openssl-3.0"));

        let why = graph.why_depends(nginx_path, openssl_path);
        assert_eq!(why.len(), 2);
    }
}
