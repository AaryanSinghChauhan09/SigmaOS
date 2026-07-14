pub mod service;
pub mod dag;

pub use service::{Service, ServiceState, ServiceManager};
pub use dag::DependencyGraph;

/// The SigmaOS native Init System.
/// Replaces systemd by providing declarative service definitions in Rust,
/// deterministic dependency DAG resolution, and socket activation.
pub struct SigmaInit {
    pub service_manager: ServiceManager,
    pub dependency_graph: DependencyGraph,
}

impl Default for SigmaInit {
    fn default() -> Self {
        Self::new()
    }
}

impl SigmaInit {
    pub fn new() -> Self {
        Self {
            service_manager: ServiceManager::new(),
            dependency_graph: DependencyGraph::new(),
        }
    }

    /// Register a new service.
    pub fn register(&mut self, service: Service) {
        let name = service.name.clone();
        for dep in &service.requires {
            self.dependency_graph.add_edge(dep.clone(), name.clone());
        }
        self.service_manager.add_service(service);
    }

    /// Start the system by resolving the DAG and launching services in order.
    pub fn boot(&mut self) -> Result<(), String> {
        let order = self.dependency_graph.resolve_order()?;
        for service_name in order {
            self.service_manager.start_service(&service_name)?;
        }
        Ok(())
    }
}
