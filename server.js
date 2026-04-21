/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN HTTP BROWSER ORCHESTRATOR
 * =========================================================================
 * Modular Node.js Server for the SigmaOS Ecosystem.
 */

const http = require('http');
const { PORT } = require('./tools/server/config');
const router = require('./tools/server/router');

const server = http.createServer(router);

server.listen(PORT, () => {
    console.log(`\x1b[36m=============================================================\x1b[0m`);
    console.log(`\x1b[35m[SIGMA-HTTP]\x1b[0m Sovereign Zenith Dashboard Active: \x1b[32mhttp://localhost:${PORT}\x1b[0m`);
    console.log(`\x1b[35m[SYSTEM]\x1b[0m The full OS 33-Suite Lattice is accessible in-browser.`);
    console.log(`\x1b[36m=============================================================\x1b[0m`);
});
