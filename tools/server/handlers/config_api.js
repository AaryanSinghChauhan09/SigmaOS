const fs   = require('fs');
const path = require('path');
const { ROOT_DIR } = require('../config');

const CONFIG_FILE  = path.join(ROOT_DIR, 'sigma_config.json');
const PLUGINS_DIR  = path.join(ROOT_DIR, 'plugins');
const PROFILES_DIR = path.join(ROOT_DIR, 'profiles');

function readJSON(filePath, fallback = {}) {
    try { return JSON.parse(fs.readFileSync(filePath, 'utf-8')); }
    catch { return fallback; }
}

function writeJSON(filePath, data) {
    fs.writeFileSync(filePath, JSON.stringify(data, null, 2), 'utf-8');
}

function collectBody(req, cb) {
    let body = '';
    req.on('data', c => body += c.toString());
    req.on('end', () => { try { cb(JSON.parse(body)); } catch { cb({}); } });
}

module.exports = (req, res) => {
    const url = req.url;
    res.setHeader('Content-Type', 'application/json');
    res.setHeader('Access-Control-Allow-Origin', '*');

    // ── GET /api/config ──────────────────────────────────────────────────────
    if (url === '/api/config' && req.method === 'GET') {
        const cfg = readJSON(CONFIG_FILE, { theme: 'MATRIX', accent: '#00f0ff', blur: 25 });
        res.writeHead(200);
        return res.end(JSON.stringify(cfg));
    }

    // ── POST /api/config ─────────────────────────────────────────────────────
    if (url === '/api/config' && req.method === 'POST') {
        collectBody(req, ({ key, value, config }) => {
            const cfg = readJSON(CONFIG_FILE, {});
            if (config) {
                // Bulk update
                Object.assign(cfg, config);
            } else if (key !== undefined) {
                cfg[key] = value;
            }
            writeJSON(CONFIG_FILE, cfg);
            res.writeHead(200);
            res.end(JSON.stringify({ ok: true, updated: key || 'bulk' }));
        });
        return;
    }

    // ── GET /api/plugins ─────────────────────────────────────────────────────
    if (url === '/api/plugins' && req.method === 'GET') {
        if (!fs.existsSync(PLUGINS_DIR)) {
            res.writeHead(200); return res.end(JSON.stringify([]));
        }
        const plugins = fs.readdirSync(PLUGINS_DIR)
            .filter(d => fs.statSync(path.join(PLUGINS_DIR, d)).isDirectory())
            .map(d => {
                const manifest = path.join(PLUGINS_DIR, d, 'plugin.json');
                return readJSON(manifest, { name: d, enabled: false });
            })
            .filter(p => p.enabled);
        res.writeHead(200);
        return res.end(JSON.stringify(plugins));
    }

    // ── GET /api/profiles ────────────────────────────────────────────────────
    if (url === '/api/profiles' && req.method === 'GET') {
        if (!fs.existsSync(PROFILES_DIR)) {
            res.writeHead(200); return res.end(JSON.stringify([]));
        }
        const profiles = fs.readdirSync(PROFILES_DIR)
            .filter(f => f.endsWith('.json'))
            .map(f => readJSON(path.join(PROFILES_DIR, f)));
        res.writeHead(200);
        return res.end(JSON.stringify(profiles));
    }

    // ── POST /api/profiles/switch ─────────────────────────────────────────────
    if (url === '/api/profiles/switch' && req.method === 'POST') {
        collectBody(req, ({ name }) => {
            const profilePath = path.join(PROFILES_DIR, `${name}.json`);
            if (!fs.existsSync(profilePath)) {
                res.writeHead(404); return res.end(JSON.stringify({ error: 'Profile not found' }));
            }
            const profile = readJSON(profilePath);
            const cfg = readJSON(CONFIG_FILE, {});
            Object.assign(cfg, { ...profile, profile: name });
            writeJSON(CONFIG_FILE, cfg);
            res.writeHead(200);
            res.end(JSON.stringify({ ok: true, profile: name, config: profile }));
        });
        return;
    }

    // ── Fallthrough ───────────────────────────────────────────────────────────
    res.writeHead(404);
    res.end(JSON.stringify({ error: 'API endpoint not found' }));
};
