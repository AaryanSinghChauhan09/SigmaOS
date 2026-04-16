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
const { spawn } = require('child_process');

const PORT = 3334;
const ROOT_DIR = __dirname;
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

    // API: Real-time Command Execution Stream (Pillar 2)
    if (req.url.startsWith('/api/run') && req.method === 'POST') {
        let body = '';
        req.on('data', chunk => body += chunk.toString());
        req.on('end', () => {
            try {
                const { cmd, cwd } = JSON.parse(body);
                res.writeHead(200, {
                    'Content-Type': 'text/plain; charset=utf-8',
                    'Transfer-Encoding': 'chunked'
                });

                const child = spawn(cmd, { shell: true, cwd: path.join(ROOT_DIR, cwd || '') });
                
                child.stdout.on('data', data => res.write(data.toString()));
                child.stderr.on('data', data => res.write(`[ERR] ${data.toString()}`));
                child.on('close', code => res.end(`\n[Process Exited: ${code}]`));
                child.on('error', err => res.end(`\n[Spawn Error: ${err.message}]`));
            } catch (e) {
                res.writeHead(400);
                res.end('Invalid Payload');
            }
        });
        return;
    }

    // API: Kernel Telemetry Stream (Pillar 4)
    if (req.url === '/api/telemetry') {
        const defaultTelemetry = { coverage: 100, iq_yield: "ABSOLUTE", memory: "1.2GB/64GB", latency: "2ms" };
        try {
            if (fs.existsSync(TELEMETRY_FILE)) {
                res.writeHead(200, { 'Content-Type': 'application/json' });
                return res.end(fs.readFileSync(TELEMETRY_FILE, 'utf-8'));
            }
        } catch(e) {}
        
        res.writeHead(200, { 'Content-Type': 'application/json' });
        return res.end(JSON.stringify(defaultTelemetry));
    }

    // API: Sigma Vault — full App Store package database
    if (req.url === '/api/vault') {
        const vaultPath = path.join(ROOT_DIR, 'web_ui', 'sigma_vault.json');
        try {
            const data = fs.readFileSync(vaultPath, 'utf-8');
            res.writeHead(200, { 'Content-Type': 'application/json' });
            return res.end(data);
        } catch (e) {
            res.writeHead(500);
            return res.end(JSON.stringify({ error: 'Vault DB unreadable' }));
        }
    }

    // API: Download/Emulate a package by ID (streams the .tar.gz payload)
    if (req.url.startsWith('/api/download/')) {
        const pkgId = decodeURIComponent(req.url.replace('/api/download/', '').split('?')[0]);

        // Read vault to get payload filename
        try {
            const vault = JSON.parse(fs.readFileSync(path.join(ROOT_DIR, 'web_ui', 'sigma_vault.json')));
            const pkg = vault.packages.find(p => p.id === pkgId);

            if (!pkg) {
                res.writeHead(404);
                return res.end(JSON.stringify({ error: `Package '${pkgId}' not in Vault.` }));
            }

            const payloadName = pkg.payload || `${pkgId}.sigma.archive`;
            const payloadPath = path.join(ROOT_DIR, 'web_ui', 'payloads', payloadName);

            if (fs.existsSync(payloadPath)) {
                const stat = fs.statSync(payloadPath);
                res.writeHead(200, {
                    'Content-Type': 'application/octet-stream',
                    'Content-Disposition': `attachment; filename="${payloadName}"`,
                    'Content-Length': stat.size
                });
                return fs.createReadStream(payloadPath).pipe(res);
            } else {
                // Generate an on-demand payload if not pre-built
                const onDemand = Buffer.alloc(1024 * 10, pkgId);
                res.writeHead(200, {
                    'Content-Type': 'application/octet-stream',
                    'Content-Disposition': `attachment; filename="${pkgId}.sigma.archive"`,
                    'Content-Length': onDemand.length,
                    'X-Sigma-Generated': 'on-demand'
                });
                return res.end(onDemand);
            }
        } catch (e) {
            res.writeHead(500);
            return res.end(`Download error: ${e.message}`);
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
