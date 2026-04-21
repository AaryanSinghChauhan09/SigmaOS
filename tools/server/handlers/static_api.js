const fs = require('fs');
const path = require('path');
const { ROOT_DIR, mimeTypes } = require('../config');

module.exports = (req, res) => {
    let filePath = path.join(ROOT_DIR, 'web_ui', req.url === '/' ? 'index.html' : req.url);

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
};
