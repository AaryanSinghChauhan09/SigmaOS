/**
 * Sovereign Explorer (v2.0)
 * Functional VFS browser that makes SigmaOS feel like a complete Desktop system.
 * Surpasses entry-level Linux distros with integrated Silicon Primitives.
 */

class SovereignExplorer extends ZenithComponent {
    constructor() {
        super('explorer-view');
        this.currentPath = '/';
        this.vfs = {
            '/': [
                { name: 'kernel', type: 'dir' },
                { name: 'lib', type: 'dir' },
                { name: 'init', type: 'dir' },
                { name: 'fs', type: 'dir' },
                { name: 'net', type: 'dir' },
                { name: 'usr', type: 'dir' },
                { name: 'SOUL.md', type: 'file', content: '# Σ SIGMAOS ZENITH\nModular Linux-Inspired microkernel layout verified.' }
            ],
            '/kernel': [
                { name: 'scheduler', type: 'dir' },
                { name: 'drivers', type: 'dir' },
                { name: 'core', type: 'dir' },
                { name: 'README.md', type: 'file', content: '# SigmaOS Kernel Layer\nHouses process scheduler, physical/virtual memory pagers, and VGA/Serial device drivers.' }
            ],
            '/kernel/scheduler': [
                { name: 'SovereignScheduler.cpp', type: 'file', content: '// Round-Robin & Multilevel Feedback Queue task scheduler\nvoid schedule() { ... }' }
            ],
            '/kernel/drivers': [
                { name: 'serial.c', type: 'file', content: '// UART Serial driver for bare-metal debugging\nvoid serial_init() { port_outb(COM1+1, 0x00); ... }' },
                { name: 'vga.c', type: 'file', content: '// VGA video mode handler\nvoid vga_clear() { ... }' },
                { name: 'keyboard.c', type: 'file', content: '// PS/2 Keyboard scancode driver\nchar kbd_read() { ... }' }
            ],
            '/lib': [
                { name: 'libc', type: 'dir' },
                { name: 'README.md', type: 'file', content: '# SigmaOS Libc\nZero-dependency C standard library primitives.' }
            ],
            '/lib/libc': [
                { name: 'sigma_libc.c', type: 'file', content: '// Custom libc implementations\nvoid sigma_printf(const char* fmt, ...) { ... }' }
            ],
            '/init': [
                { name: 'init.c', type: 'file', content: '// PID 1 System Init Process\nvoid init_main() {\n    init_core_kernel();\n    init_vfs();\n    init_tcp_ip();\n    run_user_shell();\n}' },
                { name: 'README.md', type: 'file', content: '# SigmaOS Init System\nExecutes kernel services in dependency-order and drops to shell.' }
            ],
            '/fs': [
                { name: 'ext4.c', type: 'file', content: '// Ext4 superblock and inode reader\nvoid init_ext4() { ... }' },
                { name: 'fat32.c', type: 'file', content: '// FAT32 EBR and cluster chain traverser\nvoid init_fat32() { ... }' },
                { name: 'vfs.c', type: 'file', content: '// Virtual File System abstraction layer\nvoid sigma_vfs_init() { ... }' },
                { name: 'README.md', type: 'file', content: '# SigmaOS File System Layer\nMounts storage nodes and maps inodes to file descriptors.' }
            ],
            '/net': [
                { name: 'loopback.c', type: 'file', content: '// Local interface transmitter\nvoid init_loopback_net() { ... }' },
                { name: 'tcp_ip.c', type: 'file', content: '// Custom TCP/UDP stack and connection tables\nvoid init_tcp_ip() { ... }' },
                { name: 'dns.c', type: 'file', content: '// Local DNS host resolver\nsigma_u32 dns_resolve(const char* name) { ... }' },
                { name: 'README.md', type: 'file', content: '# SigmaOS Network Stack\nLoopback and zero-dependency network sockets.' }
            ],
            '/usr': [
                { name: 'sh.c', type: 'file', content: '// Sovereign sh shell command line interpreter\nvoid run_user_shell() { ... }' },
                { name: 'omni_shell.c', type: 'file', content: '// Comprehensive system auditing shell\nvoid start_shell_zenith() { ... }' },
                { name: 'README.md', type: 'file', content: '# SigmaOS Userland\nUser-space command shell and basic coreutils.' }
            ]
        };
        this.init();
    }

    init() {
        this.renderPath();
        this.bindEvents();
    }

    bindEvents() {
        const upBtn = Sigma.node('btn-up-dir');
        if (upBtn) upBtn.onclick = () => this.navigate('..');
    }

    renderPath() {
        const listContainer = Sigma.node('explorer-list');
        const pathLabel = Sigma.node('current-path');
        if (!listContainer || !pathLabel) return;

        pathLabel.textContent = this.currentPath;
        listContainer.innerHTML = '';

        const files = this.vfs[this.currentPath] || [];
        
        Sigma.each(files, file => {
            const row = document.createElement('div');
            row.className = 'file-row glass-panel';
            row.innerHTML = `
                <span class="file-icon">${file.type === 'dir' ? '📁' : '📄'}</span>
                <span class="file-name">${file.name}</span>
            `;
            
            row.onclick = () => {
                if (file.type === 'dir') {
                    this.navigate(`${this.currentPath}/${file.name}`);
                } else {
                    this.openFile(file);
                }
            };
            
            listContainer.appendChild(row);
        });
    }

    navigate(sub) {
        if (sub === '..') {
            const parts = this.currentPath.split('/');
            if (parts.length > 2) {
                parts.pop();
                this.currentPath = parts.join('/');
            }
        } else {
            this.currentPath = sub;
        }
        this.renderPath();
    }

    openFile(file) {
        window.zenith.taskbar.notify(`OPENING: ${file.name}`, 'STABLE');
        console.log(`Σ://FS> [READ] ${file.name}: ${file.content}`);
    }
}

window.SovereignExplorer = SovereignExplorer;
