// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/dev/sigma_k3s.rs — Sigma Kubernetes k3s Orchestration
//
// Implements Kubernetes/k3s-style container orchestration with
// pod management, services, deployments, and cluster operations.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Kubernetes Types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PodPhase {
    Pending,
    Running,
    Succeeded,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServiceType {
    ClusterIP,
    NodePort,
    LoadBalancer,
    ExternalName,
}

#[derive(Debug, Clone)]
pub struct ContainerPort {
    pub name: String,
    pub container_port: u16,
    pub protocol: String,  // TCP, UDP
}

#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub image: String,
    pub ports: Vec<ContainerPort>,
    pub env_vars: HashMap<String, String>,
    pub resources: ResourceRequirements,
    pub ready: bool,
    pub restart_count: u32,
}

#[derive(Debug, Clone)]
pub struct ResourceRequirements {
    pub cpu_request: String,
    pub cpu_limit: String,
    pub memory_request: String,
    pub memory_limit: String,
}

#[derive(Debug, Clone)]
pub struct Pod {
    pub name: String,
    pub namespace: String,
    pub phase: PodPhase,
    pub containers: Vec<Container>,
    pub node_name: String,
    pub pod_ip: String,
    pub created: String,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub namespace: String,
    pub service_type: ServiceType,
    pub cluster_ip: String,
    pub external_ip: Option<String>,
    pub ports: Vec<ServicePort>,
    pub selector: HashMap<String, String>,
    pub created: String,
}

#[derive(Debug, Clone)]
pub struct ServicePort {
    pub name: String,
    pub port: u16,
    pub target_port: u16,
    pub node_port: Option<u16>,
    pub protocol: String,
}

#[derive(Debug, Clone)]
pub struct Deployment {
    pub name: String,
    pub namespace: String,
    pub replicas: u32,
    pub available_replicas: u32,
    pub updated_replicas: u32,
    pub selector: HashMap<String, String>,
    pub template: PodTemplate,
    pub created: String,
}

#[derive(Debug, Clone)]
pub struct PodTemplate {
    pub containers: Vec<Container>,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub ready: bool,
    pub roles: Vec<String>,
    pub version: String,
    pub cpu_capacity: String,
    pub memory_capacity: String,
    pub pods_running: u32,
}

// ─── K3s Manager ─────────────────────────────────────────────────────────

pub struct K3sManager {
    pub pods: HashMap<String, Pod>,
    pub services: HashMap<String, Service>,
    pub deployments: HashMap<String, Deployment>,
    pub nodes: HashMap<String, Node>,
    pub namespaces: Vec<String>,
    pub cluster_name: String,
    pub server_running: bool,
}

impl K3sManager {
    pub fn new() -> Self {
        let mut manager = K3sManager {
            pods: HashMap::new(),
            services: HashMap::new(),
            deployments: HashMap::new(),
            nodes: HashMap::new(),
            namespaces: vec!["default".to_string(), "kube-system".to_string()],
            cluster_name: "sigmaos-cluster".to_string(),
            server_running: true,
        };

        manager.init_default_nodes();
        manager
    }

    /// Initialize default nodes
    fn init_default_nodes(&mut self) {
        self.nodes.insert("node1".to_string(), Node {
            name: "node1".to_string(),
            ready: true,
            roles: vec!["control-plane".to_string(), "master".to_string()],
            version: "v1.28.0+k3s1".to_string(),
            cpu_capacity: "4".to_string(),
            memory_capacity: "16Gi".to_string(),
            pods_running: 0,
        });

        self.nodes.insert("node2".to_string(), Node {
            name: "node2".to_string(),
            ready: true,
            roles: vec!["worker".to_string()],
            version: "v1.28.0+k3s1".to_string(),
            cpu_capacity: "8".to_string(),
            memory_capacity: "32Gi".to_string(),
            pods_running: 0,
        });
    }

    /// Create a namespace
    pub fn create_namespace(&mut self, name: String) -> Result<(), String> {
        if self.namespaces.contains(&name) {
            return Err("Namespace already exists".to_string());
        }
        self.namespaces.push(name);
        Ok(())
    }

