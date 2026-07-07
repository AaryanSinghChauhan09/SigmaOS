# SigmaOS Workflow Orchestrator

> AI-powered DAG-based workflow automation engine.

## Overview

The Workflow Orchestrator enables users to define multi-step automated workflows as **Directed Acyclic Graphs (DAGs)**. Tasks are executed in dependency order with support for AI inference steps, container runs, and automatic retry logic.

## Workflow Task Types

| Task Kind       | Description                              | Example                        |
|-----------------|------------------------------------------|--------------------------------|
| ShellCommand    | Run a shell command in sandbox           | `sigma-pkg update`             |
| AiInference     | Query the local AI engine                | "Summarize today's logs"       |
| FileTransform   | Apply jq/sed/awk transformations         | Parse JSON config files        |
| NetworkFetch    | HTTP GET/POST with TLS                   | Download latest threat feeds   |
| ContainerRun    | Launch OCI container                     | Run CI/CD test suite           |
| Checkpoint      | No-op marker for synchronization         | Gate between phases            |

## DAG Execution

```
[Fetch Data] ──→ [AI Analysis] ──→ [Generate Report]
      │                                    ↑
      └──→ [Transform Logs] ──────────────┘
```

- Tasks execute in **topological order** (Kahn's algorithm)
- Cycle detection prevents infinite loops
- Failed tasks trigger **retry logic** (configurable max retries + delay)
- Skipped tasks propagate when dependencies fail

## Implementation

- **Source**: `ai/sigma_workflow_orchestrator.rs`
- **Language**: Rust (`no_std`)
- **Key APIs**:
  - `topological_sort(tasks)` — DAG validation and ordering
  - `execute_workflow(workflow)` — full pipeline execution
  - Task dispatch to shell, AI, file, network, or container executors

## Error Handling

| Error             | Cause                               | Recovery                    |
|-------------------|-------------------------------------|-----------------------------|
| `DagCycle`        | Circular dependency                 | Fix workflow definition     |
| `TaskFailed(id)`  | Task exhausted retry budget         | Check task command/config   |
| `Timeout(id)`     | Task exceeded deadline              | Increase timeout            |
| `DependencyFailed`| Upstream task failed                | Fix upstream first          |
