/*
 * =========================================================================
 * SIGMA OS: COMPREHENSIVE WINDOWS-NATIVE BUILD SYSTEM
 * =========================================================================
 * Replaces 'make' entirely. Modularized for scalability.
 */

const compileShards = require('./tools/builder/tasks/compile_shards');

console.log("╔═══════════════════════════════════════════╗");
console.log("║    SIGMA OS — SOVEREIGN BUILD SYSTEM      ║");
console.log("╚═══════════════════════════════════════════╝\n");

compileShards();

console.log("[→] Run 'node server.js' to launch Zenith Dashboard at http://localhost:3334");
