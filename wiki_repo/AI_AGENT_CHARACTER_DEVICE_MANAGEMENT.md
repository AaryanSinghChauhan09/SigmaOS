# AI Agent Character Device Driver Management Architecture in SigmaOS

This document specifies character device abstractions, TTY/PTY line disciplines, serial UART 8250 drivers, and stream-oriented device guidelines for AI agents working on input/output devices and terminals in SigmaOS (`src/kernel/tty.rs` and `src/kernel/drivers/legacy/uart_8250.rs`).

---

## ⌨️ 1. Character Device & TTY/PTY Subsystem Architecture

Character devices in SigmaOS process byte streams sequentially without block addressing:

```
+---------------------------------------------------------------------------------+
| Hardware: 16550/8250 UART (`src/kernel/drivers/legacy/uart_8250.rs`)            |
| Receives raw serial interrupts & character bytes from COM1 / serial ports.      |
+---------------------------------------------------------------------------------+
                                       |
                                       v
+---------------------------------------------------------------------------------+
| Line Discipline & Termios Layer: `Tty` (`src/kernel/tty.rs`)                    |
| Handles POSIX line editing (canonical vs raw mode), signal translation (SIGINT,  |
| SIGQUIT, SIGTSTP), flow control (XON/XOFF), and character echo buffers.         |
+---------------------------------------------------------------------------------+
                                       |
                                       v
+---------------------------------------------------------------------------------+
| Userland Stream Interface: `sys_read` / `sys_write` / `ioctl`                   |
| POSIX termios control (`TCGETS`, `TCSETS`, `TIOCGWINSZ`, `TIOCSWINSZ`).          |
+---------------------------------------------------------------------------------+
```

---

## ⚙️ 2. Line Discipline & Termios Flag Operations

1. **Input Processing (`c_iflag`)**
   - `ICRNL`: Map carriage return (`\r`) to newline (`\n`).
   - `IXON` / `IXOFF`: Software flow control via `VSTART` (Ctrl-Q, `0x11`) and `VSTOP` (Ctrl-S, `0x13`).
2. **Local Line Editing (`c_lflag`)**
   - `ICANON`: Canonical mode line editing using `canonical_buffer`. Line is committed to `input_buffer` on `\n` or `VEOF` (Ctrl-D).
   - `ISIG`: Signal generation on control characters (`VINTR` -> SIGINT, `VQUIT` -> SIGQUIT, `VSUSP` -> SIGTSTP).
   - `ECHO` / `ECHOE` / `ECHOK`: Character and erasure visual feedback.
3. **Output Translation (`c_oflag`)**
   - `OPOST` & `ONLCR`: Translate `\n` to `\r\n` sequence on output.

---

## 🛡️ 3. Rules & Directives for AI Agents

1. **Non-Blocking Stream Processing**
   - Character device `read()` operations should drain available ring buffer bytes up to `buf.len()` without blocking if data is available.
2. **Signal Handling Safety**
   - When `ISIG` is set in termios, interception of `VINTR` (`Ctrl-C`), `VQUIT` (`Ctrl-\`), or `VSUSP` (`Ctrl-Z`) MUST flush the canonical input buffer (`flush_input()`) to prevent input pollution.
3. **Hardware Lock-Free FIFO Access**
   - UART hardware interrupts and ring buffers MUST avoid dynamic heap allocations during high-frequency byte transfers.

---

## ⚙️ 4. Verification Commands for Character Device Agents

- **TTY Unit Tests:**
  `cargo test --lib -- kernel::tty::tests`
- **UART Driver Build Verification:**
  `rustc --test src/kernel/drivers/legacy/uart_8250.rs --edition=2021 -o build/uart_test && ./build/uart_test`
- **Full SigmaOS Pipeline:**
  `./run_sigma_tests.sh`
