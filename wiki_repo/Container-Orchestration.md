# Container Orchestration

SigmaOS implements container orchestration with Kubernetes-compatible APIs, pod scheduling, service discovery, and autoscaling for cloud-native application deployment.

## Overview

Container orchestration provides:
- Kubernetes-compatible API for pod and service management
- Pod scheduling with affinity and anti-affinity rules
- Service discovery with DNS and environment variables
- Horizontal pod autoscaling based on metrics
- Rolling updates and rollbacks
- Secret and configuration management
- Network policies for pod-to-pod communication
- Resource quotas and limits

## Architecture

### Orchestration Stack
```
API Server → Scheduler → Controller Manager → Kubelet
                  ↓
               etcd (cluster state)
                  ↓
               Container Runtime (containerd)
                  ↓
               CNI Plugin (networking)
```

### Pod Lifecycle
- **Pending**: Pod accepted but not yet scheduled
- **Running**: Pod containers are running
- **Succeeded**: All containers terminated successfully
- **Failed**: At least one container terminated with error
- **Unknown**: State cannot be determined

## Implementation

### Pod Scheduler
```rust
// src/orchestration/scheduler.rs
pub struct PodScheduler {
    pub node_info: BTreeMap<String, NodeInfo>,
    pub pending_pods: VecDeque<Pod>,
    pub scheduling_queue: PriorityQueue<Pod>,
}

impl PodScheduler {
    pub fn new() -> Self {
        PodScheduler {
            node_info: BTreeMap::new(),
            pending_pods: VecDeque::new(),
            scheduling_queue: PriorityQueue::new(),
        }
    }

    pub fn schedule(&mut self, pod: Pod) -> Result<String, ScheduleError> {
        // Find suitable node
        let node = self.find_suitable_node(&pod)?;
        
        // Reserve resources
        self.reserve_resources(&node, &pod)?;
        
        // Assign pod to node
        self.assign_pod(&node, pod)?;
        
        Ok(node)
    }

    fn find_suitable_node(&self, pod: &Pod) -> Result<String, ScheduleError> {
        let mut suitable_nodes = Vec::new();
        
        for (node_name, node_info) in &self.node_info {
            if self.check_node_suitability(node_info, pod) {
                suitable_nodes.push(node_name.clone());
            }
        }
        
        if suitable_nodes.is_empty() {
            return Err(ScheduleError::NoSuitableNode);
        }
        
        // Score nodes and select best
        Ok(self.score_and_select(&suitable_nodes, pod))
    }

    fn check_node_suitability(&self, node: &NodeInfo, pod: &Pod) -> bool {
        // Check resource availability
        if !self.check_resources(node, pod) {
            return false;
        }
        
        // Check affinity rules
        if !self.check_affinity(node, pod) {
            return false;
        }
        
        // Check taints and tolerations
        if !self.check_taints(node, pod) {
            return false;
        }
        
        true
    }
}
```

### Controller Manager
```rust
// src/orchestration/controller.rs
pub struct ControllerManager {
    pub replica_set_controller: ReplicaSetController,
    pub deployment_controller: DeploymentController,
    pub service_controller: ServiceController,
    pub namespace_controller: NamespaceController,
}

impl ControllerManager {
    pub fn new() -> Self {
        ControllerManager {
            replica_set_controller: ReplicaSetController::new(),
            deployment_controller: DeploymentController::new(),
            service_controller: ServiceController::new(),
            namespace_controller: NamespaceController::new(),
        }
    }

    pub fn reconcile_replica_set(&mut self, replica_set: &ReplicaSet) -> Result<(), ControllerError> {
        let current_pods = self.get_pods_for_replica_set(replica_set)?;
        let desired_replicas = replica_set.spec.replicas;
        let current_replicas = current_pods.len() as i32;
        
        if current_replicas < desired_replicas {
            // Scale up
            let diff = desired_replicas - current_replicas;
            for _ in 0..diff {
                self.create_pod_for_replica_set(replica_set)?;
            }
        } else if current_replicas > desired_replicas {
            // Scale down
            let diff = current_replicas - desired_replicas;
            for _ in 0..diff {
                if let Some(pod) = current_pods.last() {
                    self.delete_pod(pod)?;
                }
            }
        }
        
        Ok(())
    }
}
```

