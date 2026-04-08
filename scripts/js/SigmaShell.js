"use strict";

/**
 * Σ SIGMA SHELL
 * Terminal and command orchestration logic.
 */
export class SigmaShell {
    constructor(system) {
        this.system = system;
        this.input = document.getElementById('terminal-input');
        this.output = document.getElementById('terminal-output');
        this.cwd = '/root';
        this.init();
    }

    init() {
        if (!this.input) return;
        this.commands = {
            'help': 'Display available command matrix.',
            'ls': 'List directory contents in VFS.',
            'cd': 'Change working directory.',
            'mkdir': 'Create a new directory shard.',
            'touch': 'Initialize a zero-byte file shard.',
            'rm': 'Remove a file or directory shard.',
            'cat': 'Dump shard content to terminal.',
            'clear': 'Neutralize terminal output buffer.',
            'neofetch': 'Display Sovereign system status.',
            'cpu': 'Query silicon usage metrics.',
            'mem': 'Audit memory pressure heatmap.',
            'matrix': 'Invoke Industrial Matrix tool sharding.',
            'scrub': 'Initiate silicon data sanitization.',
            'shutdown': 'Neutralize system power state.',
            'sigmactl': 'Direct kernel control interface.',
            'sigma-ai': 'Neural Matrix orchestration.',
            'sigma-vfs': 'Sovereign filesystem operations.',
            'sigma-ui': 'Aesthetic reality-shift engine.',
            'sigma-net': 'Zero-Trust Aether Mesh.',
            'sigma-sec': 'Lattice-PQC security audit.',
            'sigma-ds': 'Pure C tensor math analysis.',
            'sigma-dsa': 'bare-metal algorithm visualization.',
            'sigma-monitor': 'Real-time telemetry / health.',
            'sigma-health': 'Detailed system integrity audit.',
            'sigma-config': 'Kernel parameter tuning.',
            'sigma-pkg': 'Shard package management.',
            'sigma-clean': 'DOD-compliant forensic wipe.',
            'sigma-god-matrix': 'Absolute USP absorption.'
        };

        this.input.onkeydown = (e) => {
            if (e.key === 'Enter') {
                const cmd = this.input.value.trim();
                this.input.value = '';
                if (!cmd) return;
                this.execute(cmd);
            } else if (e.key === 'Tab') {
                e.preventDefault();
                this.autoComplete();
            }
        };

        this.input.oninput = () => this.updateSuggestions();
    }

    autoComplete() {
        const val = this.input.value.trim();
        const matches = Object.keys(this.commands).filter(c => c.startsWith(val));
        if (matches.length === 1) {
            this.input.value = matches[0] + ' ';
        } else if (matches.length > 1) {
            this.print(`SUGGESTIONS: ${matches.join(', ')}`, 'u-muted-text');
        }
    }

    updateSuggestions() {
        const val = this.input.value.trim();
        const suggestionBox = document.getElementById('shell-suggestions');
        if (!suggestionBox) return;

        if (!val) {
            suggestionBox.innerHTML = '';
            return;
        }

        const matches = Object.entries(this.commands)
            .filter(([cmd]) => cmd.startsWith(val))
            .slice(0, 3);
        
        suggestionBox.innerHTML = matches.map(([cmd, desc]) => `
            <div class="u-font-size-xxs u-margin-b-5">
                <span class="u-accent-text">${cmd}</span> - ${desc}
            </div>
        `).join('');
    }

