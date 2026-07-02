# SigmaOS SDK Guide

> Build sovereign, AI-native apps for SigmaOS.
> Everything you need to go from idea to published .sigpkg.

---

## Overview

The SigmaOS SDK lets you build applications that integrate natively with:
- sigma-agent (AI CLI assistant)
- sigma-bus (typed IPC)
- sigma_pledge / sigma_unveil (security)
- sigma-pkg (package distribution)
- Zenith Desktop compositor
- sigma-ai daemon (local LLM inference)

**Languages:** Rust (primary), Nim (CLI tools), Zig (HAL/drivers), Ada/SPARK (security-critical)

---

## Quick Start

```bash
# Install SDK
sigma-pkg install sigma-sdk

# Scaffold a new app
sigma-sdk new my-app --lang rust
cd my-app

# Build
cargo build --release

# Package
sigma-sdk package --name my-app --version 0.1.0

# Test locally
sigma-pkg install my-app-0.1.0.sigpkg

# Publish
sigma-pkg publish my-app-0.1.0.sigpkg
```

---

## App Manifest (sigma-manifest.toml)

Every SigmaOS app needs a manifest:

```toml
[app]
name        = "my-app"
version     = "0.1.0"
author      = "Your Name"
description = "A sovereign SigmaOS application"
license     = "MIT"
arch        = ["x86_64", "aarch64"]

[security]
# sigma_pledge capabilities (required — be minimal)
pledge      = ["stdio", "rpath", "wpath", "inet"]
# sigma_unveil paths (required — be minimal)
unveil      = [
  { path = "/home", perms = "r" },
  { path = "/tmp",  perms = "rw" },
]

[dependencies]
sigma-libc  = ">=1.0"
sigma-tls   = ">=2.0"

[integration]
# Optional: register with sigma-agent as a tool
agent_tool  = true
agent_cmds  = ["my-app do-thing", "my-app show"]

# Optional: sigma-bus IPC channels
bus_channels = ["BUS_MYAPP"]
```

---

## Registering with sigma-agent

Make your app accessible via natural language:

```rust
// In your app: register as a sigma-agent tool
// src/sigma_tool.rs

use std::collections::BTreeMap;

/// Called by sigma-agent when user says "my-app do-thing <input>"
#[no_mangle]
pub extern "C" fn sigma_tool_execute(
    args_json: *const u8,
    args_len:  usize,
) -> *mut u8 {
    let args_str = unsafe {
        std::str::from_utf8(std::slice::from_raw_parts(args_json, args_len))
            .unwrap_or("")
    };
    // Parse args_json, do work, return JSON result
    let result = r#"{"success":true,"output":"Done!"}"#;
    let boxed = Box::new(result.as_bytes().to_vec());
    Box::into_raw(boxed) as *mut u8
}
```

Then declare in manifest:
```toml
[integration]
agent_tool = true
agent_cmds = ["my-app do-thing"]
# User can then say: sigma-agent "my-app do-thing <input>"
```

---

## Using sigma-ai in Your App

Call the local LLM daemon from your app:

```rust
// Connect to sigma-ai daemon via Unix socket
use std::os::unix::net::UnixStream;
use std::io::{Read, Write};

fn ask_sigma_ai(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = UnixStream::connect("/run/sigma/ai.sock")?;
    let request = format!(
        r#"{{"messages":[{{"role":"user","content":"{}"}}],"max_tokens":256}}"#,
        prompt.replace('"', "\\\"")
    );
    write!(stream, "{}\n", request)?;
    stream.flush()?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    // Parse {"content": "..."}
    if let Some(start) = response.find("\"content\":\"") {
        let rest = &response[start + 11..];
        if let Some(end) = rest.find("\"") {
            return Ok(rest[..end].replace("\\n", "\n"));
        }
    }
    Ok(response)
}
```

Or via HTTP (when daemon is running):
```bash
curl -X POST http://localhost:11430/v1/chat \
     -d '{"message":"explain this error: segfault in my-app"}'
```

---

## sigma-bus IPC

Communicate between shards and apps:

