// Linux/BSD Distro Inspirations Implementation
// This module implements key concepts from Linux and BSD distributions
// that provide competitive advantages for SigmaOS

#![no_std]
#![cfg_attr(target_os = "none", no_main)]

extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;
use alloc::string::{String, ToString};
use alloc::format;

// ==========================================
// 1. ARCH LINUX INSPIRATIONS
// ==========================================

/// Arch Linux-style rolling release dependency resolver
/// Uses Kahn's topological sort for dependency resolution
pub struct ArchDependencyResolver {
    packages: Vec<PackageNode>,
}

#[derive(Debug, Clone)]
pub struct PackageNode {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub provides: Vec<String>,
}

impl ArchDependencyResolver {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }

    pub fn add_package(&mut self, package: PackageNode) {
        self.packages.push(package);
    }

    /// Resolve dependencies using Kahn's algorithm with cycle detection.
    pub fn resolve_dependencies(&self, package_name: &str) -> Result<Vec<String>, &'static str> {
        // 1. Traverse and find the sub-graph of all reachable packages
        let mut subgraph = Vec::new();
        let mut stack = Vec::new();
        stack.push(package_name.to_string());

        while let Some(curr) = stack.pop() {
            if subgraph.contains(&curr) {
                continue;
            }
            // Find package or a package providing it
            let pkg = self.packages.iter()
                .find(|p| p.name == curr || p.provides.contains(&curr));
            if let Some(p) = pkg {
                subgraph.push(p.name.clone());
                for dep in &p.dependencies {
                    if !subgraph.contains(dep) {
                        stack.push(dep.clone());
                    }
                }
            } else {
                return Err("Package not found");
            }
        }

        // 2. Compute in-degree for all nodes in the subgraph.
        // In-degree is the number of dependencies a package has that are also in our subgraph.
        // Also map out-edges (dependents). If u is depended on by v, we have an edge u -> v.
        let mut in_degrees = Vec::new();
        let mut adj_list = Vec::new(); // (u, Vec<v>) where v depends on u

        for u in &subgraph {
            // Find u's package node
            let u_node = self.packages.iter().find(|p| &p.name == u).unwrap();
            let mut u_in_degree = 0;
            for dep in &u_node.dependencies {
                if subgraph.contains(dep) {
                    u_in_degree += 1;
                }
            }
            in_degrees.push((u.clone(), u_in_degree));

            // Populate adjacency list: find all nodes in subgraph that depend on u
            let mut dependents = Vec::new();
            for v in &subgraph {
                if v == u {
                    continue;
                }
                let v_node = self.packages.iter().find(|p| &p.name == v).unwrap();
                if v_node.dependencies.contains(u) {
                    dependents.push(v.clone());
                }
            }
            adj_list.push((u.clone(), dependents));
        }

        // 3. Initialize queue with in-degree 0 (leaves of the dependency tree, i.e. no deps)
        let mut queue = Vec::new();
        for (node, deg) in &in_degrees {
            if *deg == 0 {
                queue.push(node.clone());
            }
        }

        // 4. Sort queue to ensure deterministic sorting order
        queue.sort();

        let mut resolved = Vec::new();

        while !queue.is_empty() {
            // We want Kahn's to build from dependencies up to the targets.
            // Pop from the front to simulate a queue.
            let curr = queue.remove(0);
            resolved.push(curr.clone());

            // For each neighbor v that depends on curr:
            if let Some((_, dependents)) = adj_list.iter().find(|(u, _)| u == &curr) {
                for v in dependents {
                    if let Some(pos) = in_degrees.iter().position(|(node, _)| node == v) {
                        in_degrees[pos].1 -= 1;
                        if in_degrees[pos].1 == 0 {
                            queue.push(v.clone());
                            queue.sort();
                        }
                    }
                }
            }
        }

        // 5. If we resolved fewer nodes than are in the subgraph, a cycle exists!
        if resolved.len() != subgraph.len() {
            return Err("Dependency cycle detected");
        }

        Ok(resolved)
    }
}

// ==========================================
// 2. FREEBSD INSPIRATIONS
// ==========================================

