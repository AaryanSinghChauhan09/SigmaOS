# Sovereign Shell (`sigma-sh`)

`sigma-sh` is the minimal, kernel-mode interactive shell for SigmaOS. It provides debugging, recovery, and administrative access before full userland is loaded.

## Command Reference

| Command | Description | Privilege |
|---------|-------------|-----------|
| `help` | List available commands | All |
| `ps` | List active processes | All |
| `kill <pid>` | Terminate a process by PID | KERNEL |
| `devices` | List hardware device tree | All |
| `services` | List init system services | All |
| `ipc` | Show IPC status (queues/shm) | All |
| `clear` | Clear terminal output | All |
| `echo <text>` | Print text to standard output | All |
| `sandbox` | Show Sovereign Sandbox audit log | All |
| `reboot` | Restart the system | KERNEL |

## Execution Model

The shell features:
- Space-delimited tokenization (up to 16 arguments).
- An 8-command history ring buffer.
- Real-time command registry allowing dynamic addition of commands by kernel modules.
