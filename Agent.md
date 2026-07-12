# SigmaOS Autonomous Agents

SigmaOS replaces traditional background daemons and systemd services with **Autonomous Agents**. These agents are self-healing, goal-oriented shards that orchestrate the system based on the **Context Manager**.

---

## Agent Architecture

### Agent Hierarchy

The agent ecosystem is modularized into specialized tiers:

1. **Governance Agents**: Enforce security policies and resource quotas
2. **Maintenance Agents**: Perform self-healing, log rotation, and cache purging
3. **Observation Agents**: Monitor silicon health and network entropy
4. **Interface Agents**: Suggest workflows and optimize the Zenith UI
5. **Bridge Agents**: Manage legacy compatibility (e.g., Linux translation)

---

## Agent Lifecycle

### Initialization
```rust
pub trait Agent {
    fn init(&mut self, context: &Context) -> Result<()>;
    fn run(&mut self) -> Result<()>;
    fn shutdown(&mut self) -> Result<()>;
}
```

### Execution Model
- **Intent-Based**: Agents receive high-level goals (e.g., "minimize latency")
- **Event-Driven**: Agents respond to Sovereign Event Bus notifications
- **Self-Healing**: Watchdog shards monitor and restart failed agents
- **Capability-Gated**: All agent actions require explicit capability tokens

---

## Core Agents

### sigmad-health
Monitors system health and performs self-healing:
- Memory pressure detection and cleanup
- CPU throttling based on thermal state
- Disk space management
- Service restart on failure

### sigmad-pkg
Package management agent:
- Automatic dependency resolution
- Background updates
- Repository synchronization
- Signature verification

### sigmad-netd
Network management agent:
- Interface configuration
- DNS resolution
- Firewall rule management
- VPN connection management

### sigmad-vault
Secret management agent:
- TPM2 key sealing
- Credential storage
- Certificate management
- Secure key distribution

### sigmad-watchdog
Agent monitoring:
- Health checks for all agents
- Automatic restart on failure
- State rollback on corruption
- Resource quota enforcement

### sigmad-metrics
Telemetry and monitoring:
- Performance metrics collection
- Resource usage tracking
- Anomaly detection
- Alert generation

### sigmad-cloudsync
Cloud synchronization:
- File synchronization
- State backup
- Cluster coordination
- Distributed consensus

---

## Agent Communication

### Sovereign Event Bus

All agents communicate via the event bus:

```rust
pub enum Event {
    HealthCheck(AgentId),
    ResourceAlert(ResourceType, Level),
    SecurityViolation(ViolationType),
    ConfigChange(ConfigKey, ConfigValue),
    ShutdownRequest,
}
```

### Message Format

```rust
pub struct Message {
    pub source: AgentId,
    pub target: AgentId,
    pub event: Event,
    pub timestamp: u64,
    pub signature: Signature,
}
```

---

## Agent SDK

### Creating a Custom Agent

```rust
use sigma_agent::{Agent, Context, Result};

pub struct MyAgent {
    config: MyConfig,
    state: MyState,
}

impl Agent for MyAgent {
    fn init(&mut self, context: &Context) -> Result<()> {
        // Initialize agent
        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        // Main agent loop
        loop {
            // Wait for events
            let event = context.receive_event()?;
            
            // Process event
            self.handle_event(event)?;
            
            // Check for shutdown
            if context.should_shutdown() {
                break;
            }
        }
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        // Cleanup
        Ok(())
    }
}
```

---

## Security Model

### Capability Requirements

All agents must declare required capabilities:

```rust
#[agent(capabilities = ["network", "filesystem", "process")]
pub struct NetworkAgent;
```

### Sandboxing

Agents run in capability-gated sandboxes:
- Filesystem access restricted to declared paths
- Network access restricted to declared endpoints
- System calls filtered by capability list
- Resource quotas enforced by kernel

### Audit Logging

All agent actions are logged:
- BLAKE3 hash of action state
- Cryptographic signature for verification
- Immutable audit trail
- Tamper-evident storage

---

## Performance Considerations

### Intent Caching

```rust
pub struct AgentBase {
    intent_cache: HashMap<Intent, CachedResult>,
    cache_ttl: Duration,
}
```

### Asynchronous Execution

Agents use async/await for non-blocking operations:
- I/O operations don't block other agents
- Priority-aware task scheduling
- Lock-free data structures where possible

### Resource Quotas

Each agent has resource limits:
- CPU time quota
- Memory limit
- I/O bandwidth limit
- Network bandwidth limit

---

## Monitoring

### Health Checks

Agents report health status:
- Status: Healthy/Degraded/Failed
- Resource usage
- Error count
- Last successful operation

### Metrics

Agents expose metrics:
- Request rate
- Latency percentiles
- Error rate
- Resource utilization

---

## Troubleshooting

### Agent Not Starting
1. Check capability declarations
2. Verify resource quotas
3. Review agent logs
4. Check watchdog status

### Agent Crashing
1. Review crash logs
2. Check for resource exhaustion
3. Verify event bus connectivity
4. Check for capability violations

### Agent Not Responding
1. Check if agent is blocked on I/O
2. Verify event bus is functioning
3. Check for deadlock
4. Review agent priority

---

*See also: [AGENTS.md](AGENTS.md) · [AI_Agent.md](AI_Agent.md) · [Architecture.md](Architecture.md)*
