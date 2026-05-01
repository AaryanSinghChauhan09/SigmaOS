const fs = require('fs');
const path = require('path');

const coreDir = 'kernel/core';

const categories = {
    memory: ['Allocator', 'MMU', 'Heap', 'Mem', 'Paging', 'Cache', 'LRU'],
    security: ['SecHardener', 'Entropy', 'Audit', 'Sandbox', 'PQC', 'Crypto', 'Vault', 'Identity', 'Gatekeeper', 'Enclave', 'PrivDash'],
    network: ['Net', 'NIC', 'WiFi', 'Bluetooth', 'ZeroNet', 'Protocol'],
    fs: ['FS', 'VFS', 'Storage', 'Persistence', 'Disk', 'Namespace'],
    ui: ['UI', 'Zenith', 'Theme', 'Canvas', 'Display', 'Gesture', 'Haptic', 'Audio', 'Voice', 'DisplayServer', 'SpatialUI', 'Desktop'],
    ai: ['Neural', 'AI', 'Cognitive', 'Predict', 'Omni', 'Healer', 'Kube'],
    hardware: ['HW', 'GPU', 'USB', 'Battery', 'Power', 'Thermal', 'Device', 'NICDriver', 'Micro', 'Nano', 'IoT'],
    system: ['Main', 'Init', 'Boot', 'Process', 'IPC', 'Monitor', 'Sched', 'SMP', 'Diag', 'KernelIO', 'Log', 'Orchestrator', 'Syscall', 'Task', 'Microkernel', 'Watchdog', 'Hypervisor', 'Container', 'Install']
};

function getCategory(filename) {
    for (const [cat, keywords] of Object.entries(categories)) {
        for (const kw of keywords) {
            if (filename.includes(kw)) {
                return cat;
            }
        }
    }
    return 'misc';
}

const files = fs.readdirSync(coreDir).filter(f => f.endsWith('.cpp') || f.endsWith('.hpp') || f.endsWith('.c') || f.endsWith('.h') || f.endsWith('.asm') || f.endsWith('.rs'));

let makefileContent = fs.readFileSync('Makefile', 'utf8');

files.forEach(f => {
    const cat = getCategory(f);
    const catDir = path.join(coreDir, cat);
    if (!fs.existsSync(catDir)) {
        fs.mkdirSync(catDir, { recursive: true });
    }
    const oldPath = path.join(coreDir, f);
    const newPath = path.join(catDir, f);
    
    fs.renameSync(oldPath, newPath);
    
    // Update Makefile
    const oldMakePath = `kernel/core/${f.replace('.cpp', '.o').replace('.asm', '.o')}`;
    const newMakePath = `kernel/core/${cat}/${f.replace('.cpp', '.o').replace('.asm', '.o')}`;
    makefileContent = makefileContent.replace(oldMakePath, newMakePath);
});

fs.writeFileSync('Makefile', makefileContent);
console.log('Kernel core modularization complete.');