    /// Create a pod
    pub fn create_pod(&mut self, name: String, namespace: String, containers: Vec<Container>) -> Result<Pod, String> {
        let pod_id = format!("{}/{}", namespace, name);
        
        if self.pods.contains_key(&pod_id) {
            return Err("Pod already exists".to_string());
        }

        let pod = Pod {
            name: name.clone(),
            namespace: namespace.clone(),
            phase: PodPhase::Pending,
            containers,
            node_name: "node1".to_string(),
            pod_ip: "10.42.0.1".to_string(),
            created: "now".to_string(),
            labels: HashMap::new(),
        };

        self.pods.insert(pod_id.clone(), pod.clone());
        
        // Simulate pod starting
        if let Some(p) = self.pods.get_mut(&pod_id) {
            p.phase = PodPhase::Running;
        }

        Ok(pod)
    }

    /// Delete a pod
    pub fn delete_pod(&mut self, namespace: &str, name: &str) -> Result<(), String> {
        let pod_id = format!("{}/{}", namespace, name);
        if self.pods.remove(&pod_id).is_some() {
            Ok(())
        } else {
            Err("Pod not found".to_string())
        }
    }

    /// Create a service
    pub fn create_service(&mut self, name: String, namespace: String, service_type: ServiceType, ports: Vec<ServicePort>, selector: HashMap<String, String>) -> Result<Service, String> {
        let service_id = format!("{}/{}", namespace, name);
        
        if self.services.contains_key(&service_id) {
            return Err("Service already exists".to_string());
        }

        let cluster_ip = match service_type {
            ServiceType::ClusterIP => "10.43.0.1".to_string(),
            ServiceType::NodePort => "10.43.0.2".to_string(),
            ServiceType::LoadBalancer => "10.43.0.3".to_string(),
            ServiceType::ExternalName => "external.example.com".to_string(),
        };

        let service = Service {
            name: name.clone(),
            namespace: namespace.clone(),
            service_type,
            cluster_ip,
            external_ip: None,
            ports,
            selector,
            created: "now".to_string(),
        };

        self.services.insert(service_id.clone(), service.clone());
        Ok(service)
    }

