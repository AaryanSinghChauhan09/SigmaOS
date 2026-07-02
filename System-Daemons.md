# ⚙️ System Daemons & Utilities

SigmaOS implements essential background daemons and advanced utilities natively, with zero dependence on the POSIX ecosystem. This ensures maximum control, minimum footprint, and tight security.

## Background Daemons

### `sigma_init`
The root process of the system (PID 1). It mounts all necessary virtual filesystems (`/dev`, `/proc`, `/sys`), forks foundational background daemons (`cron`, `syslog`), drops the user into an interactive `sigma-sh` shell, and functions as an infinite loop to reap zombie child processes via `waitpid`.

### `sigma_voice_control`
NLP-driven intent execution for Voice-First OS control.

### `sigma_p2p_update`
Decentralized update daemon distributing `.spkg` via mesh.

### `sigma_cron`
A zero-dependency scheduling daemon absorbing Vixie cron functionality.
- Uses an in-memory `crontab` structure containing `{minute, hour, command}` combinations.
- Uses `sigma_sys_sleep` to efficiently idle the CPU between time-checks, and `sigma_sys_fork`/`sigma_sys_execve` to execute scheduled scripts in the background.

### `sigma_syslog`
A centralized logging daemon (not yet fully fleshed out but conceptually integrated) to intercept kernel buffer outputs and `stdout` multiplexing.

## Archiving & Compression

### `sigma_gzip`
A bare-metal, pure-C++ implementation of DEFLATE compression.
- Completely free of external `zlib` dependencies.
- Operates directly on the virtual FAT32 filesystem by reading files into 4KB chunks and writing out standard `.gz` headers (Magic Bytes `0x1F 0x8B`).

## Networking & Editors

### `sigma_nc`
A sovereign network tool mimicking standard `netcat`. Allows arbitrary TCP connections and simple listeners on specified ports, binding directly to `sigma_tcp_connect` and `sigma_tcp_listen` syscall equivalents.

### `sigma_vi`
A terminal-based modal text editor, avoiding massive dependencies like `ncurses`. Uses bare-metal VGA escapes to implement pure `INSERT` and `NORMAL` modes. Allows entering keystrokes to a buffer and features commands like `:w` and `:q`.
