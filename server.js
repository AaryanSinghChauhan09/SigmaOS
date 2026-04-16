/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN HTTP BROWSER ORCHESTRATOR
 * =========================================================================
 * Zero-dependency pure Node.js HTTP Server for local browser exploration.
 * Serves the web UI and exposes the repository file system.
 */

const http = require('http');
const fs = require('fs');
const path = require('path');

const PORT = 3333;
const ROOT_DIR = __dirname;

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
    '.txt': 'text/plain'
};

const server = http.createServer((req, res) => {
    // CORS headers for local usage
    res.setHeader('Access-Control-Allow-Origin', '*');
    
    // API: File System Navigation
    if (req.url.startsWith('/api/fs')) {
        const urlParams = new URL(req.url, `http://${req.headers.host}`);
        const queryPath = urlParams.searchParams.get('path') || '/';
        const targetPath = path.normalize(path.join(ROOT_DIR, queryPath));

        // Prevent traversal outside repo
        if (!targetPath.startsWith(ROOT_DIR)) {
            res.writeHead(403, { 'Content-Type': 'application/json' });
            return res.end(JSON.stringify({ error: 'Access Denied.' }));
        }

        try {
            const stats = fs.statSync(targetPath);
            if (stats.isDirectory()) {
                const results = fs.readdirSync(targetPath).map(file => {
                    const filePath = path.join(targetPath, file);
                    const isDir = fs.statSync(filePath).isDirectory();
                    return {
                        name: file,
                        isDir: isDir,
                        path: path.join(queryPath, file).replace(/\\/g, '/')
                    };
                });
                
                // Sort by directories first, then alphabetical
                results.sort((a, b) => {
                    if (a.isDir === b.isDir) return a.name.localeCompare(b.name);
                    return a.isDir ? -1 : 1;
                });

                res.writeHead(200, { 'Content-Type': 'application/json' });
                return res.end(JSON.stringify(results));
            } else {
                const content = fs.readFileSync(targetPath, 'utf-8');
                res.writeHead(200, { 'Content-Type': 'text/plain' });
                return res.end(content);
            }
        } catch (err) {
            res.writeHead(404, { 'Content-Type': 'application/json' });
            return res.end(JSON.stringify({ error: 'Not found or permission denied' }));
        }
    }

    // Static Server Logic
    let filePath = path.join(ROOT_DIR, 'web_ui', req.url === '/' ? 'index.html' : req.url);

    // If it doesn't exist in web_ui, try falling back to the repo root for static viewing
    if (!fs.existsSync(filePath)) {
        filePath = path.join(ROOT_DIR, req.url.split('?')[0]);
    }

    const extname = path.extname(filePath);
    const contentType = mimeTypes[extname] || 'application/octet-stream';

    try {
        if (fs.existsSync(filePath) && fs.statSync(filePath).isFile()) {
            const content = fs.readFileSync(filePath);
            res.writeHead(200, { 'Content-Type': contentType });
            res.end(content, 'utf-8');
        } else {
            res.writeHead(404, { 'Content-Type': 'text/html' });
            res.end('<h1>404: File Not Found in Sovereign Matrix</h1>', 'utf-8');
        }
    } catch (e) {
        res.writeHead(500, { 'Content-Type': 'text/html' });
        res.end(`<h1>500: Server Error</h1><p>${e.message}</p>`, 'utf-8');
    }
});

server.listen(PORT, () => {
    console.log(`\x1b[36m=============================================================\x1b[0m`);
    console.log(`\x1b[35m[SIGMA-HTTP]\x1b[0m Sovereign Zenith Dashboard Active: \x1b[32mhttp://localhost:${PORT}\x1b[0m`);
    console.log(`\x1b[35m[SYSTEM]\x1b[0m The full OS 33-Suite Lattice is accessible in-browser.`);
    console.log(`\x1b[36m=============================================================\x1b[0m`);
});
