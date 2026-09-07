# AI Agent High-Availability Cluster Operation & Fleet Mesh Management in SigmaOS

## Overview

SigmaOS cluster architecture (`src/distro/missing_distro_innovations.rs`, `src/net/mesh/`, `src/orchestration/`) provides high-availability multi-node mesh clustering, Raft/Paxos-parity consensus state replication, P2P fleet mesh routing (`sigma_fleet_protocol`), and distributed workload balancing.

AI agents (such as Jules, Herdr agentic cluster managers, distributed build orchestrators, and fault-tolerant worker nodes) must adhere to these cluster operation guidelines when orchestrating multi-node agent fleets.

---

## Cluster Node Topology & Node Roles

SigmaOS cluster nodes operate under distinct roles (`ClusterNodeRole`):

```
       [ Leader Node ] (Consensus Master & Task Dispatcher)
              │
      ┌───────┴───────┐
      ▼               ▼
[ Worker Node ]  [ Worker Node ] (Subagent Sandbox Execution)
      │               │
      └───────┬───────┘
              ▼
    [ Observer Node ] (Raft Consensus Read-Only Follower)
```

| Node Role | Class | Primary Function |
|-----------|-------|------------------|
| **`ClusterNodeRole::Leader`** | Primary Master | Coordinates consensus state, dispatches distributed agent tasks |
| **`ClusterNodeRole::Worker`** | Task Executor | Runs sandboxed AI subagents and reports task telemetry |
| **`ClusterNodeRole::Observer`** | Read-Only Monitor | Maintains replicated audit logs and participates in leader elections |

---

## 1. High-Availability Mesh Orchestration (`SovereignHighAvailabilityMeshEngine`)

AI agents manage multi-node cluster membership and node health heartbeats:

```rust
use sigmaos::distro::missing_distro_innovations::{SovereignHighAvailabilityMeshEngine, ClusterNodeRole};

let mut cluster = SovereignHighAvailabilityMeshEngine::new();

// Register node into high-availability cluster mesh
cluster.register_cluster_node("node-agent-01", "192.168.1.10", ClusterNodeRole::Worker)?;

// Send periodic node heartbeat ping
cluster.process_node_heartbeat("node-agent-01", now_timestamp_sec)?;

// Check cluster quorum status
if cluster.has_quorum() {
    println!("Cluster quorum active. Safe to dispatch distributed agent tasks.");
}
```

---

## 2. P2P Fleet Mesh Routing (`sigma_fleet_protocol`)

AI agents communicate across cluster nodes using encrypted P2P fleet mesh routing:

```rust
use sigmaos::net::mesh::SigmaFleetProtocol;

let mut fleet = SigmaFleetProtocol::new("cluster-mesh-sec01");

// Broadcast task payload across cluster nodes
fleet.broadcast_task_payload("TASK_ID_42", agent_task_bytes)?;

// Receive worker node execution status
if let Some(status) = fleet.receive_node_status("node-agent-01") {
    println!("Node status: {}", status);
}
```

---

## 3. Automated Node Failover & Consensus Re-Election

When a worker or leader node experiences network partition or hardware failure, the high-availability engine triggers failover:

```
Heartbeat Timeout (node-agent-01) → Trigger Failover Event
                                          │
                                          ▼
                         Re-allocate Active Subagent Workloads
                                          │
                                          ▼
                         Re-elect New Cluster Leader Node
```

---

## Directives for AI Agents Operating in Clusters

1. **Verify Quorum Before Mutations**: Always confirm `cluster.has_quorum()` before executing stateful cluster operations.
2. **Encrypted Fleet Communications**: Ensure all P2P inter-node messages use WireGuard or post-quantum Dilithium5 signed channels.
3. **Graceful Node Decommissioning**: Drain active subagent tasks (`quiesce_node`) before shutting down cluster nodes.
