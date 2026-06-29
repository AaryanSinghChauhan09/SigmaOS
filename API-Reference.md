# navigator.sigmaos API Reference

All SigmaOS platform APIs are exposed under the `navigator.sigmaos` namespace and are available to any PWA or web app running inside the SigmaOS shell, provided the required capability is declared in the app's `manifest.json`.

---

## navigator.sigmaos.process

### `.spawn(options)`

Spawns a native process inside a bubblewrap sandbox and streams its output.

**Requires capability**: `process:spawn`

```js
const proc = await navigator.sigmaos.process.spawn({
  cmd: "ffmpeg",
  args: ["-i", "input.mp4", "-vcodec", "h264", "output.mp4"],
  caps: [
    "bin:ffmpeg",
    "fs:/home/user/Videos",
    "fs:/tmp"
  ]
});

// Stream stdout in real time
for await (const chunk of proc.stdout) {
  console.log(chunk);
}

const exitCode = await proc.wait();
```

**Options:**

| Field | Type | Description |
|---|---|---|
| `cmd` | `string` | Binary name to execute |
| `args` | `string[]` | Argument list |
| `caps` | `string[]` | Per-invocation capability overrides |
| `stdin` | `Uint8Array \| ReadableStream` | Optional stdin data |
| `cwd` | `string` | Working directory inside the sandbox |
| `env` | `Record<string, string>` | Additional environment variables |

**Returns**: `SigmaProcess` object with `stdout`, `stderr` (async iterables), and `wait()` (resolves to exit code).

---

### `.kill(pid, signal?)`

Sends a signal to a running process.

```js
await navigator.sigmaos.process.kill(proc.pid, "SIGTERM");
```

---

## navigator.sigmaos.fs

### `.read(path)`

Reads a file and returns its contents as a `Uint8Array`.

**Requires capability**: `fs:<path>` covering the target path.

```js
const bytes = await navigator.sigmaos.fs.read("/home/user/notes.md");
const text = new TextDecoder().decode(bytes);
```

---

### `.write(path, data)`

Writes data to a file, creating it if it doesn't exist.

```js
await navigator.sigmaos.fs.write("/home/user/notes.md", new TextEncoder().encode("Hello"));
```

---

### `.readdir(path)`

Returns a list of directory entries.

```js
const entries = await navigator.sigmaos.fs.readdir("/home/user");
// [{ name: "notes.md", type: "file", size: 1024 }, ...]
```

---

### `.watch(path, callback)`

Watches a path for changes (uses `inotify` under the hood).

```js
const unwatch = await navigator.sigmaos.fs.watch("/home/user", (event) => {
  console.log(event.type, event.path); // "modify" "/home/user/notes.md"
});

// Stop watching
unwatch();
```

---

## navigator.sigmaos.pkg

### `.ensure(packages)`

Ensures one or more Alpine packages are installed in the user's home namespace. Downloads and installs them on first call; no-ops if already present.

**Requires capability**: `process:spawn` (package manager runs in a sandboxed namespace)

```js
await navigator.sigmaos.pkg.ensure(["ffmpeg", "imagemagick"]);

// Now ffmpeg is available at ~/.sigmaos/bin/ffmpeg
const result = await navigator.sigmaos.process.spawn({
  cmd: "ffmpeg",
  args: ["-version"],
  caps: ["bin:~/.sigmaos/bin/ffmpeg"]
});
```

---

### `.list()`

Returns all currently installed packages.

```js
const packages = await navigator.sigmaos.pkg.list();
// [{ name: "ffmpeg", version: "6.0-r1", size: "18.2 MB" }, ...]
```

---

## navigator.sigmaos.window

### `.create(options)`

Creates a native frameless floating window (WebKit BrowserWindow-style) that renders a URL outside the normal tab chrome.

```js
const win = await navigator.sigmaos.window.create({
  url: "https://app.sigmaos/sigma-code",
  width: 1024,
  height: 768,
  title: "SigmaCode",
  resizable: true,
  alwaysOnTop: false
});
```

**Options:**

| Field | Type | Default | Description |
|---|---|---|---|
| `url` | `string` | — | URL to load in the window |
| `width` | `number` | `800` | Initial width in pixels |
| `height` | `number` | `600` | Initial height in pixels |
| `title` | `string` | `""` | Window title bar text |
| `resizable` | `boolean` | `true` | Whether the user can resize the window |
| `alwaysOnTop` | `boolean` | `false` | Pin the window above all others |
| `frameless` | `boolean` | `true` | Hide the OS window chrome |

**Returns**: `SigmaWindow` with `.close()`, `.focus()`, `.resize(w, h)`, `.move(x, y)`.

---

## navigator.sigmaos.notification

### `.show(options)`

Displays a notification in the Notification Center.

```js
await navigator.sigmaos.notification.show({
  title: "Sync Complete",
  body: "Your files have been synced to Google Drive.",
  icon: "/app-icons/sync.png",
  actions: [
    { label: "View Files", action: "open_files" }
  ]
});
```

---

## navigator.sigmaos.ai

### `.complete(prompt, options?)`

Sends a completion request to the local `sigmad-ai` daemon (TinyLlama on port 17392).

**Requires capability**: `ai:complete`

```js
const response = await navigator.sigmaos.ai.complete(
  "Summarize this document:\n\n" + documentText,
  { maxTokens: 256, temperature: 0.7 }
);

console.log(response.text);
```

---

### `.predict(payload)`

Sends a raw prediction request to `localhost:17392/v1/predict`.

```js
const prediction = await navigator.sigmaos.ai.predict({
  model: "sigma-ui-v1",
  features: contextVector
});
```

---

## navigator.sigmaos.clipboard

### `.read()`

Reads the current clipboard contents from `sigmad-clipboard`.

**Requires capability**: `clipboard:read`

```js
const content = await navigator.sigmaos.clipboard.read();
// { type: "text/plain", data: "Hello world" }
// or { type: "image/png", data: Uint8Array }
```

---

### `.write(content)`

Writes to the shared clipboard. All registered apps receive a `clipboard-updated` event.

**Requires capability**: `clipboard:write`

```js
await navigator.sigmaos.clipboard.write({
  type: "text/plain",
  data: "Copied text"
});
```

---

## navigator.sigmaos.shell

### `.exec(options)`

Higher-level wrapper combining `pkg.ensure` + `process.spawn` into a single call. Ensures the binary is available before executing.

```js
const result = await navigator.sigmaos.shell.exec({
  cmd: "ffmpeg",
  args: ["-i", "input.mp4", "output.webm"],
  caps: ["bin:~/.sigmaos/bin/ffmpeg", "fs:/tmp"],
  stdin: videoBytes  // optional Uint8Array
});

console.log(result.stdout);
```

---

## Error Handling

All `navigator.sigmaos` APIs return Promises that reject with typed errors:

| Error class | When it's thrown |
|---|---|
| `PermissionDeniedError` | Required capability not in manifest |
| `DaemonUnavailableError` | The target daemon is not running |
| `TimeoutError` | Daemon did not respond within 10 seconds |
| `ProcessError` | Spawned process exited with non-zero code |
| `FSError` | Filesystem operation failed (path not found, permission denied, etc.) |

```js
try {
  await navigator.sigmaos.process.spawn({ cmd: "rm", args: ["-rf", "/"] });
} catch (e) {
  if (e instanceof PermissionDeniedError) {
    console.error("App does not have process:spawn capability");
  }
}
```

---

*See also: [Security Model](Security-Model) · [Writing Your First App](Your-First-App) · [App Manifest Format](App-Manifest)*
