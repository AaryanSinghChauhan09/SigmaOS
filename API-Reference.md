# API Reference

Full reference for the `navigator.sigmaos` API — available in all web apps running inside SigmaOS.

---

## Overview

```javascript
// Check API is available
if (navigator.sigmaos) {
  console.log("SigmaOS version:", navigator.sigmaos.version); // "0.1.0"
}
```

All methods return Promises. Privileged APIs require user permission on first call.

---

## navigator.sigmaos.fs

File access within `~/SigmaOS/` (the user's data directory). Paths are always relative to this root.

| Method | Permission | Returns |
|--------|-----------|---------|
| `readFile(path)` | `fs.read` | `Promise<Uint8Array>` |
| `writeFile(path, data)` | `fs.write` | `Promise<void>` |

### Examples

```javascript
// Write a file
const data = new TextEncoder().encode("Hello SigmaOS!");
await navigator.sigmaos.fs.writeFile("notes/hello.txt", data);

// Read it back
const bytes = await navigator.sigmaos.fs.readFile("notes/hello.txt");
console.log(new TextDecoder().decode(bytes)); // "Hello SigmaOS!"
```

---

## navigator.sigmaos.shell

Run arbitrary CLI tools in a bubblewrap sandbox. Network, filesystem, and binary access are all individually capability-gated.

| Method | Permission | Returns |
|--------|-----------|---------|
| `exec(opts)` | `caps` listed in opts | `Promise<{ok, stdout, stderr, code}>` |

```typescript
interface ShellExecOpts {
  cmd: string;          // e.g. "/usr/bin/ffmpeg"
  args?: string[];      // e.g. ["-i", "pipe:0", ...]
  stdin?: Uint8Array;   // optional stdin bytes
  env?: Record<string, string>;
  caps: string[];       // ["bin:/usr/bin/ffmpeg", "fs:/tmp", "net"]
}
```

### Example: Video Transcoding

```javascript
await navigator.sigmaos.pkg.ensure(["ffmpeg"]);

const input = new Uint8Array(await videoFile.arrayBuffer());
const result = await navigator.sigmaos.shell.exec({
  cmd: "ffmpeg",
  args: ["-i", "pipe:0", "-c:v", "libx264", "-f", "mp4", "pipe:1"],
  stdin: input,
  caps: ["bin:~/.sigmaos/bin/ffmpeg", "fs:/tmp"]
});

if (result.ok) {
  await navigator.sigmaos.fs.writeFile("out.mp4", result.stdout);
}
```

---

## navigator.sigmaos.pkg

Install any Alpine Linux package into `~/.sigmaos/pkg` using `apk` in a user namespace. No root required.

| Method | Permission | Returns |
|--------|-----------|---------|
| `ensure(packages)` | `pkg.install` | `Promise<string[]>` — list of newly installed |
| `list()` | none | `Promise<string[]>` — all installed packages |

```javascript
await navigator.sigmaos.pkg.ensure(["imagemagick", "ffmpeg", "yt-dlp"]);
const installed = await navigator.sigmaos.pkg.list();
```

This replaces the need for SigmaOS to ship `zip`, `pdf`, `ocr`, `git`, `python`, `node`, `sqlite` etc. as built-in APIs. **Everything is `apk add` away.**

---

## navigator.sigmaos.process

| Method | Permission | Returns |
|--------|-----------|---------|
| `spawn(cmd, args)` | `process.spawn` | `Promise<{pid, stdout: ReadableStream}>` |
| `list()` | `process.list` | `Promise<object[]>` |

```javascript
const { pid, stdout } = await navigator.sigmaos.process.spawn("python3", ["-c", "print('hi')"]);
const reader = stdout.getReader();
const { value } = await reader.read();
console.log(value); // "hi\n"
```

---

## navigator.sigmaos.ai

On-device AI via TinyLlama running locally with `llama.cpp`. **No data leaves the device.**

| Method | Permission | Returns |
|--------|-----------|---------|
| `summarize(text, opts?)` | `ai.complete` | `Promise<string>` |
| `complete(prompt, opts?)` | `ai.complete` | `Promise<string>` |

```javascript
const summary = await navigator.sigmaos.ai.summarize(longText, { maxTokens: 100 });
const answer  = await navigator.sigmaos.ai.complete("What is Dijkstra's algorithm?");
```

---

## navigator.sigmaos.workspace

| Method | Permission | Returns |
|--------|-----------|---------|
| `current()` | none | `Promise<Workspace>` |
| `list()` | none | `Promise<Workspace[]>` |
| `create(name)` | `workspace.create` | `Promise<Workspace>` |
| `switch(id)` | `workspace.switch` | `Promise<void>` |

Each workspace maps to an isolated Chromium profile at `~/.sigmaos/workspaces/<id>`. Switching relaunches the browser in that profile.

---

## navigator.sigmaos.system

| Method | Permission | Returns |
|--------|-----------|---------|
| `syncNow()` | `system.sync` | `Promise<{ok, status}>` |
| `setTheme(theme)` | none | `void` |

`setTheme` accepts: `"dark"`, `"light"`, or `"magic"` (matches the color palette of the current page).

---

## navigator.sigmaos.notification

| Method | Permission | Returns |
|--------|-----------|---------|
| `show({title, body})` | none | `Promise<void>` |

```javascript
await navigator.sigmaos.notification.show({
  title: "Done!",
  body: "Your video has been saved to ~/SigmaOS/out.mp4"
});
```
