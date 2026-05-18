# Web Programming → SigmaWeb Runtime

> Maps the Web Programming syllabus (HTML5, CSS, JS, PHP) to `SigmaWeb` — the browser-like runtime environment embedded inside SigmaOS.

---

## Architecture Overview

```
SigmaWeb Runtime
├── HTML5 Parser + DOM Engine
├── CSS Layout Engine (Flexbox, Grid)
├── JavaScript Engine (V8-compatible or QuickJS)
├── PHP Runtime (PHP-FPM compatible)
├── WebSocket + WebWorker support
├── Geolocation API (from HAL GPS driver)
└── localStorage / sessionStorage (on SovereignFS)
```

---

## Unit I: HTML5 Foundations

### Basic Tags & Structure

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SigmaOS Web App</title>
    <link rel="stylesheet" href="sigma.css">
</head>
<body>
    <!-- Heading tags: h1–h6 -->
    <h1>SigmaOS Dashboard</h1>

    <!-- Paragraph & Formatting -->
    <p>This is a <strong>bold</strong> and <em>italic</em> text.</p>
    <p><u>Underlined</u>, <s>strikethrough</s>, <mark>highlighted</mark></p>
    <pre><code>monospace code block</code></pre>

    <!-- Lists -->
    <ul><li>Unordered item 1</li><li>Item 2</li></ul>
    <ol><li>Ordered item 1</li><li>Item 2</li></ol>
    <dl><dt>CPU</dt><dd>Central Processing Unit</dd></dl>

    <!-- Table -->
    <table border="1">
        <thead><tr><th>Process</th><th>PID</th><th>Status</th></tr></thead>
        <tbody>
            <tr><td>init</td><td>1</td><td>Running</td></tr>
            <tr><td>sigma-ui</td><td>42</td><td>Running</td></tr>
        </tbody>
    </table>

    <!-- Form -->
    <form action="/sigma/api/submit" method="POST">
        <input type="text" name="username" placeholder="Username" required>
        <input type="password" name="password" required>
        <input type="email" name="email">
        <input type="number" name="age" min="1" max="120">
        <select name="role"><option value="admin">Admin</option></select>
        <textarea name="bio" rows="4"></textarea>
        <input type="checkbox" name="agree"> I agree
        <input type="radio" name="os" value="sigma"> SigmaOS
        <input type="file" name="upload">
        <button type="submit">Login</button>
    </form>

    <!-- Media -->
    <audio controls src="sigma-boot.mp3"></audio>
    <video controls width="640" height="360" src="sigma-demo.mp4">
        <track kind="subtitles" src="sigma.vtt">
    </video>
    <img src="logo.png" alt="SigmaOS Logo" width="200">

    <script src="sigma.js"></script>
</body>
</html>
```

---

## Unit II: Advanced HTML5

### Semantic Tags

```html
<header>   <!-- Page/section header -->
<nav>      <!-- Navigation links -->
<main>     <!-- Main content area -->
<article>  <!-- Self-contained content -->
<section>  <!-- Thematic grouping -->
<aside>    <!-- Sidebar content -->
<footer>   <!-- Page/section footer -->
<figure>   <!-- Image with caption -->
<figcaption>
<details>  <!-- Collapsible content -->
<summary>  <!-- Visible heading for <details> -->
<time datetime="2026-05-18">May 18, 2026</time>
<mark>     <!-- Highlighted text -->
<progress value="75" max="100">75%</progress>
<meter value="0.7">70%</meter>
```

### HTML5 Web Storage

```javascript
// localStorage — persists after browser close (stored in SovereignFS)
localStorage.setItem('theme', 'dark');
let theme = localStorage.getItem('theme');
localStorage.removeItem('theme');

// sessionStorage — cleared when tab closes
sessionStorage.setItem('session_token', 'abc123');

// SigmaOS maps these to /sigma/web/storage/<origin>/
```

### HTML5 APIs

```javascript
// Web Workers — background thread
const worker = new Worker('sigma-worker.js');
worker.postMessage({ task: 'compute', data: largeArray });
worker.onmessage = (e) => console.log('Result:', e.data);

// WebSockets — real-time communication
const ws = new WebSocket('ws://localhost:8080/sigma/realtime');
ws.onopen    = () => ws.send(JSON.stringify({ type: 'hello' }));
ws.onmessage = (e) => console.log('Received:', e.data);
ws.onclose   = () => console.log('Connection closed');

// Geolocation (from HAL GPS driver in SigmaOS)
navigator.geolocation.getCurrentPosition(
    pos => console.log(pos.coords.latitude, pos.coords.longitude),
    err => console.error(err)
);

// Drag and Drop
element.draggable = true;
element.addEventListener('dragstart', e => e.dataTransfer.setData('text', 'sigma'));
target.addEventListener('drop', e => {
    e.preventDefault();
    const data = e.dataTransfer.getData('text');
});
```

---

## Unit III: JavaScript & CSS

### CSS

```css
/* Box model, selectors, specificity */
* { box-sizing: border-box; margin: 0; padding: 0; }

