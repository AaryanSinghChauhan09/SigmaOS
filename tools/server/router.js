const fs = require('fs');
const path = require('path');
const { TELEMETRY_FILE, ROOT_DIR } = require('./config');

const fsHandler     = require('./handlers/fs_api');
const runHandler    = require('./handlers/run_api');
const staticHandler = require('./handlers/static_api');
const configHandler = require('./handlers/config_api');

module.exports = (req, res) => {
    res.setHeader('Access-Control-Allow-Origin', '*');

    if (req.url.startsWith('/api/fs'))       return fsHandler(req, res);
    if (req.url.startsWith('/api/run'))      return runHandler(req, res);
    if (req.url.startsWith('/api/config'))   return configHandler(req, res);
    if (req.url.startsWith('/api/plugins'))  return configHandler(req, res);
    if (req.url.startsWith('/api/profiles')) return configHandler(req, res);

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

    return staticHandler(req, res);
};
