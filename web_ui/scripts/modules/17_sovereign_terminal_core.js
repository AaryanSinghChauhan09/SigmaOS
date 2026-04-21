/**
 * Sovereign Terminal Core (v3.0)
 * Unified CLI for all OS operations. 
 * Supports commands for filesystem, themes, sharding, and telemetry.
 */

class SovereignTerminal extends ZenithComponent {
    constructor() {
        super('cli-input-box');
        this.output = Sigma.node('cli-output');
        this.commands = {};
        this.history = [];
        this.historyIndex = -1;
        this.init();
    }

    init() {
        this.registerCoreCommands();
        if (this.element) {
            this.element.addEventListener('keydown', (e) => {
                if (e.key === 'Enter') {
                    this.processCommand(e.target.value);
                } else if (e.key === 'ArrowUp') {
                    this.navigateHistory(1);
                } else if (e.key === 'ArrowDown') {
                    this.navigateHistory(-1);
                }
            });
        }
    }

    navigateHistory(dir) {
        if (this.history.length === 0) return;
        this.historyIndex = Math.min(Math.max(this.historyIndex + dir, 0), this.history.length - 1);
        this.element.value = this.history[this.history.length - 1 - this.historyIndex];
    }

    registerCoreCommands() {
        this.commands = {
            'vault': (args) => {
                const sub = args[0];
                if (sub === 'install') {
                    window.vault.install(args[1]);
                } else {
                    this.write('VAULT PACKAGES: S34_MediaNexus, S35_QuantumCrypt, S36_NeuralLink');
                }
            },
            'snapshot': (args) => {
                const sub = args[0];
                if (sub === 'create') {
                    window.snapshots.createSnapshot(args[1]);
                } else if (sub === 'rollback') {
                    window.snapshots.rollback(args[1]);
                } else {
                    this.write('USAGE: snapshot [create|rollback] [id/label]');
                }
            },
            'tile': () => window.tiling.toggle(),
            'sandbox': (args) => {
                const sub = args[0];
                if (sub === 'run') {
                    window.sandbox.runInSandbox(args[1] || 'sh_untrusted_1', '// UNTRUSTED_CODE');
                } else if (sub === 'kill') {
                    window.sandbox.terminate(args[1]);
                } else {
                    this.write('USAGE: sandbox [run|kill] [id]');
                }
            },
            'session': (args) => {
                const username = args[0];
                if (username) {
                    window.sessions.switchUser(username);
                } else {
                    this.write(`CURRENT SESSION: ${window.sessions.currentUser}`);
                }
            },
            'top': () => {
                const top = window.profiler.getTopProcesses();
                this.write('Σ://PROFILER-TOP> Most Active Shards:');
                top.forEach(p => {
                    this.write(`- ${p.id.padEnd(6)} | ${p.cpu.padStart(5)}% CPU | ${p.mem.toString().padStart(4)}MB`);
                });
            },
            'shortcuts': () => {
                this.write('Σ://SHORTCUTS> Universal Hotkeys:');
                this.write('- Ctrl + K: Universal Search');
                this.write('- Ctrl + `: Focus Terminal');
                this.write('- Alt  + T: Toggle Tiling');
                this.write('- Ctrl + S: Create Lattice Snapshot');
            },
            'prof': () => this.commands.top(),
            'help': () => this.write('AVAILABLE: theme, ls, cd, cat, notify, shard, flush, stats, telemetry, set, get, vault, snapshot, tile, sandbox, session, top, prof, shortcuts, clear, version'),
            'clear': () => this.output.innerHTML = '',
            'version': () => this.write('Σ SIGMAOS ZENITH v33.0.4-SINGULARITY'),
            
            // Theme Integration
            'theme': (args) => {
                if (!args[0]) return this.write('USAGE: theme [MATRIX|GHOST_MICA|SOVEREIGN_GOLD]');
                window.theme.applyTheme(args[0].toUpperCase());
                this.write(`THEME SYNCHRONIZED: ${args[0]}`);
            },

            // Filesystem Integration
            'ls': () => {
                const files = window.explorer.vfs[window.explorer.currentPath] || [];
                Sigma.each(files, f => this.write(`[${f.type.toUpperCase()}] ${f.name}`));
            },
            'cd': (args) => {
                const path = args[0] || '/';
                window.explorer.navigate(path);
                this.write(`PATH CHANGED: ${window.explorer.currentPath}`);
            },
            'cat': (args) => {
                const files = window.explorer.vfs[window.explorer.currentPath] || [];
                const file = files.find(f => f.name === args[0]);
                if (file) this.write(file.content);
                else this.write(`ERR: FILE NOT FOUND: ${args[0]}`);
            },

            // Notification Integration
            'notify': (args) => {
                window.zenith.taskbar.notify(args.join(' '), 'OPTIMAL');
                this.write('NOTIFICATION DISPATCHED.');
            },

            // Kernel Sharding Integration
            'shard': (args) => {
                const sub = args[0];
                const id = args[1];
                if (sub === 'kill' && id) {
                    window.sharding.simulateFailure(id);
                    this.write(`SIGNAL SENT: PANIC SHARD ${id}`);
                } else if (sub === 'ls') {
                    this.write('LATTICE SHARDS: S01-S33 [ARMED]');
                } else {
                    this.write('USAGE: shard [ls|kill] [id]');
                }
            },

            // System Maintenance
            'vitals': () => {
                this.write(`LATTICE HEALTH: 98.4% [OPTIMAL]`);
                this.write(`WASM JIT LOAD: 2.1%`);
                this.write(`MEMORY USAGE: 144MB / 4G`);
            },
            'logs': () => {
                this.write('Σ://LOGS> [BOOT] SOVEREIGN KERNEL v33.0.4 LOADED.');
                this.write('Σ://LOGS> [AUTH] USER Ʃ_ZENITH AUTHENTICATED.');
                this.write('Σ://LOGS> [LATTICE] 33 SUITES ONLINE.');
            },
            'set': (args) => {
                if (args.length < 2) return this.write('USAGE: set [key] [value]');
                window.settings.set(args[0], args[1]);
                this.write(`SYSTEM SYNC: ${args[0]} = ${args[1]}`);
            },
            'get': (args) => {
                if (!args[0]) return this.write(JSON.stringify(window.settings.config, null, 2));
                this.write(`${args[0]}: ${window.settings.config[args[0]]}`);
            },
            'flush': () => {
                console.log('Σ://KERNEL> Flushing Silicon Primitives...');
                this.write('MEMORY FLUSH COMPLETE.');
            }
        };
    }

    processCommand(raw) {
        if (!raw.trim()) return;
        this.history.push(raw);
        this.historyIndex = -1;
        
        const parts = raw.trim().split(' ');
        const cmd = parts[0].toLowerCase();
        const args = parts.slice(1);

        this.write(`> ${raw}`, 'cmd-echo');
        
        if (this.commands[cmd]) {
            this.commands[cmd](args);
        } else {
            this.write(`Σ://ERR> COMMAND UNKNOWN: ${cmd}`);
        }

        if (this.element) this.element.value = '';
    }

    write(text, type = 'info') {
        if (!this.output) return;
        const line = document.createElement('div');
        line.className = `cli-line line-${type}`;
        line.textContent = text;
        this.output.appendChild(line);
        this.output.scrollTop = this.output.scrollHeight;
    }
}

window.SovereignTerminal = SovereignTerminal;