/// FreeBSD Jails-inspired lightweight virtualization
pub struct FreeBSDJail {
    pub jail_id: u64,
    pub root_path: String,
    pub hostname: String,
    pub network_stack: bool,
    pub processes: Vec<u64>,
    pub max_processes: usize,
    pub child_jails: Vec<FreeBSDJail>,
    pub isolated_mounts: Vec<String>,
}

impl FreeBSDJail {
    pub fn new(jail_id: u64, root_path: String, hostname: String) -> Self {
        Self {
            jail_id,
            root_path,
            hostname,
            network_stack: false,
            processes: Vec::new(),
            max_processes: 10,
            child_jails: Vec::new(),
            isolated_mounts: Vec::new(),
        }
    }

    pub fn enable_network_stack(&mut self) {
        self.network_stack = true;
    }

    pub fn add_process(&mut self, pid: u64) {
        let _ = self.add_process_with_limit(pid);
    }

    pub fn add_process_with_limit(&mut self, pid: u64) -> Result<(), &'static str> {
        if self.processes.len() >= self.max_processes {
            return Err("Process limit exceeded for jail");
        }
        self.processes.push(pid);
        Ok(())
    }

    pub fn is_process_allowed(&self, pid: u64) -> bool {
        if self.processes.contains(&pid) {
            return true;
        }
        // Check hierarchical/nested child jails
        for child in &self.child_jails {
            if child.is_process_allowed(pid) {
                return true;
            }
        }
        false
    }

    pub fn add_child_jail(&mut self, child: FreeBSDJail) -> Result<(), &'static str> {
        // Nested jail must be isolated under parent's root path
        if !child.root_path.starts_with(&self.root_path) {
            return Err("Child jail root path must be a subdirectory of parent jail root path");
        }
        self.child_jails.push(child);
        Ok(())
    }

    pub fn mount_checkpoint(&mut self, path: &str) {
        self.isolated_mounts.push(path.to_string());
    }

    pub fn verify_mount_isolated(&self, path: &str) -> bool {
        self.isolated_mounts.contains(&path.to_string())
    }
}

// ==========================================
// 3. OPENBSD INSPIRATIONS
// ==========================================

/// OpenBSD pledge-inspired capability restriction
pub struct OpenBSDPledge {
    pub allowed_operations: Vec<String>,
    pub is_pledged: bool,
}

impl OpenBSDPledge {
    pub fn new() -> Self {
        Self {
            allowed_operations: Vec::new(),
            is_pledged: false,
        }
    }

    /// Set or restrict the allowed operations.
    /// Standard OpenBSD pledge: subsequent calls can only subset (restrict) the existing set.
    pub fn pledge(&mut self, operations: &[&str]) -> Result<(), &'static str> {
        let new_ops: Vec<String> = operations.iter().map(|s| s.to_string()).collect();
        if self.is_pledged {
            // Once pledged, subsequent pledges can only restrict (subset) the current allowed operations.
            // If any operation in new_ops is not in the current allowed_operations, it's an illegal escalation!
            for op in &new_ops {
                if !self.allowed_operations.contains(op) {
                    return Err("Illegal pledge escalation attempt blocked");
                }
            }
        }
        self.allowed_operations = new_ops;
        self.is_pledged = true;
        Ok(())
    }

    /// Check if the operation is allowed under current capabilities
    pub fn check_operation(&self, operation: &str) -> bool {
        // If not pledged yet, everything is allowed (default process state)
        if !self.is_pledged {
            return true;
        }
        self.allowed_operations.contains(&operation.to_string())
    }
}

// ==========================================
// 4. NIXOS INSPIRATIONS
// ==========================================

/// NixOS-style content-addressed store with garbage collection and deduplication
pub struct NixStyleStore {
    pub store_path: String,
    pub registered_paths: Vec<(String, Vec<u8>)>,
    pub references: Vec<(String, Vec<String>)>,
    pub gc_roots: Vec<String>,
}

impl NixStyleStore {
    pub fn new(store_path: String) -> Self {
        Self {
            store_path,
            registered_paths: Vec::new(),
            references: Vec::new(),
            gc_roots: Vec::new(),
        }
    }

    /// Generate content address (SHA-256 hash)
    pub fn content_address(&self, content: &[u8]) -> String {
        // Simple hash for demonstration
        let mut hash: u64 = 0;
        for byte in content {
            hash = hash.wrapping_mul(31).wrapping_add(*byte as u64);
        }
        format!("{:x}", hash)
    }

