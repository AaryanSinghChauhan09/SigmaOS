# SigmaOS API Reference

## navigator.sigmaos.* — Browser Shell API

The browser profile exposes native system primitives to PWAs via `navigator.sigmaos.*`. All calls are capability-gated — the app must declare required permissions in its `sigma.manifest.json`.

---

### Process Management

```js
// Spawn a subprocess
const proc = await navigator.sigmaos.spawn("/usr/bin/ls", ["-la", "/home"]);
proc.stdout.on("data", chunk => console.log(chunk));
await proc.wait(); // resolves with exit code

// Pipe two processes
const ls = await navigator.sigmaos.spawn("ls", ["-la"]);
const grep = await navigator.sigmaos.spawn("grep", [".md"]);
ls.stdout.pipe(grep.stdin);
```

### Filesystem (`rpath` / `wpath` capability required)

```js
// Read a file
const data = await navigator.sigmaos.readFile("/home/user/doc.txt", "utf8");

// Write a file
await navigator.sigmaos.writeFile("/home/user/out.txt", "hello sigma\n");

// List directory
const entries = await navigator.sigmaos.readdir("/home/user");

// File watcher
const watcher = navigator.sigmaos.watch("/home/user/docs");
watcher.on("change", (event, filename) => console.log(filename, "changed"));
```

### Memory Mapping (`mmap` capability required)

```js
// Map a shared memory region
const shm = await navigator.sigmaos.mmap({
    size: 4096,
    prot: "rw",
    flags: "shared",
    name: "my-shm"
});
const view = new Uint8Array(shm.buffer);
```

### /dev Access (`dev` capability required)

```js
// Open a device node
const cam = await navigator.sigmaos.openDevice("/dev/video0");
const frame = await cam.read(1280 * 720 * 3);
```

### System Info

```js
// CPU + memory stats
const stats = await navigator.sigmaos.sysinfo();
console.log(stats.cpuCount, stats.totalRam, stats.freeRam);

// Kernel version
const ver = await navigator.sigmaos.uname();
console.log(ver.sysname, ver.release); // "SigmaOS" "16.0.0-Apex"
```

---

## sigma-pkg CLI

```bash
# Install a package
sigma-pkg install firefox@latest

# Remove a package
sigma-pkg remove firefox

# Update all packages
sigma-pkg update

# Delta update (incremental)
sigma-pkg update --delta

# Search packages
sigma-pkg search "text editor"

# Show package info
sigma-pkg info neovim

# List installed
sigma-pkg list

# Reproduce-verify a build
sigma-pkg verify firefox
```

---

## sigma-cli System Tools

```bash
# System status
sigma status

# Top-like process viewer
sigma-top

# Network monitor
sigma-net status
sigma-net dns set 1.1.1.1

# Package manager shorthand
sigma install firefox
sigma remove firefox

# Profile switcher
sigma profile set developer
sigma profile list

# Snapshot management
sigma snapshot create my-backup
sigma snapshot restore my-backup
sigma snapshot diff my-backup HEAD
```

---

## sigma-bus IPC (C++ API)

```cpp
#include "include/SovereignIPC.h"

// Server side
auto server = sigma_bus_listen("my.service");
while (auto req = server->accept()) {
    auto token = sigma_cap_verify(req->capability, "my.service/read");
    if (!token) { req->deny(); continue; }
    req->reply(handle_request(req->payload));
}

// Client side
auto conn = sigma_bus_connect("my.service");
auto cap = sigma_cap_request("my.service/read");  // from sigma-trustd
auto resp = conn->call(cap, payload);
```

---

## App Manifest (`sigma.manifest.json`)

```json
{
  "name": "My SigmaOS App",
  "version": "1.0.0",
  "entry": "index.html",
  "capabilities": ["rpath", "inet", "stdio"],
  "unveil": [
    { "path": "/home/$USER/Documents", "perm": "rw" },
    { "path": "/usr/share/fonts", "perm": "r" }
  ],
  "sigma_bus": ["my.service"],
  "min_sigmaos": "15.0.0"
}
```

---

*See also: [App-Manifest](App-Manifest) · [Your-First-App](Your-First-App) · [Security-Model](Security-Model)*
