# SigmaOS Process Monitoring Architecture & Tools Specification

## 1. Overview

Process monitoring in SigmaOS provides complete visibility into execution states, thread hierarchies, resource allocations, seccomp/landlock isolation boundaries, and IPC channels across the operating system.

## 2. Kernel Process Metadata Interfaces (`/proc` & `/sys`)

Process information is exposed through the `/proc` virtual file system:
- **/proc/[pid]/status**: Readable process summary (PID, PPID, UID, GID, state, thread count, memory footprint, capability masks).
- **/proc/[pid]/cmdline**: Full zero-delimited command-line argument vector.
- **/proc/[pid]/statm**: Detailed memory statistics (size, resident, shared, text, data).
- **/proc/[pid]/cgroup**: Active Cgroup v2 resource control paths.
- **/proc/[pid]/fds/**: Open file descriptors, sockets, and anonymous IPC endpoints.
- **/proc/[pid]/io**: Per-process block storage reading and writing byte counters.

## 3. System Administration CLI Utilities

### 3.1 `ps` (Process Status Utility)
Lists active processes with support for BSD and System V flags (`ps aux`, `ps -ef`, `ps -eo pid,user,%cpu,%mem,cmd`).

### 3.2 `btop` / `top` / `htop` (Interactive Process Viewer)
Terminal GUI monitoring tools providing:
- Real-time process listing sorted by CPU usage, memory allocation, or IO rate.
- Process tree visualization (`pstree` view).
- Interactive signal dispatching (`SIGTERM`, `SIGKILL`, `SIGHUP`, `SIGSTOP`).
- Cgroup sandbox quota modification.

```
 SigmaOS Sovereign Monitor (btop)
 Tasks: 184 total, 2 running, 182 sleeping
 CPU [||||||||||||||||||||||||                  ] 42.1%  3.80 GHz
 Mem [|||||||||||||||||||||||||||||||||||       ]  8.4 GB / 16.0 GB
 Swap[||                                        ]  0.1 GB /  4.0 GB

 PID    USER     PR  NI  VIRT   RES   %CPU  %MEM  TIME+     COMMAND
 1240   jules    20   0  1.2G  140M   18.5   0.8  04:12.30  zenith-compositor
 1892   jules    20   0  850M  210M   12.2   1.3  02:45.10  firefox
 1      root     20   0   12M   4M     0.0   0.0  00:01.12  sigma-init
```

### 3.3 `sysinternals-procmon` (Advanced System Tracing)
Low-overhead event tracing tool capturing process creation, file I/O operations, registry/config accesses, and network socket bindings via kernel eBPF tracepoints.

## 4. Programmatic API

```rust
use sigmaos_sysinfo::process::{ProcessManager, ProcessState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pm = ProcessManager::new()?;
    let processes = pm.list_processes()?;

    for p in processes {
        if p.cpu_usage_pct() > 10.0 {
            println!("High CPU Process: PID {} ({}) - {:.1}% CPU", p.pid(), p.name(), p.cpu_usage_pct());
        }
    }
    Ok(())
}
```
