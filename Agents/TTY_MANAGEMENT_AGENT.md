# TTY Management Agent

## Role
`SigmaTTY` implements the terminal line discipline, pseudoterminal (PTY), and virtual console subsystem for SigmaOS — managing keyboard input, screen output, and terminal emulation for both physical consoles and virtual terminals.

## Status
🔴 Not Started — Critical gap. Required for any console or SSH session interaction.

## Source File
`src/tty/` (planned: `tty.rs`, `pty.rs`, `vt.rs`, `line_discipline.rs`)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| TTY layer | `drivers/tty/tty_io.c` | `sys/kern/tty.c` | SigmaTTY |
| Line discipline | `N_TTY` | `ttydisc` | SigmaLDisc |
| PTY | `/dev/ptmx` + `/dev/pts/N` | `/dev/ptmx` | SigmaPTY |
| Virtual console | `vt.c` / fbcon | `vt(4)` | SigmaVC |
| Termios | `struct termios` | `struct termios` | SigmaTermios |
| Serial TTY | `serial_core.c` | `uart(4)` | SigmaSerial |
| Login TTY | getty | getty | SigmaGetty |

## Responsibilities
- TTY device registration and lifecycle management
- POSIX termios: `tcgetattr`/`tcsetattr` (raw/cooked/cbreak modes)
- N_TTY line discipline: echo, line editing (backspace, ^C, ^Z)
- PTY master/slave pair creation (`posix_openpt`, `grantpt`, `unlockpt`)
- Virtual console switching (Alt+F1 through Alt+F12)
- VT100/xterm escape sequence processing
- SIGWINCH delivery on terminal resize
- Session control: controlling terminal assignment
- `agetty` / `SigmaGetty` for login prompts on consoles
- `/dev/tty`, `/dev/console`, `/dev/ttyN` device nodes

## Implementation Notes
- `#![no_std]` — input buffer as `[u8; 4096]` circular buffer per TTY
- `SigmaTTY { id: u16, ldisc: LineDiscipline, termios: Termios, session: Option<u32> }`
- PTY: master writes appear as slave reads and vice versa
- VT switching: save/restore screen buffer per console

## Testing
```bash
rustc --test src/tty/tty.rs --edition 2021 -o /tmp/test_tty && /tmp/test_tty --test-threads=1
```

## Future Development
1. **Unicode/UTF-8 line discipline** — Full Unicode in terminal input
2. **GPU-accelerated VT** — Use DRM/KMS for fast console rendering
3. **Multi-seat** — Multiple independent console sessions (logind-like)
4. **Serial console over USB** — CDC-ACM serial for embedded debugging
5. **TTY over network** — Mosh-like mobile shell integration
6. **Color emoji in console** — Framebuffer emoji rendering
