# Writing Your First SigmaOS App

This tutorial walks through building, testing, and "installing" a minimal SigmaOS PWA from scratch. By the end you'll have a working app that reads a file, spawns a native process, and displays the output — all from a web page.

---

## What you'll build

A simple **file word-counter** app that:
1. Lets the user pick a text file from their home directory
2. Spawns the native `wc` binary to count words
3. Displays the result in the browser

---

## Prerequisites

- SigmaOS running in QEMU or on hardware (or the web shell at `http://localhost:8080` in dev mode)
- A text editor (SigmaCode is built in, or use the host machine)
- Basic knowledge of HTML/JavaScript

---

## Step 1: Create the app manifest

Every SigmaOS app needs a `manifest.json` that declares its identity and capabilities.

```json
{
  "name": "WordCounter",
  "id": "dev.sigmaos.wordcounter",
  "version": "1.0.0",
  "description": "Counts words in a text file using native wc",
  "start_url": "/index.html",
  "icon": "/icon.png",
  "capabilities": [
    "process:spawn",
    "fs:/home/user",
    "bin:wc"
  ]
}
```

**Key fields:**
- `id` — reverse-domain identifier, must be globally unique
- `start_url` — the entry point URL for the app
- `capabilities` — the minimum set of permissions the app needs to function

---

## Step 2: Write the HTML

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>WordCounter</title>
  <style>
    body { font-family: system-ui; max-width: 600px; margin: 2rem auto; padding: 0 1rem; }
    button { padding: 0.5rem 1rem; cursor: pointer; }
    #result { margin-top: 1rem; font-size: 1.5rem; font-weight: bold; }
    #error  { color: red; margin-top: 0.5rem; }
  </style>
</head>
<body>
  <h1>Word Counter</h1>
  <p>Pick a text file from your home directory:</p>

  <select id="file-picker"></select>
  <button id="count-btn">Count Words</button>

  <div id="result"></div>
  <div id="error"></div>

  <script src="app.js"></script>
</body>
</html>
```

---

## Step 3: Write the JavaScript

```js
// app.js

const picker  = document.getElementById('file-picker');
const btn     = document.getElementById('count-btn');
const result  = document.getElementById('result');
const errDiv  = document.getElementById('error');

// Populate the file list on load
async function loadFiles() {
  try {
    const entries = await navigator.sigmaos.fs.readdir('/home/user');

    entries
      .filter(e => e.type === 'file' && e.name.endsWith('.txt'))
      .forEach(e => {
        const opt = document.createElement('option');
        // Use textContent — never innerHTML — to safely set user-supplied names
        opt.textContent = e.name;
        opt.value = '/home/user/' + e.name;
        picker.appendChild(opt);
      });
  } catch (err) {
    showError('Could not read home directory: ' + err.message);
  }
}

// Count words in the selected file
async function countWords() {
  result.textContent = '';
  errDiv.textContent  = '';

  const filePath = picker.value;
  if (!filePath) {
    showError('Please select a file first.');
    return;
  }

  btn.disabled = true;
  btn.textContent = 'Counting...';

  try {
    // Spawn the native wc binary inside a sandbox
    const proc = await navigator.sigmaos.process.spawn({
      cmd: 'wc',
      args: ['-w', filePath],
      caps: ['bin:wc', 'fs:/home/user']
    });

    // Collect stdout
    let output = '';
    for await (const chunk of proc.stdout) {
      output += chunk;
    }

    const exitCode = await proc.wait();

    if (exitCode !== 0) {
      showError('wc exited with code ' + exitCode);
      return;
    }

    // wc output: "  1234 /home/user/notes.txt"
    const wordCount = output.trim().split(/\s+/)[0];
    result.textContent = wordCount + ' words';

  } catch (err) {
    if (err.name === 'PermissionDeniedError') {
      showError('Missing capability. Make sure manifest.json includes "process:spawn".');
    } else {
      showError('Error: ' + err.message);
    }
  } finally {
    btn.disabled = false;
    btn.textContent = 'Count Words';
  }
}

function showError(msg) {
  errDiv.textContent = msg;
}

btn.addEventListener('click', countWords);
loadFiles();
```

---

## Step 4: Install the app

In the SigmaOS shell, open the App Store (or use the developer install flow):

1. Open the **Settings** app → **Developer Mode** → enable it.
2. Click **Install from URL** and paste the URL where your app is hosted (or `localhost:3000` if running a dev server).
3. SigmaOS reads the `manifest.json`, prompts you to grant the declared capabilities, and adds the app to the launcher.

For local development, you can also serve the app with:

```bash
npx serve .   # runs on localhost:3000
```

Then install it from `http://localhost:3000/manifest.json`.

---

## Step 5: Try the capability system

Now try removing `"process:spawn"` from your manifest and reinstalling. When you click Count Words, you'll get a `PermissionDeniedError` — the extension blocks the call before it reaches the daemon. This is the capability system working correctly.

---

## What's next

- **Add stdin**: Pass file contents as stdin to `wc -w` instead of a path argument, using `navigator.sigmaos.fs.read()`.
- **Stream output**: For long-running processes, render each chunk as it arrives using the `for await` loop.
- **Add AI**: Call `navigator.sigmaos.ai.complete()` to summarize the document after counting words.
- **Publish**: Submit a PR to the [sigmaos/app-registry](https://github.com/sigmaos/app-registry) repo with your `manifest.json` and it will appear in the public App Store.

---

*See also: [API Reference](API-Reference) · [App Manifest Format](App-Manifest) · [Security Model](Security-Model)*