```rust
// Declare your channel in manifest, then use sigma-bus API
extern "C" {
    fn sigma_bus_send(channel: u32, data: *const u8, len: usize) -> i32;
    fn sigma_bus_recv(channel: u32, buf: *mut u8, buf_len: usize, timeout_ms: u32) -> i32;
}

const BUS_MYAPP: u32 = 0x4D59_4150;  // "MYAP" in hex

unsafe {
    let msg = b"hello from my-app";
    sigma_bus_send(BUS_MYAPP, msg.as_ptr(), msg.len());
}
```

---

## sigma_pledge / sigma_unveil

All SigmaOS apps must declare their capabilities:

```rust
// Call at startup, before opening any files or network connections
extern "C" {
    fn sigma_pledge(promises: *const u8, len: usize) -> i32;
    fn sigma_unveil(path: *const u8, path_len: usize,
                    perms: *const u8, perms_len: usize) -> i32;
}

fn restrict_capabilities() {
    unsafe {
        // Only allow file reads, stdio, and internet
        let promises = b"stdio rpath inet";
        sigma_pledge(promises.as_ptr(), promises.len());

        // Only allow reading /home/user/documents
        let path = b"/home/user/documents\0";
        let perms = b"r\0";
        sigma_unveil(path.as_ptr(), path.len(), perms.as_ptr(), perms.len());
    }
}
```

---

## Packaging Your App

```bash
# Build release binary
cargo build --release

# Create package structure
mkdir -p dist/usr/bin dist/usr/share/my-app
cp target/release/my-app dist/usr/bin/
cp -r assets/ dist/usr/share/my-app/

# Create package
sigma-sdk package \
  --name my-app \
  --version 0.1.0 \
  --manifest sigma-manifest.toml \
  --source dist/ \
  --output my-app-0.1.0.sigpkg

# Sign the package (Dilithium-5)
sigma-pkg sign my-app-0.1.0.sigpkg --key ~/.config/sigma/signing.key

# Verify
sigma-pkg verify my-app-0.1.0.sigpkg
```

---

## Publishing to sigma_pkg_registry

```bash
# Create a recipe file
sigma-sdk recipe create my-app
# Edit sigma_pkg_registry/recipes/my-app.toml

# Submit via PR to github.com/AaryanSinghChauhan09/SigmaOS
# File: sigma_pkg_registry/recipes/my-app.toml
```

Recipe format:
```toml
[package]
name     = "my-app"
version  = "0.1.0"
homepage = "https://github.com/you/my-app"
source   = "https://github.com/you/my-app/releases/download/v0.1.0/my-app-0.1.0.sigpkg"
sha256   = "abc123..."
sig_dilithium5 = "..."
license  = "MIT"
```

---

## Nim CLI Tools

Tools and daemons are written in Nim:

```nim
# my-tool.nim
import std/[os, osproc, strformat]

proc main() =
  echo "My SigmaOS tool v0.1.0"
  let cmd = paramStr(1)
  case cmd
  of "hello": echo "Hello from my-tool!"
  else: echo fmt"Unknown command: {cmd}"

main()
```

```bash
# Build
nim c -d:release --opt:speed -o:my-tool my-tool.nim

# Package
sigma-sdk package --name my-tool --binary my-tool
```

---

## CI Integration

Add to `.github/workflows/`:

```yaml
name: Build my-app
on: [push, pull_request]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with: {toolchain: stable}
      - run: cargo build --release
      - run: cargo test
      # Package for SigmaOS
      - run: |
          sigma-pkg install sigma-sdk || true
          sigma-sdk package --name my-app --version 0.1.0 || true
```

---

## Examples

| Example | Description | Location |
|---|---|---|
| `sigma-edit` | Text editor | `sdk/examples/sigma-edit/` |
| `sigma-calc` | Calculator | `sdk/examples/sigma-calc/` |
| `sigma-files` | File manager | `sdk/examples/sigma-files/` |
| Hello World | Minimal app | `sdk/examples/hello-world/` |
| sigma-agent plugin | Plugin example | `userland/agent/` |

---

*See also: [sigma-pkg Spec](sigpkg-Spec) · [App Manifest](App-Manifest) · [Architecture Overview](Architecture-Overview) · [Security Model](Security-Model)*
