const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

/* =========================================================================
 * SIGMA OS: COMPREHENSIVE WINDOWS-NATIVE BUILD SYSTEM
 * Replaces 'make' entirely. Compiles all C kernel shards using GCC.
 * =========================================================================
 */

console.log("╔═══════════════════════════════════════════╗");
console.log("║    SIGMA OS — SOVEREIGN BUILD SYSTEM      ║");
console.log("╚═══════════════════════════════════════════╝\n");

let gcc_available = false;
try {
    execSync('gcc --version', { stdio: 'ignore' });
    gcc_available = true;
    console.log("[✓] GCC detected. Compiling native C kernel shards...\n");
} catch {
    console.log("[!] GCC not found. Skipping C compilation.\n");
    console.log("    To compile the bare-metal kernel, install GCC:");
    console.log("    → winget install GnuWin32.GCC  (or MSYS2 / MinGW)\n");
}

// All sovereign C source shards
const shards = [
    { name: "PMM (Memory Matrix)",     src: "kernel/suites/S05_Memory/sigma_pmm.c",           obj: "build/sigma_pmm.o" },
    { name: "VGA (Display Driver)",    src: "kernel/suites/S03_Hardware/sigma_vga.c",          obj: "build/sigma_vga.o" },
    { name: "CPU GDT (Ring Setup)",    src: "kernel/suites/S02_CPU/sigma_cpu.c",               obj: "build/sigma_cpu.o" },
    { name: "CPU IDT (Interrupt MTX)", src: "kernel/suites/S02_CPU/sigma_idt.c",               obj: "build/sigma_idt.o" },
    { name: "Scheduler (ThreadMTX)",   src: "kernel/suites/S01_Scheduler/sigma_process.c",     obj: "build/sigma_process.o" },
    { name: "File System (SigmaFS)",   src: "kernel/suites/S06_Storage/sigma_fs.c",             obj: "build/sigma_fs.o" },
    { name: "Security (SovereignSec)", src: "kernel/suites/S08_Security/sigma_security.c",     obj: "build/sigma_security.o" },
    { name: "Registry (SovereignReg)", src: "kernel/suites/S10_Registry/sigma_registry.c",     obj: "build/sigma_registry.o" },
    { name: "Network (SNPS Stack)",    src: "kernel/suites/S07_Network/sigma_network.c",       obj: "build/sigma_network.o" },
    { name: "AI Memory (Agents)",      src: "kernel/suites/S09_Intelligence/sigma_ai_mem.c",   obj: "build/sigma_ai_mem.o" },
    { name: "Hypervisor (VirtIO)",     src: "kernel/suites/S11_Virtualization/sigma_virtio.c", obj: "build/sigma_virtio.o" },
    { name: "Kernel Main (kmain)",     src: "kernel/sigma_kernel.c",                           obj: "build/sigma_kernel.o" },
    { name: "Syscall Dispatcher",      src: "kernel/suites/S25_ZeroKernel/sigma_syscall.c",    obj: "build/sigma_syscall.o" },
];

// Ensure build dir exists
if (!fs.existsSync('build')) fs.mkdirSync('build');

let compiled = 0, skipped = 0, failed = 0;

shards.forEach(shard => {
    if (!fs.existsSync(shard.src)) {
        console.log(`  [~] SKIP  ${shard.name.padEnd(28)} (source missing)`);
        skipped++;
        return;
    }
    if (!gcc_available) {
        console.log(`  [~] SKIP  ${shard.name.padEnd(28)} (no GCC)`);
        skipped++;
        return;
    }
    try {
        execSync(`gcc -c ${shard.src} -o ${shard.obj} -Iinclude -std=c11 -ffreestanding -Wall`, { stdio: 'pipe' });
        console.log(`  [✓] BUILT ${shard.name}`);
        compiled++;
    } catch (e) {
        const err = e.stderr ? e.stderr.toString().split('\n')[0] : 'Unknown error';
        console.log(`  [✗] FAIL  ${shard.name.padEnd(28)} → ${err}`);
        failed++;
    }
});

console.log(`\n─────────────────────────────────────────────`);
console.log(`  Compiled: ${compiled}  |  Skipped: ${skipped}  |  Failed: ${failed}`);
console.log(`─────────────────────────────────────────────\n`);

if (compiled > 0 && failed === 0) {
    console.log("[✓] All shards compiled. Link with: nasm boot.asm + ld -T kernel/sigma.ld");
}
console.log("[→] Run 'node server.js' to launch Zenith Dashboard at http://localhost:3334");
