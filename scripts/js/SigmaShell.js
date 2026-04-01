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
        this.input.onkeydown = (e) => {
            if (e.key === 'Enter') {
                const cmd = this.input.value.trim();
                this.input.value = '';
                if (!cmd) return;
                this.execute(cmd);
            }
        };
    }

    execute(command) {
        const parts = command.split(' ');
        const name = parts[0].toLowerCase();
        const args = parts.slice(1);

        this.print(`root@sigmaos:~# ${command}`, 'u-accent-text');

        const cmds = {
            help: () => this.print('Commands: help, ls, cd, mkdir, touch, rm, cat, clear, neofetch, cpu, mem, matrix, scrub, shutdown, sigmactl'),
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
            apt: (a) => this.handlePkg(a, 'APT'),
            pacman: (a) => this.handlePkg(a, 'PACMAN'),
            neofetch: () => this.print(`Σ SIGMAOS ZENITH\nUptime: ${this.system.uptime}s\nKernel: Sovereign C11\nResolution: Industrial Retina`),
            cpu: () => this.print('CPU: ' + document.getElementById('cpu-val').textContent),
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
        if (args[0] === 'health') this.print(`Uptime: ${this.system.uptime}s | VFS Integrity: 100%`);
        if (args[0] === 'status') this.print('SIGMAOS SOVEREIGN ZENITH: ACTIVE');
    }
}
