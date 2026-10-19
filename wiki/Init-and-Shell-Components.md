# Init and Shell Components

## Init Supervisor

### Overview
PID 1 init supervisor following BSD-inspired patterns with dependency DAG and service management.

### Features
- Service registration and lifecycle management
- Dependency DAG topological sort
- Start/stop order enforcement
- Restart policy support (Never, OnFailure, Always)

### Status
- **Implementation**: Planned
- **Dependencies**: VFS, shell, package manager
- **Roadmap Phase**: Init and shell (P1)

### Related Files
- `src/init/supervisor.rs` - Implementation
- `FEATURE_STATUS_EXPANSION.toml` - Status tracking
- `docs/INIT_SUPERVISOR_PLAN.md` - Detailed plan

## Shell Subsystem

### Overview
Basic shell command parsing and execution following SigmaOS principles.

### Features
- Basic command parsing
- Pipeline support (|, <, >)
- Built-in commands (cd, pwd, echo, exit)
- Command history

### Status
- **Implementation**: Planned
- **Dependencies**: Init supervisor, VFS
- **Roadmap Phase**: Init and shell (P1)

### Related Files
- Shell command structures and parsing logic

### Principles Applied
- **COMMAND_QUERY_SEPARATION**: Parsing vs execution separation
- **SINGLE_RESPONSIBILITY**: Shell has one clear purpose
- **OPEN_CLOSED**: Extensible via built-in commands
- **ZERO_ONE_INFINITY**: Controlled command set