### Service Discovery
```rust
// src/orchestration/service_discovery.rs
pub struct ServiceDiscovery {
    pub services: BTreeMap<String, Service>,
    pub endpoints: BTreeMap<String, Vec<Endpoint>>,
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub namespace: String,
    pub selector: BTreeMap<String, String>,
    pub ports: Vec<ServicePort>,
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub ip: String,
    pub port: u16,
    pub pod_name: String,
}

impl ServiceDiscovery {
    pub fn new() -> Self {
        ServiceDiscovery {
            services: BTreeMap::new(),
            endpoints: BTreeMap::new(),
        }
    }

    pub fn resolve_service(&self, service_name: &str, namespace: &str) -> Option<Vec<Endpoint>> {
        let key = format!("{}/{}", namespace, service_name);
        self.endpoints.get(&key).cloned()
    }

    pub fn update_endpoints(&mut self, pods: &[Pod], services: &[Service]) {
        for service in services {
            let key = format!("{}/{}", service.namespace, service.name);
            let mut endpoints = Vec::new();
            
            for pod in pods {
                if self.pod_matches_service(pod, service) {
                    endpoints.push(Endpoint {
                        ip: pod.status.pod_ip.clone(),
                        port: service.ports[0].port,
                        pod_name: pod.metadata.name.clone(),
                    });
                }
            }
            
            self.endpoints.insert(key, endpoints);
        }
    }
}
```

### Autoscaler
```rust
// src/orchestration/autoscaler.rs
pub struct HorizontalPodAutoscaler {
    pub target_utilization: f64,
    pub min_replicas: i32,
    pub max_replicas: i32,
    pub scale_up_cooldown: Duration,
    pub scale_down_cooldown: Duration,
}

impl HorizontalPodAutoscaler {
    pub fn new() -> Self {
        HorizontalPodAutoscaler {
            target_utilization: 80.0,
            min_replicas: 1,
            max_replicas: 10,
            scale_up_cooldown: Duration::from_secs(60),
            scale_down_cooldown: Duration::from_secs(300),
        }
    }

    pub fn calculate_desired_replicas(&self, current_replicas: i32, current_utilization: f64) -> i32 {
        if current_utilization < self.target_utilization {
            // Scale down
            let ratio = current_utilization / self.target_utilization;
            let desired = ((current_replicas as f64) * ratio).ceil() as i32;
            desired.max(self.min_replicas)
        } else {
            // Scale up
            let ratio = current_utilization / self.target_utilization;
            let desired = ((current_replicas as f64) * ratio).ceil() as i32;
            desired.min(self.max_replicas)
        }
    }
}
```

## Configuration

### Orchestration Configuration
```toml
# /etc/sigmaos/orchestration.toml
[api_server]
# API server settings
enabled = true
port = 6443
tls_enabled = true

[scheduler]
# Scheduler settings
strategy = "binpack"
bind_timeout_seconds = 30

[controller]
# Controller manager settings
concurrency = 5
resync_interval_seconds = 600

[autoscaler]
# Autoscaler settings
enabled = true
target_utilization = 80
min_replicas = 1
max_replicas = 10
```

### Runtime Control
```bash
# Create pod
sigkubectl create-pod --name nginx --image nginx

# Create deployment
sigkubectl create-deployment --name web --image nginx --replicas 3

# Scale deployment
sigkubectl scale deployment web --replicas 5

# View pods
sigkubectl get pods

# View services
sigkubectl get services

# Autoscale deployment
sigkubectl autoscale deployment web --min=2 --max=10 --cpu-percent=80
```

## Performance Optimization

### Scheduling
Optimize pod scheduling:
```bash
# Use bin-packing strategy
sigkubectl set-scheduler-strategy binpack

# Increase bind timeout
sigkubectl set-bind-timeout 60

# Enable preemption
sigkubectl enable-preemption
```

### Controller
Optimize controller performance:
```bash
# Increase concurrency
sigkubectl set-controller-concurrency 10

# Adjust resync interval
sigkubectl set-resync-interval 300

# Enable leader election
sigkubectl enable-leader-election
```

### Autoscaling
Optimize autoscaling:
```bash
# Adjust target utilization
sigkubectl set-target-utilization 70

# Adjust cooldown periods
sigkubectl set-scale-up-cooldown 30
sigkubectl set-scale-down-cooldown 180
```

## Troubleshooting

### Pod Pending
If pod is stuck in Pending state:
1. Check events: `sigkubectl describe pod <pod-name>`
2. Check scheduler logs: `sigkubectl logs scheduler`
3. Check resource availability: `sigkubectl describe nodes`
4. Check affinity rules
5. Check taints and tolerations

### Pod CrashLoopBackOff
If pod is in CrashLoopBackOff:
1. Check pod logs: `sigkubectl logs <pod-name>`
2. Check previous logs: `sigkubectl logs <pod-name> --previous`
3. Check resource limits
4. Check image pull status
5. Check container command

### Service Not Reachable
If service is not reachable:
1. Check service endpoints: `sigkubectl get endpoints <service-name>`
2. Check pod labels: `sigkubectl get pods --show-labels`
3. Check service selector
4. Check network policies
5. Check DNS resolution

### Autoscaling Not Working
If autoscaling is not working:
1. Check metrics server: `sigkubectl get metrics`
2. Check HPA status: `sigkubectl describe hpa <hpa-name>`
3. Check target utilization
4. Check min/max replicas
5. Check cooldown periods

---

**[Orchestration](Category-Orchestration)** | **[Containers](Category-Containers)** | **[Kubernetes](Category-Kubernetes)**