    /// Delete a service
    pub fn delete_service(&mut self, namespace: &str, name: &str) -> Result<(), String> {
        let service_id = format!("{}/{}", namespace, name);
        if self.services.remove(&service_id).is_some() {
            Ok(())
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Create a deployment
    pub fn create_deployment(&mut self, name: String, namespace: String, replicas: u32, containers: Vec<Container>, selector: HashMap<String, String>) -> Result<Deployment, String> {
        let deployment_id = format!("{}/{}", namespace, name);
        
        if self.deployments.contains_key(&deployment_id) {
            return Err("Deployment already exists".to_string());
        }

        let deployment = Deployment {
            name: name.clone(),
            namespace: namespace.clone(),
            replicas,
            available_replicas: 0,
            updated_replicas: 0,
            selector: selector.clone(),
            template: PodTemplate {
                containers,
                labels: selector,
            },
            created: "now".to_string(),
        };

        self.deployments.insert(deployment_id.clone(), deployment.clone());
        
        // Simulate deployment
        self.scale_deployment(namespace, name, replicas).ok();
        
        Ok(deployment)
    }

    /// Scale a deployment
    pub fn scale_deployment(&mut self, namespace: &str, name: &str, replicas: u32) -> Result<(), String> {
        let deployment_id = format!("{}/{}", namespace, name);
        if let Some(deployment) = self.deployments.get_mut(&deployment_id) {
            deployment.replicas = replicas;
            deployment.available_replicas = replicas;
            deployment.updated_replicas = replicas;
            
            // Update pod count on nodes
            for node in self.nodes.values_mut() {
                node.pods_running = self.pods.values()
                    .filter(|p| p.node_name == node.name)
                    .count() as u32;
            }
            
            Ok(())
        } else {
            Err("Deployment not found".to_string())
        }
    }

    /// Delete a deployment
    pub fn delete_deployment(&mut self, namespace: &str, name: &str) -> Result<(), String> {
        let deployment_id = format!("{}/{}", namespace, name);
        if self.deployments.remove(&deployment_id).is_some() {
            Ok(())
        } else {
            Err("Deployment not found".to_string())
        }
    }

    /// Get pods by namespace
    pub fn get_pods(&self, namespace: Option<&str>) -> Vec<&Pod> {
        if let Some(ns) = namespace {
            self.pods.values().filter(|p| p.namespace == ns).collect()
        } else {
            self.pods.values().collect()
        }
    }

    /// Get services by namespace
    pub fn get_services(&self, namespace: Option<&str>) -> Vec<&Service> {
        if let Some(ns) = namespace {
            self.services.values().filter(|s| s.namespace == ns).collect()
        } else {
            self.services.values().collect()
        }
    }

    /// Get deployments by namespace
    pub fn get_deployments(&self, namespace: Option<&str>) -> Vec<&Deployment> {
        if let Some(ns) = namespace {
            self.deployments.values().filter(|d| d.namespace == ns).collect()
        } else {
            self.deployments.values().collect()
        }
    }

    /// Get all nodes
    pub fn get_nodes(&self) -> Vec<&Node> {
        self.nodes.values().collect()
    }

    /// Get cluster info
    pub fn get_cluster_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("cluster_name".to_string(), self.cluster_name.clone());
        info.insert("server_running".to_string(), self.server_running.to_string());
        info.insert("nodes".to_string(), self.nodes.len().to_string());
        info.insert("pods".to_string(), self.pods.len().to_string());
        info.insert("services".to_string(), self.services.len().to_string());
        info.insert("deployments".to_string(), self.deployments.len().to_string());
        info.insert("namespacespaces".to_string(), self.namespaces.len().to_string());
        info
    }

    /// Start server
    pub fn start_server(&mut self) {
        self.server_running = true;
    }

    /// Stop server
    pub fn stop_server(&mut self) {
        self.server_running = false;
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut k3s = K3sManager::new();
    
    println!("Sigma k3s v0.1 - Kubernetes Orchestration");
    
    loop {
        println!("\n--- k3s Commands ---");
        println!("cluster           - Show cluster info");
        println!("nodes             - List nodes");
        println!("namespaces        - List namespaces");
        println!("create_ns <name>  - Create namespace");
        println!("pods [ns]         - List pods");
        println!("create_pod <name> <ns> <image> - Create pod");
        println!("delete_pod <ns> <name> - Delete pod");
        println!("services [ns]     - List services");
        println!("create_svc <name> <ns> <type> <port> - Create service");
        println!("delete_svc <ns> <name> - Delete service");
        println!("deployments [ns]  - List deployments");
        println!("create_deploy <name> <ns> <replicas> <image> - Create deployment");
        println!("scale <ns> <name> <replicas> - Scale deployment");
        println!("delete_deploy <ns> <name> - Delete deployment");
        println!("start_server      - Start k3s server");
        println!("stop_server       - Stop k3s server");
        println!("quit              - Exit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "cluster" => {
                println!("--- Cluster Info ---");
                for (key, value) in k3s.get_cluster_info() {
                    println!("{}: {}", key, value);
                }
            }
            "nodes" => {
                println!("--- Nodes ---");
                for node in k3s.get_nodes() {
                    println!("{} - {:?} - {} - {} - {} pods", 
                        node.name, node.ready, node.version, node.roles.join(", "), node.pods_running);
                }
            }
            "namespaces" => {
                println!("--- Namespaces ---");
                for ns in &k3s.namespaces {
                    println!("{}", ns);
                }
            }
            "create_ns" => {
                if let Some(name) = parts.get(1) {
                    match k3s.create_namespace(name.to_string()) {
                        Ok(_) => println!("Namespace created"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "pods" => {
                let ns = parts.get(1).copied();
                println!("--- Pods ---");
                for pod in k3s.get_pods(ns) {
                    println!("{}/{} - {:?} - {} - {}", 
                        pod.namespace, pod.name, pod.phase, pod.node_name, pod.pod_ip);
                }
            }
            "create_pod" => {
                if parts.len() >= 4 {
                    let name = parts[1].to_string();
                    let namespace = parts[2].to_string();
                    let image = parts[3].to_string();
                    let container = Container {
                        name: name.clone(),
                        image,
                        ports: vec![],
                        env_vars: HashMap::new(),
                        resources: ResourceRequirements {
                            cpu_request: "100m".to_string(),
                            cpu_limit: "500m".to_string(),
                            memory_request: "128Mi".to_string(),
                            memory_limit: "256Mi".to_string(),
                        },
                        ready: true,
                        restart_count: 0,
                    };
                    match k3s.create_pod(name, namespace, vec![container]) {
                        Ok(_) => println!("Pod created"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "delete_pod" => {
                if parts.len() >= 3 {
                    let namespace = parts[1];
                    let name = parts[2];
                    match k3s.delete_pod(namespace, name) {
                        Ok(_) => println!("Pod deleted"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "services" => {
                let ns = parts.get(1).copied();
                println!("--- Services ---");
                for service in k3s.get_services(ns) {
                    println!("{}/{} - {:?} - {} - {:?}", 
                        service.namespace, service.name, service.service_type, service.cluster_ip, service.ports);
                }
            }
            "create_svc" => {
                if parts.len() >= 5 {
                    let name = parts[1].to_string();
                    let namespace = parts[2].to_string();
                    let service_type = match parts[3] {
                        "ClusterIP" => ServiceType::ClusterIP,
                        "NodePort" => ServiceType::NodePort,
                        "LoadBalancer" => ServiceType::LoadBalancer,
                        "ExternalName" => ServiceType::ExternalName,
                        _ => ServiceType::ClusterIP,
                    };
                    let port = parts[4].parse::<u16>().unwrap_or(80);
                    let service_port = ServicePort {
                        name: "http".to_string(),
                        port,
                        target_port: port,
                        node_port: if service_type == ServiceType::NodePort { Some(30000 + port) } else { None },
                        protocol: "TCP".to_string(),
                    };
                    match k3s.create_service(name, namespace, service_type, vec![service_port], HashMap::new()) {
                        Ok(_) => println!("Service created"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "delete_svc" => {
                if parts.len() >= 3 {
                    let namespace = parts[1];
                    let name = parts[2];
                    match k3s.delete_service(namespace, name) {
                        Ok(_) => println!("Service deleted"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "deployments" => {
                let ns = parts.get(1).copied();
                println!("--- Deployments ---");
                for deployment in k3s.get_deployments(ns) {
                    println!("{}/{} - {}/{} - {}/{} ready", 
                        deployment.namespace, deployment.name, 
                        deployment.available_replicas, deployment.replicas,
                        deployment.updated_replicas, deployment.replicas);
                }
            }
            "create_deploy" => {
                if parts.len() >= 5 {
                    let name = parts[1].to_string();
                    let namespace = parts[2].to_string();
                    let replicas = parts[3].parse::<u32>().unwrap_or(1);
                    let image = parts[4].to_string();
                    let container = Container {
                        name: name.clone(),
                        image,
                        ports: vec![],
                        env_vars: HashMap::new(),
                        resources: ResourceRequirements {
                            cpu_request: "100m".to_string(),
                            cpu_limit: "500m".to_string(),
                            memory_request: "128Mi".to_string(),
                            memory_limit: "256Mi".to_string(),
                        },
                        ready: true,
                        restart_count: 0,
                    };
                    let selector = HashMap::from([("app".to_string(), name.clone())]);
                    match k3s.create_deployment(name, namespace, replicas, vec![container], selector) {
                        Ok(_) => println!("Deployment created"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "scale" => {
                if parts.len() >= 4 {
                    let namespace = parts[1];
                    let name = parts[2];
                    let replicas = parts[3].parse::<u32>().unwrap_or(1);
                    match k3s.scale_deployment(namespace, name, replicas) {
                        Ok(_) => println!("Deployment scaled"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "delete_deploy" => {
                if parts.len() >= 3 {
                    let namespace = parts[1];
                    let name = parts[2];
                    match k3s.delete_deployment(namespace, name) {
                        Ok(_) => println!("Deployment deleted"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "start_server" => {
                k3s.start_server();
                println!("k3s server started");
            }
            "stop_server" => {
                k3s.stop_server();
                println!("k3s server stopped");
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
