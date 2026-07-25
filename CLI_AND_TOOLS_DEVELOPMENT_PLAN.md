# 💻 SigmaOS CLI Shell & System Tools Development Plan

This document details the architectural design and implementation plan for the **SigmaOS CLI Shell Utilities and Terminal Tools**, mapping all graphical user interface (GUI) tasks into powerful, fast, and secure CLI equivalents.

By using **zero-dependency** low-level components, this plan ensures that tasks normally requiring window managers (such as taking screenshots, monitoring threads, modifying themes, starting container sandboxes, or searching encrypted directories) are executed with maximum performance directly from the `sigma-sh` terminal.

---

## 🗺️ Architectural Inspiration
*   **BusyBox / Coreutils:** Combines lightweight versions of many common system utilities into a single multi-call binary, optimizing disk space and cache locality.
*   **nushell:** Employs a structured, typed pipeline model where commands pass data-blocks and tabular records instead of raw, unparsed string sequences.

---

## 🏗️ OOP Design & Unified Command Pipeline

SigmaOS models terminal execution using an object-oriented command parser, history buffer, and environment registry. Every CLI command must inherit from a polymorphic base and enforce strict capability restrictions.

```text
                               +-----------------------------+
                               |      Sigma Shell REPL       |
                               +-----------------------------+
                                              |
                   +--------------------------+--------------------------+
                   v                                                     v
      +-------------------------+                           +-------------------------+
      |  System Management CLI  |                           |  Visual & Desktop CLI   |
      +-------------------------+                           +-------------------------+
       - `sigmapkg` (Packages)                               - `screencap` (Screenshots)
       - `sigmacont` (Containers)                            - `themer` (Visual settings)
```

### CLI Execution Transitions:
```text
  PromptWaiting ➡️ CommandParsing ➡️ SecurityPledgeCheck ➡️ Running ➡️ ResultFlushed
```

### Polymorphic CLI Command Interface:
```rust
pub trait ShellCommand {
    fn name(&self) -> &[u8];
    fn execute(&mut self, args: &[&[u8]]) -> Result<(), ShellError>;
    fn help(&self) -> &[u8];
}
```

---

## ⚙️ GUI Tasks to CLI Command Mapping Matrix

| GUI Application | CLI Command Equivalent | Sub-Command Parameters & Flags |
| :--- | :--- | :--- |
| **Zenith Desktop (Theme)** | `themer` | `themer set [dark/light/classic]` \| `themer accent [color_hex]` |
| **System Monitor** | `sigmatop` | `sigmatop --threads` \| `sigmatop --mem` \| `sigmatop --interval [ms]` |
| **Container Manager** | `sigmacont` | `sigmacont run [image_id]` \| `sigmacont stop [container_id]` \| `sigmacont list` |
| **Screen Recorder / Capture** | `screencap` | `screencap --full [output_path.png]` \| `screencap --rect [x,y,w,h]` |
| **Unified Package Manager** | `sigmapkg` | `sigmapkg install [recipe]` \| `sigmapkg rollback [snapshot_hash]` |
| **File Manager** | `sigmasfs` | `sigmasfs encrypt [path] [key_hash]` \| `sigmasfs snapshot [volume_id]` |

---

## 🛠️ Multi-Language Architecture & Executable Code

To maximize execution speeds and keep the static binary footprint < 100KB, core CLI parsers are implemented across Rust, Zig, and Nim.

### ⚡ Rust: Multi-Call Command Dispatcher (`sigma-sh`)
```rust
// Multi-call dispatch parser for system commands
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellError {
    Success,
    CommandNotFound,
    InvalidArgument,
    PermissionDenied,
}

pub struct SimpleCliDispatcher {
    pub active_user: String,
}

impl SimpleCliDispatcher {
    pub fn new(user: String) -> Self {
        Self { active_user: user }
    }

    pub fn dispatch_screencap(&self, args: &[&str]) -> Result<(), ShellError> {
        if args.is_empty() {
            return Err(ShellError::InvalidArgument);
        }

        let flag = args[0];
        match flag {
            "--full" => {
                println!("[CLI Screencap]: Captured full hardware buffer. Frame saved to /var/screenshots/.");
                Ok(())
            }
            "--rect" if args.len() >= 2 => {
                println!("[CLI Screencap]: Captured custom sub-region coordinates: {}.", args[1]);
                Ok(())
            }
            _ => Err(ShellError::InvalidArgument),
        }
    }

    pub fn dispatch_sigmacont(&self, args: &[&str]) -> Result<(), ShellError> {
        if args.len() < 2 {
            return Err(ShellError::InvalidArgument);
        }
        let action = args[0];
        let target = args[1];

        match action {
            "run" => {
                println!("[CLI Container]: Initialized OCI container sandbox mapping image '{}'.", target);
                Ok(())
            }
            "stop" => {
                println!("[CLI Container]: Stopped container workspace ID '{}'.", target);
                Ok(())
            }
            _ => Err(ShellError::InvalidArgument),
        }
    }
}
```

### ⚡ Zig: High-Performance Tab Auto-Completer
```zig
const std = @import("std");

pub const AutoCompleter = struct {
    commands: []const []const u8,

    pub fn suggestMatch(self: AutoCompleter, input: []const u8) ?[]const u8 {
        // Linear fast search over registered commands to support terminal autocomplete
        for (self.commands) |cmd| {
            if (std.mem.startsWith(u8, cmd, input)) {
                return cmd;
            }
        }
        return null;
    }
};
```

### ⚡ Nim: Structured Shell Pipeline Tokenizer
```nim
type
  PipelineCommand* = object
    command*: string
    args*: seq[string]

proc tokenizePipeline*(rawInput: string): seq[PipelineCommand] {.exportc, cdecl.} =
  # Tokenize structured shell pipelines (e.g. "sigmatop | grep net")
  result = @[]
  # Parser split logic goes here...
  result.add(PipelineCommand(command: "sigmatop", args: @[]))
```

---

## 📈 Quality Assurance & PnP Validation

1.  **Unified Integration Test:** Verify that executing `themer set dark` changes the active memory system visual parameters instantly.
2.  **No-Allocation Shell Benchmark:** Verify that parsing and executing basic shell commands (like `help` or `clear`) incurs absolutely zero dynamic heap allocation in the microkernel space.
