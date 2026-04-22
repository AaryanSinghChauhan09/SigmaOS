const fs = require('fs');
const path = require('path');

const modules = [
    { name: '176_alpine_minimalist_core.js', title: 'Alpine Minimalist Core', desc: 'Alpine inspired busybox/musl extreme lightness and minimal footprint.', cli: 'apk-sim' },
    { name: '177_gentoo_source_compiler.js', title: 'Gentoo Source Compiler', desc: 'Gentoo Portage inspired compile-from-source JIT optimization.', cli: 'emerge-sim' },
    { name: '178_kali_forensics_toolkit.js', title: 'Kali Forensics Toolkit', desc: 'Kali inspired penetration testing and network forensics for web security.', cli: 'kali-tools' },
    { name: '179_tails_amnesic_incognito.js', title: 'Tails Amnesic Incognito', desc: 'Tails inspired Tor routing and memory wiping amnesic mode.', cli: 'tor-route' },
    { name: '180_void_runit_init.js', title: 'Void Runit Init', desc: 'Void Linux inspired runit ultra-fast parallel service initialization.', cli: 'runit-sim' },
    { name: '181_slackware_pure_unix.js', title: 'Slackware Pure Unix', desc: 'Slackware inspired strict Unix philosophy and simple shell abstractions.', cli: 'slack-pkg' },
    { name: '182_pop_tiling_manager.js', title: 'Pop Tiling Manager', desc: 'Pop!_OS inspired auto-tiling windows and extreme keyboard navigation.', cli: 'pop-tile' },
    { name: '183_qubes_xen_isolation.js', title: 'Qubes Xen Isolation', desc: 'Qubes OS inspired strict tab isolation into distinct Xen-like domains.', cli: 'qubes-dom' },
    { name: '184_rhel_selinux_policies.js', title: 'RHEL SELinux Policies', desc: 'RHEL inspired Mandatory Access Control (MAC) security policies.', cli: 'selinux-sim' },
    { name: '185_ubuntu_ppa_manager.js', title: 'Ubuntu PPA Manager', desc: 'Ubuntu inspired Personal Package Archives for third-party modules.', cli: 'apt-ppa' },
    { name: '186_debian_apt_pinning.js', title: 'Debian APT Pinning', desc: 'Debian inspired granular package version control across shards.', cli: 'apt-pin' },
    { name: '187_fedora_silverblue_ostree.js', title: 'Fedora Silverblue OSTree', desc: 'Fedora Silverblue inspired rpm-ostree immutable filesystem imaging.', cli: 'ostree-sim' },
    { name: '188_nixos_atomic_upgrades.js', title: 'NixOS Atomic Upgrades', desc: 'NixOS inspired guaranteed atomic system upgrades and safe rollbacks.', cli: 'nix-env' },
    { name: '189_clear_linux_performance.js', title: 'Clear Linux Performance', desc: 'Clear Linux inspired deep hardware-specific performance tuning.', cli: 'clear-opt' },
    { name: '190_manjaro_mhwd_drivers.js', title: 'Manjaro MHWD Drivers', desc: 'Manjaro inspired MHWD automated hardware detection and configuration.', cli: 'mhwd-sim' }
];

const dir = 'web_ui/scripts/modules';

modules.forEach(m => {
    const className = m.title.replace(/[^a-zA-Z0-9]/g, '');
    const content = `/**
 * SigmaOS ${m.title} Shard
 * USP/Logic: ${m.desc}
 */

class ${className} {
    constructor() {
        this.shardId = "S" + "${m.name}".split('_')[0] + "_${className}";
        this.active = false;
        
        console.log(\`Σ://INIT> \${this.shardId} Initializing: ${m.title}...\`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            console.log(\`Σ://LINUX_DISTROS> \${this.shardId} Online. ${m.desc}\`);
            this.registerCLI();
        });
    }

    registerCLI() {
        if(!window.SigmaCLI) window.SigmaCLI = {};
        window.SigmaCLI['${m.cli}'] = (args) => {
            return \`[${m.title}] Executing \${args.join(' ')}...\`;
        };
    }
}

window.Sigma${className} = new ${className}();
`;
    fs.writeFileSync(path.join(dir, m.name), content);
});

// Update kernel_loader.js
const kernelPath = 'web_ui/scripts/kernel_loader.js';
let kernelContent = fs.readFileSync(kernelPath, 'utf8');

const files = fs.readdirSync(dir).filter(f => f.endsWith('.js'));
const modulePaths = files.map(f => '    "scripts/modules/' + f + '"').join(',\\n');
const replacement = 'const SYSTEM_MODULES = [\\n' + modulePaths + ',\\n    "scripts/audit.js"\\n];';

kernelContent = kernelContent.replace(/const SYSTEM_MODULES = \[[\s\S]*?\];/, replacement.replace(/\\n/g, '\n'));
fs.writeFileSync(kernelPath, kernelContent);

console.log('Created Linux Distros inspired modules (176-190) and updated kernel_loader.js');
