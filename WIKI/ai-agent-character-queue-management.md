# AI Agent Character Queue & TTY Terminal Stream Management in SigmaOS

## Overview

SigmaOS terminal and character stream subsystem (`src/kernel/tty.rs`, `src/productivity/terminal.rs`, `src/shell/sigma_sh.rs`) handles TTY character queues, raw vs canonical line editing modes, ANSI escape sequence tokenization, software flow control (`XON`/`XOFF`), and asynchronous character ring buffers.

AI agents (such as Jules, Herdr agentic terminal sessions, REPL shell runners, and CLI command dispatchers) must follow character queue management guidelines when streaming input and output characters.

---

## TTY Character Queue Architecture

```
Agent Character Output → Raw Input Ring Buffer (`c_ibuf`)
                                 │
                     ┌───────────┴───────────┐
                     ▼                       ▼
            Canonical Line Mode         Raw Line Mode
           (Line Buffer & Echo)      (Instant Key Event)
                     │                       │
                     └───────────┬───────────┘
                                 ▼
                     Output Ring Buffer (`c_obuf`)
                                 │
                                 ▼
                 ANSI / VT100 Terminal Escape Parser
```

---

## 1. Raw vs Canonical Mode Operation

AI agents interacting with terminal sessions choose between canonical (cooked) and raw modes based on task requirements:

| Mode | Behavior | AI Agent Use Case |
|------|----------|-------------------|
| **Canonical Mode** | Character input buffered until `\n` or `EOF`. Echo & line editing (`backspace`, `ctrl+u`) enabled. | REPL commands (`sigma-sh`), batch shell scripts |
| **Raw Mode** | Input characters available immediately without newline buffering. Echo disabled. | Full-screen terminal apps (`nvim`, `top`, `tmux`), interactive prompts |

```rust
use sigmaos::terminal::TerminalSession;

let mut term = TerminalSession::new();

// Set TTY into raw mode for instant character-by-character processing
term.set_raw_mode(true)?;

// Process individual character from input queue
if let Some(ch) = term.pop_input_char() {
    println!("Received character byte: 0x{:02x}", ch);
}
```

---

## 2. Software Flow Control (`XON` / `XOFF`) & Watermarks

To prevent TTY character queue overflow (`c_ibuf` capacity exhaustion), SigmaOS enforces software flow control:

- **`XOFF` (`0x13` / `Ctrl+S`)**: Sent to producer when character queue reaches `WatermarkHigh` (80% full). Production pauses.
- **`XON` (`0x11` / `Ctrl+Q`)**: Sent to producer when character queue drains below `WatermarkLow` (20% full). Production resumes.

---

## 3. ANSI / VT100 Escape Sequence Tokenization

AI agents parsing terminal character queues handle 24-bit ANSI color and cursor positioning sequences:

```rust
use sigmaos::shell::ZshPromptFormatter;

// Example ANSI color sequence tokenization
let colorized_prompt = "\x1B[32msovereign@sigmaos\x1B[0m:$ ";
```

---

## Directives for AI Agents

1. **Flush Output Queues**: Always flush character output queues (`c_obuf`) before expecting user or subagent input responses.
2. **Handle Backspace & EOF**: In canonical mode, handle `0x08` (`BS`), `0x7F` (`DEL`), and `0x04` (`EOF`) appropriately.
3. **Restore TTY State**: Always restore original termios settings upon exiting interactive subagent terminal sessions.
