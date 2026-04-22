const fs = require('fs');
const path = require('path');
const { ROOT_DIR } = require('../config');

module.exports = (req, res) => {
    const urlParams = new URL(req.url, `http://${req.headers.host}`);
    const queryPath = urlParams.searchParams.get('path') || '/';
    const targetPath = path.normalize(path.join(ROOT_DIR, queryPath));

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
};
