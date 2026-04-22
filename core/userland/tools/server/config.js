const path = require('path');

const PORT = 3334;
const ROOT_DIR = path.resolve(__dirname, '../../');
const TELEMETRY_FILE = path.join(ROOT_DIR, 'kernel', 'telemetry.json');

const mimeTypes = {
    '.html': 'text/html',
    '.css': 'text/css',
    '.js': 'text/javascript',
    '.md': 'text/markdown',
    '.json': 'application/json',
    '.c': 'text/plain',
    '.h': 'text/plain',
    '.asm': 'text/plain',
    '.ps1': 'text/plain',
    '.py': 'text/plain',
    '.txt': 'text/plain',
    '.gz': 'application/gzip',
    '.archive': 'application/octet-stream'
};

module.exports = { PORT, ROOT_DIR, TELEMETRY_FILE, mimeTypes };
