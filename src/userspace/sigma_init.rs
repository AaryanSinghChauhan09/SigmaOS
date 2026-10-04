extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartPolicy {
    Always,
    OnFailure,
    Never,
}

#[derive(Debug, Clone)]
pub struct ServiceUnit {
    pub name: String,
    pub description: String,
    pub exec_start: String,
    pub dependencies: Vec<String>,
    pub restart_policy: RestartPolicy,
}

pub struct InitSystem {
    services: BTreeMap<String, ServiceUnit>,
    running: Vec<String>,
}

impl InitSystem {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            running: Vec::new(),
        }
    }

    pub fn add_service(&mut self, service: ServiceUnit) {
        self.services.insert(service.name.clone(), service);
    }

    pub fn topological_sort(&self) -> Result<Vec<String>, &'static str> {
        let mut sorted = Vec::new();
        let mut visited = BTreeMap::new();
        
        for name in self.services.keys() {
            visited.insert(name.clone(), false);
        }

        for name in self.services.keys() {
            if !visited[name] {
                self.visit(name, &mut visited, &mut sorted)?;
            }
        }
        
        Ok(sorted)
    }

    fn visit(&self, name: &String, visited: &mut BTreeMap<String, bool>, sorted: &mut Vec<String>) -> Result<(), &'static str> {
        visited.insert(name.clone(), true);
        if let Some(service) = self.services.get(name) {
            for dep in &service.dependencies {
                if let Some(is_visited) = visited.get(dep) {
                    if !is_visited {
                        self.visit(dep, visited, sorted)?;
                    }
                }
            }
        }
        sorted.push(name.clone());
        Ok(())
    }

    pub fn start_all(&mut self) -> Result<(), &'static str> {
        let order = self.topological_sort()?;
        for srv in order {
            self.running.push(srv);
        }
        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.running.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_service() {
        let mut init = InitSystem::new();
        init.add_service(ServiceUnit {
            name: "network".into(),
            description: "Network service".into(),
            exec_start: "/sbin/netd".into(),
            dependencies: vec![],
            restart_policy: RestartPolicy::Always,
        });
        assert_eq!(init.services.len(), 1);
    }

    #[test]
    fn test_topological_sort() {
        let mut init = InitSystem::new();
        init.add_service(ServiceUnit {
            name: "web".into(),
            description: "Web server".into(),
            exec_start: "/sbin/httpd".into(),
            dependencies: vec!["network".into()],
            restart_policy: RestartPolicy::Always,
        });
        init.add_service(ServiceUnit {
            name: "network".into(),
            description: "Network".into(),
            exec_start: "/sbin/netd".into(),
            dependencies: vec![],
            restart_policy: RestartPolicy::Always,
        });

        let order = init.topological_sort().unwrap();
        assert_eq!(order, vec!["network".to_string(), "web".to_string()]);
    }

    #[test]
    fn test_start_all() {
        let mut init = InitSystem::new();
        init.add_service(ServiceUnit {
            name: "network".into(),
            description: "Network".into(),
            exec_start: "/sbin/netd".into(),
            dependencies: vec![],
            restart_policy: RestartPolicy::Always,
        });
        assert!(init.start_all().is_ok());
        assert_eq!(init.running, vec!["network".to_string()]);
    }

    #[test]
    fn test_shutdown() {
        let mut init = InitSystem::new();
        init.running = vec!["network".into()];
        init.shutdown();
        assert!(init.running.is_empty());
    }

    #[test]
    fn test_service_creation() {
        let srv = ServiceUnit {
            name: "db".into(),
            description: "Database".into(),
            exec_start: "/sbin/dbd".into(),
            dependencies: vec![],
            restart_policy: RestartPolicy::OnFailure,
        };
        assert_eq!(srv.restart_policy, RestartPolicy::OnFailure);
    }
}