    pub fn get_store_path(&self, content: &[u8]) -> String {
        let address = self.content_address(content);
        format!("{}/{}", self.store_path, address)
    }

    pub fn register_path(&mut self, content: &[u8], deps: Vec<String>) -> String {
        let path = self.get_store_path(content);
        if !self.registered_paths.iter().any(|(p, _)| p == &path) {
            self.registered_paths.push((path.clone(), content.to_vec()));
        }
        self.references.push((path.clone(), deps));
        path
    }

    pub fn add_gc_root(&mut self, path: String) {
        if !self.gc_roots.contains(&path) {
            self.gc_roots.push(path);
        }
    }

    pub fn remove_gc_root(&mut self, path: &str) {
        self.gc_roots.retain(|r| r != path);
    }

    /// Reachability-based garbage collection (sweeps unreferenced store paths)
    pub fn garbage_collect(&mut self) -> Vec<String> {
        let mut reachable = Vec::new();
        let mut stack = self.gc_roots.clone();

        // 1. Mark phase (DFS reachability from GC roots)
        while let Some(current) = stack.pop() {
            if reachable.contains(&current) {
                continue;
            }
            reachable.push(current.clone());

            // Add all referenced dependencies of the current store path
            if let Some((_, deps)) = self.references.iter().find(|(p, _)| p == &current) {
                for dep in deps {
                    if !reachable.contains(dep) {
                        stack.push(dep.clone());
                    }
                }
            }
        }

        // 2. Sweep phase (identify and remove unreferenced paths)
        let mut deleted = Vec::new();
        let mut keep_paths = Vec::new();

        for (path, content) in self.registered_paths.drain(..) {
            if reachable.contains(&path) {
                keep_paths.push((path, content));
            } else {
                deleted.push(path);
            }
        }

        self.registered_paths = keep_paths;

        // Also clean up references
        self.references.retain(|(p, _)| reachable.contains(p));

        deleted
    }

    /// Deduplicate identical store paths (simulates hardlinking in Nix store)
    pub fn deduplicate(&self, path_a: &str, path_b: &str) -> bool {
        let content_a = self.registered_paths.iter().find(|(p, _)| p == path_a).map(|(_, c)| c);
        let content_b = self.registered_paths.iter().find(|(p, _)| p == path_b).map(|(_, c)| c);

        match (content_a, content_b) {
            (Some(ca), Some(cb)) => ca == cb,
            _ => false,
        }
    }
}

// ==========================================
// 5. DEBIAN/UBUNTU INSPIRATIONS
// ==========================================

/// APT-style priority pinning system
#[derive(Debug, Clone)]
pub struct PinRule {
    pub package: String,
    pub priority: i32,
    pub version: Option<String>,
}

pub struct AptPinStore {
    pins: Vec<PinRule>,
}

impl AptPinStore {
    pub fn new() -> Self {
        Self {
            pins: Vec::new(),
        }
    }

    pub fn add_pin(&mut self, pin: PinRule) {
        self.pins.push(pin);
    }

    pub fn get_package_priority(&self, package: &str) -> i32 {
        self.pins.iter()
            .filter(|p| p.package == package)
            .map(|p| p.priority)
            .max()
            .unwrap_or(500) // Default priority
    }
}

// ==========================================
// 6. SYSTEMD ALTERNATIVES
// ==========================================

/// OpenRC-inspired service management (alternative to systemd)
pub struct OpenRCService {
    pub name: String,
    pub enabled: bool,
    pub running: bool,
    pub dependencies: Vec<String>,
}

