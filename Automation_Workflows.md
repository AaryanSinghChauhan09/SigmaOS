# Automation Workflows: Orchestrator Pipeline

Learn how to configure multi-step execution graphs in SigmaOS without human intervention.

---

## 🔗 Action-Trigger Model

Workflows are modeled as Directed Acyclic Graphs (DAGs) in `sigma_logic.rs`.

```
[System Trigger / Metric Alert]
              │
              ▼
    [Filter & Pre-Condition]
              │
              ▼
   ┌──────────┴──────────┐
   ▼                     ▼
[Action A: Sandbox]    [Action B: Telemetry]
   │                     │
   └──────────┬──────────┘
              ▼
     [Final Consolidation]
```

### Workflow Components

#### Triggers

Triggers initiate workflow execution:

- **System Events**: Boot, shutdown, hardware changes
- **Metric Alerts**: CPU threshold, memory pressure, disk space
- **Time-based**: Cron-like scheduling
- **User Actions**: File changes, application launches
- **External Events**: Network events, API calls

#### Filters

Filters determine if a workflow should execute:

- **Condition Evaluation**: Boolean logic on system state
- **Threshold Checks**: Metric threshold validation
- **State Verification**: System state validation
- **Permission Checks**: Capability verification
- **Resource Availability**: Resource availability checks

#### Actions

Actions are the executable steps in a workflow:

- **Sandboxed Execution**: Run in isolated environment
- **Telemetry Collection**: Gather system metrics
- **Resource Allocation**: Allocate system resources
- **Configuration Changes**: Apply system changes
- **Notification**: Send alerts or notifications

#### Consolidation

Consolidation merges results from parallel actions:

- **Result Aggregation**: Combine results from multiple actions
- **Conflict Resolution**: Resolve conflicting results
- **State Update**: Update system state
- **Cleanup**: Clean up temporary resources
- **Logging**: Log workflow execution

---

## ⚙️ Example: Dev Environment Setup

To setup a new development workspace, the agent automatically executes:

1. `sigpkg install gcc git rust`
2. `git clone <repository>`
3. Set workspace environment variables
4. Launch `sigma-edit` bound to the workspace directory

### Workflow Definition

```toml
[workflow.dev-setup]
trigger = "user:new-workspace"
description = "Automated development environment setup"

[[workflow.dev-setup.steps]]
name = "install-tools"
command = "sigpkg install gcc git rust"
sandbox = true

[[workflow.dev-setup.steps]]
name = "clone-repo"
command = "git clone ${repo_url}"
depends_on = ["install-tools"]

[[workflow.dev-setup.steps]]
name = "set-env"
command = "export PATH=${workspace}/bin:$PATH"
depends_on = ["clone-repo"]

[[workflow.dev-setup.steps]]
name = "launch-editor"
command = "sigma-edit ${workspace}"
depends_on = ["set-env"]
```

---

## 🚀 Built-in Workflows

### System Maintenance

#### Auto-Update Workflow

```toml
[workflow.auto-update]
trigger = "schedule:daily"
description = "Automatic system updates"

[[workflow.auto-update.steps]]
name = "check-updates"
command = "sigpkg check-updates"

[[workflow.auto-update.steps]]
name = "download-updates"
command = "sigpkg download"
depends_on = ["check-updates"]
condition = "updates_available"

[[workflow.auto-update.steps]]
name = "apply-updates"
command = "sigpkg upgrade"
depends_on = ["download-updates"]
requires_reboot = true
```

#### Log Rotation Workflow

```toml
[workflow.log-rotate]
trigger = "schedule:weekly"
description = "Rotate and compress system logs"

[[workflow.log-rotate.steps]]
name = "compress-logs"
command = "sigma-log compress --older-than 7d"

[[workflow.log-rotate.steps]]
name = "archive-logs"
command = "sigma-log archive --destination /var/archive/logs"
depends_on = ["compress-logs"]

[[workflow.log-rotate.steps]]
name = "cleanup-old-logs"
command = "sigma-log cleanup --older-than 90d"
depends_on = ["archive-logs"]
```

### Performance Optimization

#### Memory Cleanup Workflow

```toml
[workflow.memory-cleanup]
trigger = "metric:memory-pressure > 80%"
description = "Clean up memory when pressure is high"

[[workflow.memory-cleanup.steps]]
name = "clear-cache"
command = "sigma-sys clear-cache"

[[workflow.memory-cleanup.steps]]
name = "terminate-idle"
command = "sigma-sys terminate-idle-processes"
depends_on = ["clear-cache"]

[[workflow.memory-cleanup.steps]]
name = "compact-memory"
command = "sigma-sys compact-memory"
depends_on = ["terminate-idle"]
```

#### Disk Cleanup Workflow

```toml
[workflow.disk-cleanup]
trigger = "metric:disk-usage > 90%"
description = "Clean up disk space when usage is high"

[[workflow.disk-cleanup.steps]]
name = "clear-temp"
command = "sigma-fs clean /tmp"

[[workflow.disk-cleanup.steps]]
name = "clear-cache"
command = "sigpkg clean-cache"
depends_on = ["clear-temp"]

[[workflow.disk-cleanup.steps]]
name = "remove-old-packages"
command = "sigpkg remove-old"
depends_on = ["clear-cache"]
```

### Security Workflows

#### Security Audit Workflow

