const fs = require('fs');
const path = require('path');

const baseDir = path.join(__dirname);
const appJsPath = path.join(baseDir, 'scripts', 'app.js');
const indexHtmlPath = path.join(baseDir, 'index.html');
const modulesDir = path.join(baseDir, 'scripts', 'modules');

if (!fs.existsSync(modulesDir)) {
    fs.mkdirSync(modulesDir);
}

let text = fs.readFileSync(appJsPath, 'utf-8');

text = text.replace(
    /method:\s*'POST',(\s*)body:\s*JSON\.stringify/g,
    "method: 'POST',$1headers: { 'Content-Type': 'application/json' },$1body: JSON.stringify"
);

const configSplit = text.split('document.addEventListener');
const configCode = configSplit[0];
let rest = configSplit[1];
rest = rest.substring(rest.indexOf('{') + 1, rest.lastIndexOf('}'));

const parts = rest.split(/\/\* ={62}\s*\*\s*(.*?)\s*\*\s*={62} \*\//g);

const sections = {};
sections['globals'] = parts[0];

for (let i = 1; i < parts.length; i += 2) {
    sections[parts[i]] = parts[i + 1];
}

fs.writeFileSync(path.join(modulesDir, '01_config.js'), configCode, 'utf-8');

for (const [k, v] of Object.entries(sections)) {
    const safeName = k.replace(/[^a-zA-Z0-9]/g, '_').toLowerCase();
    fs.writeFileSync(
        path.join(modulesDir, `02_${safeName}.js`),
        `document.addEventListener("DOMContentLoaded", () => {\n${v.trim()}\n});`,
        'utf-8'
    );
}

// Rename the original app.js to app.js.bak
if (fs.existsSync(appJsPath)) {
    fs.renameSync(appJsPath, appJsPath + '.bak');
}

let html = fs.readFileSync(indexHtmlPath, 'utf-8');
html = html.replace('<script src="scripts/app.js"></script>', `
    <script src="scripts/modules/01_config.js"></script>
    <script src="scripts/modules/02_globals.js"></script>
    <script src="scripts/modules/02_paradigm_selection__boot_menu_.js"></script>
    <script src="scripts/modules/02_gui_mode__zenith_dashboard.js"></script>
    <script src="scripts/modules/02_browser_use__dom_sweep_protocol.js"></script>
    <script src="scripts/modules/02_cli_mode__sovereign_shell.js"></script>
`);

fs.writeFileSync(indexHtmlPath, html, 'utf-8');
console.log('Modularisation completed via Node.js');