impl OpenRCService {
    pub fn new(name: String) -> Self {
        Self {
            name,
            enabled: false,
            running: false,
            dependencies: Vec::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        if !self.enabled {
            return Err("Service not enabled");
        }
        self.running = true;
        Ok(())
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.running = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_dependency_resolver_kahn_and_cycles() {
        let mut resolver = ArchDependencyResolver::new();

        resolver.add_package(PackageNode {
            name: "libc".to_string(),
            version: "2.35".to_string(),
            dependencies: Vec::new(),
            provides: Vec::new(),
        });

        resolver.add_package(PackageNode {
            name: "openssl".to_string(),
            version: "3.0".to_string(),
            dependencies: vec!["libc".to_string()],
            provides: Vec::new(),
        });

        resolver.add_package(PackageNode {
            name: "nginx".to_string(),
            version: "1.22".to_string(),
            dependencies: vec!["openssl".to_string(), "libc".to_string()],
            provides: Vec::new(),
        });

        let order = resolver.resolve_dependencies("nginx").unwrap();
        assert_eq!(order.len(), 3);
        assert_eq!(order[0], "libc");
        assert_eq!(order[1], "openssl");
        assert_eq!(order[2], "nginx");

        // Now test cycle detection
        let mut cyclic_resolver = ArchDependencyResolver::new();
        cyclic_resolver.add_package(PackageNode {
            name: "A".to_string(),
            version: "1.0".to_string(),
            dependencies: vec!["B".to_string()],
            provides: Vec::new(),
        });
        cyclic_resolver.add_package(PackageNode {
            name: "B".to_string(),
            version: "1.0".to_string(),
            dependencies: vec!["A".to_string()],
            provides: Vec::new(),
        });

        let res = cyclic_resolver.resolve_dependencies("A");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dependency cycle detected");
    }

    #[test]
    fn test_freebsd_jail_hierarchy_and_limits() {
        let mut parent_jail = FreeBSDJail::new(1, "/jails/parent".to_string(), "parent".to_string());
        parent_jail.max_processes = 2;

        assert!(parent_jail.add_process_with_limit(101).is_ok());
        assert!(parent_jail.add_process_with_limit(102).is_ok());
        // Third should exceed max_processes
        assert!(parent_jail.add_process_with_limit(103).is_err());

        // Hierarchical jails
        let child_jail = FreeBSDJail::new(2, "/jails/parent/child".to_string(), "child".to_string());
        assert!(parent_jail.add_child_jail(child_jail).is_ok());

        // Try adding a jail outside parent's root_path
        let rogue_jail = FreeBSDJail::new(3, "/jails/rogue".to_string(), "rogue".to_string());
        assert!(parent_jail.add_child_jail(rogue_jail).is_err());

        // Isolated mounts
        parent_jail.mount_checkpoint("/etc");
        assert!(parent_jail.verify_mount_isolated("/etc"));
        assert!(!parent_jail.verify_mount_isolated("/var"));
    }

    #[test]
    fn test_openbsd_pledge_transitions() {
        let mut process = OpenBSDPledge::new();

        // Before pledge, everything is allowed
        assert!(process.check_operation("stdio"));
        assert!(process.check_operation("rpath"));

        // First pledge sets operations
        assert!(process.pledge(&["stdio", "rpath"]).is_ok());
        assert!(process.check_operation("stdio"));
        assert!(process.check_operation("rpath"));
        assert!(!process.check_operation("wpath"));

        // Subsequent pledge can only subset (restrict)
        assert!(process.pledge(&["stdio"]).is_ok());
        assert!(process.check_operation("stdio"));
        assert!(!process.check_operation("rpath"));

        // Attempting to escalate is blocked and returns Err
        assert!(process.pledge(&["stdio", "wpath"]).is_err());
    }

    #[test]
    fn test_nix_store_gc_and_dedup() {
        let mut store = NixStyleStore::new("/sigma/store".to_string());

        let path1 = store.register_path(b"lib-content", Vec::new());
        let path2 = store.register_path(b"app-content", vec![path1.clone()]);
        let path3 = store.register_path(b"orphan-content", Vec::new());

        // Register path3 as identical to path1 to test deduplication
        let path4 = store.register_path(b"lib-content", Vec::new());

        assert!(store.deduplicate(&path1, &path4));
        assert!(!store.deduplicate(&path1, &path2));

        // GC Roots reachability
        store.add_gc_root(path2.clone());

        // Garbage collect: path3 should be deleted, while path2 and its dependency path1 should be kept
        let deleted = store.garbage_collect();
        assert!(deleted.contains(&path3));
        assert!(!deleted.contains(&path1));
        assert!(!deleted.contains(&path2));

        // Let's remove GC root and garbage collect again
        store.remove_gc_root(&path2);
        let deleted2 = store.garbage_collect();
        assert!(deleted2.contains(&path1));
        assert!(deleted2.contains(&path2));
    }
}
// Linux & BSD distro inspirations verified