.sigma-panel {
    background: linear-gradient(135deg, #1a1a2e, #16213e);
    color: #e0e0ff;
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0,0,0,0.4);
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

/* Grid layout */
.dashboard { display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem; }

/* Animations */
@keyframes sigma-pulse {
    0%   { transform: scale(1); }
    50%  { transform: scale(1.05); }
    100% { transform: scale(1); }
}
.btn:hover { animation: sigma-pulse 0.3s ease; }

/* Media queries */
@media (max-width: 768px) { .dashboard { grid-template-columns: 1fr; } }
```

### JavaScript Core

```javascript
// Variables
var legacy = 'avoid';
let blockScoped = 42;
const immutable = 'SigmaOS';

// Operators
let result = 10 + 3;    // 13
let bits = 0xFF & 0x0F; // 0x0F — bitwise AND
let str = `SigmaOS v${15}.1`; // template literal

// Conditionals
if (role === 'admin') { grantAccess(); }
else if (role === 'user') { limitAccess(); }
else { deny(); }

const label = isActive ? 'Running' : 'Stopped';  // ternary

// Loops
for (let i = 0; i < procs.length; i++) { /* ... */ }
for (const proc of procs) { console.log(proc.name); }
while (retries < 3 && !connected) { retries++; }

// break / continue
for (const item of items) {
    if (item.type === 'skip') continue;
    if (item.critical) { process(item); break; }
}

// Dialog boxes
alert('SigmaOS boot complete');
const ok = confirm('Restart?');
const input = prompt('Enter username:', 'admin');

// Arrays
const cpus = ['x86-64', 'ARM64', 'RISC-V'];
cpus.push('GPU');
cpus.forEach(cpu => console.log(cpu));
const filtered = cpus.filter(c => c.includes('64'));
const mapped = cpus.map(c => c.toUpperCase());

// User-defined functions
function formatBytes(bytes) {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024*1024) return (bytes/1024).toFixed(1) + ' KB';
    return (bytes/1024/1024).toFixed(1) + ' MB';
}

// Arrow function
const square = x => x * x;

// Async/await
async function fetchSystemInfo() {
    const res = await fetch('/sigma/api/sysinfo');
    const data = await res.json();
    return data;
}
```

---

## Unit IV: PHP

```php
<?php
// Variables, echo, print
$kernel_version = "15.1";
$is_running = true;
echo "SigmaOS v" . $kernel_version . "\n";
print("Status: " . ($is_running ? "Running" : "Stopped"));

// Data Types
$int_val  = 42;
$float_val = 3.14;
$str_val  = "SigmaOS";
$bool_val = true;
$arr_val  = [1, 2, 3];
$null_val = null;

// Strings
echo strlen($str_val);            // 7
echo strtoupper($str_val);        // SIGMAOS
echo str_replace("OS", "OS v15.1", $str_val);
echo substr($str_val, 0, 5);      // Sigma

// Constants
define('SIGMA_VERSION', '15.1');
const MAX_PROCS = 4096;

// Operators
$result = 10 ** 2;         // 100 (power)
$combined = $str_val . " Zenith";

// Conditionals
if ($is_running) {
    echo "System Active";
} elseif ($kernel_version > 15) {
    echo "Latest version";
} else {
    echo "Offline";
}

// Arrays
$drivers = ["NVMe", "USB", "Audio", "Network"];
$process = ["pid" => 1, "name" => "init", "state" => "running"];
echo $process["name"];  // init

// Loops
foreach ($drivers as $key => $driver) {
    echo "$key: $driver\n";
}

for ($i = 0; $i < count($drivers); $i++) {
    echo $drivers[$i] . "\n";
}

while ($retries < 3) {
    if (connect()) break;
    $retries++;
}

// Functions
function get_memory_info(string $unit = 'MB'): string {
    $bytes = memory_get_usage();
    return match($unit) {
        'KB' => round($bytes / 1024, 2) . ' KB',
        'MB' => round($bytes / 1024 / 1024, 2) . ' MB',
        default => $bytes . ' B',
    };
}

echo get_memory_info('MB');
?>
```

---

## SigmaWeb Runtime Integration

| Web Standard | SigmaOS Component | File |
|---|---|---|
| HTML5 Parser | `SigmaDOM` | `userland/sigmaweb/dom/` |
| CSS Engine | `SigmaLayout` | `userland/sigmaweb/css/` |
| JS Engine | QuickJS embed | `userland/sigmaweb/js/` |
| PHP Runtime | PHP-FPM bridge | `userland/sigmaweb/php/` |
| WebSockets | `SovereignNetStack` | `kernel/net/` |
| localStorage | `SovereignFS` VFS | `kernel/fs/` |
| Geolocation | HAL GPS driver | `kernel/hal/` |

*Last updated: 2026-05-18 | SigmaOS Zenith v15.1*