```toml
[workflow.security-audit]
trigger = "schedule:daily"
description = "Daily security audit"

[[workflow.security-audit.steps]]
name = "check-vulnerabilities"
command = "sigpkg check-vulnerabilities"

[[workflow.security-audit.steps]]
name = "audit-capabilities"
command = "sigma-sec audit-capabilities"
depends_on = ["check-vulnerabilities"]

[[workflow.security-audit.steps]]
name = "verify-integrity"
command = "sigpkg verify-integrity"
depends_on = ["audit-capabilities"]

[[workflow.security-audit.steps]]
name = "generate-report"
command = "sigma-sec generate-report"
depends_on = ["verify-integrity"]
```

#### Incident Response Workflow

```toml
[workflow.incident-response]
trigger = "alert:security-breach"
description = "Automated incident response"

[[workflow.incident-response.steps]]
name = "isolate-system"
command = "sigma-sec isolate"
priority = "critical"

[[workflow.incident-response.steps]]
name = "collect-evidence"
command = "sigma-forensics collect"
depends_on = ["isolate-system"]

[[workflow.incident-response.steps]]
name = "notify-admin"
command = "sigma-notify admin --severity critical"
depends_on = ["collect-evidence"]

[[workflow.incident-response.steps]]
name = "lock-down"
command = "sigma-sec lockdown"
depends_on = ["notify-admin"]
```

---

## 🔧 Custom Workflow Creation

### Workflow Syntax

Workflows are defined in TOML format:

```toml
[workflow.<name>]
trigger = "<trigger-specification>"
description = "<workflow-description>"
enabled = true

[[workflow.<name>.steps]]
name = "<step-name>"
command = "<command-to-execute>"
depends_on = ["<previous-step>"]
condition = "<condition-expression>"
sandbox = true
timeout = 300
retry = 3
```

### Trigger Specifications

| Trigger Type | Format | Example |
|--------------|--------|---------|
| System Event | `system:<event>` | `system:boot` |
| Metric Alert | `metric:<metric> <operator> <value>` | `metric:cpu > 90%` |
| Time-based | `schedule:<cron>` | `schedule:0 0 * * *` |
| User Action | `user:<action>` | `user:file-change` |
| External Event | `external:<event>` | `external:api-call` |

### Condition Expressions

Conditions use a simple expression language:

```toml
condition = "updates_available == true"
condition = "memory_usage > 80%"
condition = "disk_space < 10GB"
condition = "network_status == 'connected'"
```

### Step Options

| Option | Type | Description |
|--------|------|-------------|
| `name` | string | Step identifier |
| `command` | string | Command to execute |
| `depends_on` | array | Steps this depends on |
| `condition` | string | Condition for execution |
| `sandbox` | boolean | Run in sandbox |
| `timeout` | integer | Timeout in seconds |
| `retry` | integer | Number of retries |
| `priority` | string | Execution priority |
| `requires_reboot` | boolean | Requires system reboot |

---

## 📊 Workflow Monitoring

### Execution Status

Monitor workflow execution:

```bash
sigma-workflow list                    # List all workflows
sigma-workflow status <workflow-id>     # Get workflow status
sigma-workflow history <workflow-id>    # View execution history
sigma-workflow logs <execution-id>      # View execution logs
```

### Metrics

Workflow execution metrics:

- **Execution Time**: Time taken to complete workflow
- **Success Rate**: Percentage of successful executions
- **Resource Usage**: CPU, memory, I/O usage
- **Error Rate**: Percentage of failed executions

### Alerts

Configure workflow alerts:

```toml
[workflow.alerts]
on_failure = true
on_success = false
on_timeout = true
notification_method = "email"
notification_endpoint = "admin@example.com"
```

---

## 🔒 Security Considerations

### Capability Requirements

Workflows require appropriate capabilities:

```toml
[workflow.<name>]
capabilities = ["system:write", "network:access"]
```

### Sandbox Isolation

Steps can be sandboxed for security:

```toml
[[workflow.<name>.steps]]
sandbox = true
sandbox_profile = "restricted"
```

### Audit Logging

All workflow executions are logged:

- **Trigger**: What triggered the workflow
- **Steps**: Steps executed and their results
- **Resources**: Resources used
- **Duration**: Execution time
- **User**: User who initiated (if applicable)

---

## 🚦 Best Practices

### Workflow Design

1. **Keep workflows simple**: Complex workflows are harder to debug
2. **Use clear naming**: Descriptive names for workflows and steps
3. **Add conditions**: Prevent unnecessary executions
4. **Set timeouts**: Prevent hanging workflows
5. **Handle failures**: Implement retry logic

### Performance

1. **Parallelize independent steps**: Use DAG structure effectively
2. **Cache results**: Avoid redundant computations
3. **Minimize resource usage**: Use sandbox limits
4. **Monitor performance**: Track execution metrics
5. **Optimize hot paths**: Focus on frequently executed workflows

### Security

1. **Principle of least privilege**: Only grant necessary capabilities
2. **Sandbox untrusted steps**: Isolate potentially dangerous operations
3. **Audit everything**: Log all workflow executions
4. **Review regularly**: Periodically review workflow security
5. **Test thoroughly**: Test workflows before deployment

---

*See also: [AI_AUTOMATION_GATEWAY.md](AI_AUTOMATION_GATEWAY.md) · [AGENTS.md](AGENTS.md) · [sigma-cron](sigma-cron.md) · [System Administration](System-Administration.md)*