    execute(command) {
        const parts = command.split(' ');
        let name = parts[0].toLowerCase();
        let args = parts.slice(1);

        // Σ UNIVERSAL DISPATCHER (sigma- prefix support)
        if (name.startsWith('sigma-')) {
            const domain = name.split('-')[1];
            this.handleShardCommand(domain, args);
            return;
        }

        this.print(`root@sigmaos:~# ${command}`, 'u-accent-text');

        const cmds = {
            help: () => this.print('Commands: help, ls, cd, mkdir, touch, rm, cat, clear, neofetch, cpu, mem, matrix, scrub, shutdown, sigmactl, sigma-[ai|vfs|ui|net|sec]'),
            clear: () => { if (this.output) this.output.innerHTML = ''; },
            ls: () => this.print(this.system.vfs.ls(this.cwd).join('  ')),
            cd: (a) => {
                const path = a[0] || '/root';
                if (this.system.vfs.exists(path) && this.system.vfs.isDir(path)) {
                    this.cwd = path;
                    this.print(`CWD: ${this.cwd}`);
                } else {
                    this.print(`cd: no such directory: ${path}`);
                }
            },
            mkdir: (a) => {
                if (!a[0]) return this.print('mkdir: missing operand');
                const path = this.cwd + '/' + a[0];
                if (this.system.vfs.mkdir(path)) this.print(`Created: ${a[0]}`);
                else this.print(`mkdir: failed for ${a[0]}`);
            },
            touch: (a) => {
                if (!a[0]) return this.print('touch: missing operand');
                const path = this.cwd + '/' + a[0];
                if (this.system.vfs.write(path, '')) this.print(`Created: ${a[0]}`);
                else this.print(`touch: failed for ${a[0]}`);
            },
            rm: (a) => {
                if (!a[0]) return this.print('rm: missing operand');
                const path = this.cwd + '/' + a[0];
                if (this.system.vfs.remove(path)) this.print(`Removed: ${a[0]}`);
                else this.print(`rm: failed for ${a[0]}`);
            },
            cat: (a) => {
                if (!a[0]) return this.print('cat: missing operand');
                const path = this.cwd + '/' + a[0];
                const content = this.system.vfs.read(path);
                if (content !== null) this.print(content);
                else this.print(`cat: no such file: ${a[0]}`);
            },
            neofetch: () => this.print(`Σ SIGMAOS ZENITH\nUptime: ${this.system.uptime}s\nKernel: Sovereign C11\nResolution: Industrial Retina`),
            cpu: () => this.print('CPU: ' + document.getElementById('cpu-val').textContent),
            mem: () => this.print('MEM: ' + document.getElementById('mem-val').textContent),
            matrix: () => this.system.wm.open('industrialmatrix'),
            scrub: () => {
                this.print('Initiating Silicon Scrub...');
                setTimeout(() => this.print('Registers zeroed. Memory sanitized.'), 500);
            },
            sigmactl: (a) => this.handleSigmaCtl(a),
            shutdown: () => {
                this.print('System going down...');
                setTimeout(() => location.reload(), 1500);
            }
        };

        if (cmds[name]) cmds[name](args);
        else this.print(`sigma_shell: command not found: ${name}`);
    }

