const { execSync } = require('child_process');
const fs = require('fs');

/* =========================================================================
 * SIGMA OS: SOVEREIGN BUILD SYSTEM
 * A portable JavaScript-based build system for cross-platform kernel compilation.
 * ========================================================================= */

console.log("--- SIGMA OS BOOTSTRAP BUILDER ---");

const commands = [
    { name: "C-Kernel", cmd: "gcc -c kernel/sigma_kernel.c -o kernel/sigma_kernel.o -Iinclude" },
    { name: "VFS Shard", cmd: "gcc -c kernel/fs/sigma_vfs.c -o kernel/fs/sigma_vfs.o -Iinclude" },
    { name: "PMM Matrix", cmd: "gcc -c kernel/suites/S05_Memory/sigma_pmm.c -o kernel/suites/S05_Memory/sigma_pmm.o -Iinclude" },
    { name: "Network Suite", cmd: "gcc -c kernel/suites/S07_Network/sigma_network.c -o kernel/suites/S07_Network/sigma_network.o -Iinclude" },
    { name: "Intelligence Matrix", cmd: "gcc -c kernel/suites/S09_Intelligence/sigma_ai_mem.c -o kernel/suites/S09_Intelligence/sigma_ai_mem.o -Iinclude" },
    { name: "HTTP Engine", cmd: "gcc -c kernel/net/sigma_http.c -o kernel/net/sigma_http.o -Iinclude" },
    { name: "Web Server", cmd: "gcc -c kernel/SovereignHTTPServer.c -o kernel/SovereignHTTPServer.o -Iinclude" },
];

let success = true;
commands.forEach(comp => {
    try {
        process.stdout.write(`[*] Compiling ${comp.name}... `);
        execSync(comp.cmd, { stdio: 'ignore' });
        console.log("DONE");
    } catch (e) {
        console.log("FAILED (Check GCC installation)");
        success = false;
    }
});

if (success) {
    console.log("\n[SUCCESS] SigmaOS Sovereign Modules compiled into Silicon Matrix.");
    console.log("Run 'node server.js' to launch the Zenith UI.");
} else {
    console.log("\n[ERROR] Build was incomplete. Ensure 'gcc' is in your PATH.");
}