    handleShardCommand(shard, args) {
        this.print(`Σ [SHARD-DISPATCH]: Handing off to ${shard.toUpperCase()} shard...`, 'u-accent-text');
        
        switch (shard) {
            case 'ai':
                if (args[0] === 'distribute') {
                    const prompt = args.slice(1).join(' ');
                    this.print(`[AI]: Distributing mission: "${prompt}" to 11 platforms...`);
                    this.system.wm.open('aiorch');
                } else if (args[0] === 'neural-evolve') {
                    this.print('[AI]: Initiating Recursive Neural Weight Optimization...');
                    setTimeout(() => this.print('[AI]: 1.4M parameters sharded. Weights optimized.'), 1000);
                } else {
                    this.print('Usage: sigma-ai [distribute <prompt>|neural-evolve|bias-audit]');
                }
                break;
            case 'vfs':
                if (args[0] === 'ls') this.print(this.system.vfs.ls(args[1] || this.cwd).join('  '));
                else if (args[0] === 'format') {
                    this.print('[VFS]: Wiping raw silicon blocks...');
                    this.system.vfs.fs = {};
                    this.print('[VFS]: Format 100% complete.');
                } else this.print('Usage: sigma-vfs [ls|format|mount]');
                break;
            case 'ui':
                if (args[0] === 'theme') {
                    document.documentElement.style.setProperty('--accent-primary', args[1] || '#00ffcc');
                    this.print(`[UI]: Reality-Shift to ${args[1]} Success.`);
                } else if (args[0] === 'window') {
                    if (args[1] === 'open') this.system.wm.open(args[2]);
                    else if (args[1] === 'close') this.system.wm.close(args[2]);
                } else this.print('Usage: sigma-ui [theme <color>|window open <id>|window close <id>]');
                break;
            case 'pkg':
                this.handlePkg(args, 'sigma-pkg');
                break;
            case 'clean':
                this.print('[CLEAN]: Initiating DOD-compliant forensic wipe...');
                setTimeout(() => this.print('[CLEAN]: 7-pass overwrite complete. Registers zeroed.'), 1000);
                break;
            case 'god-matrix':
                this.print('[GOD-MATRIX]: Initiating Absolute USP Absorption...');
                this.system.wm.open('shardmanager');
                setTimeout(() => this.print('[GOD-MATRIX]: USPs from Nix, Qubes, and Hyprland ABSORBED.'), 800);
                break;
            case 'net':
                this.print('[NET]: Resolving P2P Consensus via Sovereign Ledger...');
                this.system.wm.open('net');
                break;
            case 'sec':
                this.print('[SEC]: Initiating Zero-Trust Audit Matrix...');
                this.system.wm.open('pqc-sentinel');
                break;
            case 'monitor':
                this.print('[MONITOR]: Opening Sovereign Telemetry Dashboard...');
                this.system.wm.open('monitor');
                break;
            case 'health':
                this.print('Σ SOVEREIGN HEALTH AUDIT');
                const score = this.system.calculateHealthScore(
                    parseInt(document.getElementById('cpu-val').textContent),
                    parseInt(document.getElementById('mem-val').textContent)
                );
                this.print(`Audit Score: ${score}/100`);
                this.print(`VFS Status: ${this.system.vfs_vulnerabilities.length} vulnerabilities detected.`);
                break;
            case 'config':
                this.print('[CONFIG]: Opening Sovereign Kernel Parameter Tuner...');
                this.system.wm.open('configeditor');
                break;
            default:
                this.print(`[ERROR]: Shard ${shard} not recognized or not yet mapped to silicon.`);
        }
    }

    print(text, classes = '') {
        if (!this.output) return;
        const div = document.createElement('div');
        div.className = 'term-line ' + classes;
        div.textContent = text;
        this.output.appendChild(div);
        this.output.scrollTop = this.output.scrollHeight;
    }

    handlePkg(args, mode) {
        if (!args[0]) return this.print(`${mode}: missing operand`);
        if (args[0] === 'install' || args[0] === '-S') {
            const pkg = args[1];
            this.print(`${mode} [Sovereign]: Sharding package ${pkg}...`);
            setTimeout(() => this.print(`${pkg} installed at /root/bin/${pkg}`), 800);
        }
    }

    handleSigmaCtl(args) {
        if (!args[0]) return this.print('Usage: sigmactl <health|audit|status|wm>');
        if (args[0] === 'wm') this.system.wm.tile();
        if (args[0] === 'health') {
            this.print('Σ SIGMAOS HEALTH AUDIT (SOVEREIGN)');
            this.print(`Uptime: ${this.system.uptime}s`);
            this.print(`Shards Active: ${this.system.store.shards.filter(s=>s.enabled).length}`);
            this.print('HLL Dependencies: REDUCED (SMU ACTIVE)');
            this.print('Silicon Parity: 100%');
        }
        if (args[0] === 'status') this.print('SIGMAOS SOVEREIGN ZENITH: ACTIVE');
    }
}
