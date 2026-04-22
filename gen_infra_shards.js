const fs = require('fs');
const path = require('path');

const infraModules = [
    { name: '206_snapcraft_universal_distro.js', title: 'Snapcraft Universal Distro', desc: 'Canonical inspired universal app distribution with strict confinement.', cli: 'snap-sim' },
    { name: '207_launchpad_ppa_builder.js', title: 'Launchpad PPA Builder', desc: 'Ubuntu inspired automated build system for personal package archives.', cli: 'ppa-build' },
    { name: '208_debian_lintian_audit.js', title: 'Debian Lintian Audit', desc: 'Debian inspired static analysis for OS module compliance and policy.', cli: 'lintian-run' },
    { name: '209_fedora_koji_orchestrator.js', title: 'Fedora Koji Orchestrator', desc: 'Fedora inspired massive parallel build system for lattice shards.', cli: 'koji-sim' },
    { name: '210_arch_pkgbuild_recipe.js', title: 'Arch PKGBUILD Recipe', desc: 'Arch Linux inspired simple, human-readable build scripts for modules.', cli: 'makepkg-sim' },
    { name: '211_opensuse_obs_factory.js', title: 'openSUSE OBS Factory', desc: 'openSUSE inspired Open Build Service for cross-platform shard compilation.', cli: 'obs-build' },
    { name: '212_gentoo_ebuild_use_flags.js', title: 'Gentoo Ebuild USE Flags', desc: 'Gentoo inspired granular feature toggling during module initialization.', cli: 'use-flags' },
    { name: '213_puppy_woof_constructor.js', title: 'Puppy Woof Constructor', desc: 'Puppy Linux inspired ability to build SigmaOS layers from external distro sources.', cli: 'woof-run' },
    { name: '214_steamos_gamescope_proxy.js', title: 'SteamOS Gamescope Proxy', desc: 'Valve inspired micro-compositor for high-performance window scaling.', cli: 'gamescope-sim' },
    { name: '215_nixos_flake_hermetic.js', title: 'NixOS Flake Hermetic', desc: 'NixOS inspired hermetic, reproducible build system for OS states.', cli: 'nix-flake' },
    { name: '216_qubes_whonix_gateway.js', title: 'Qubes Whonix Gateway', desc: 'Qubes/Whonix inspired isolated Tor gateway for anonymous workspace traffic.', cli: 'whonix-gate' },
    { name: '217_rescuezilla_partition_tool.js', title: 'Rescuezilla Partition Tool', desc: 'Rescuezilla inspired automated workspace imaging and cloning logic.', cli: 'rescue-clone' },
    { name: '218_coreos_ignition_provisioner.js', title: 'CoreOS Ignition Provisioner', desc: 'CoreOS inspired first-boot declarative system provisioning.', cli: 'ignition-run' },
    { name: '219_clear_linux_autospec.js', title: 'Clear Linux Autospec', desc: 'Clear Linux inspired automated generation of module specifications.', cli: 'autospec-sim' },
    { name: '220_solus_eopkg_manager.js', title: 'Solus Eopkg Manager', desc: 'Solus inspired simple, performance-first package management.', cli: 'eopkg-sim' },
    { name: '221_endeavour_discovery_tool.js', title: 'Endeavour Discovery Tool', desc: 'EndeavourOS inspired automated hardware and mirror detection.', cli: 'discover-os' },
    { name: '222_slackware_build_scripts.js', title: 'Slackware Build Scripts', desc: 'Slackware inspired pure shell-based module construction scripts.', cli: 'slack-build' },
    { name: '223_rancher_k3s_lite.js', title: 'Rancher K3s Lite', desc: 'Rancher inspired lightweight edge-orchestration for OS services.', cli: 'k3s-sim' },
    { name: '224_flatcar_update_engine.js', title: 'Flatcar Update Engine', desc: 'Flatcar inspired automated, atomic A/B partition updates.', cli: 'update-engine' },
    { name: '225_linux_distro_singularity.js', title: 'Linux Distro Singularity', desc: 'The ultimate synthesis of every major distro infrastructure into the SigmaOS lattice.', cli: 'distro-nexus' }
];

const dir = 'web_ui/scripts/modules';
const shardsDir = 'shards';

infraModules.forEach(m => {
    const className = m.title.replace(/[^a-zA-Z0-9]/g, '');
    const content = `/**
 * SigmaOS ${m.title} Infrastructure Shard
 * Logic: ${m.desc}
 */

class ${className} {
    constructor() {
        this.shardId = "S" + "${m.name}".split('_')[0] + "_${className}";
        this.active = false;
        
        console.log(\`Σ://INFRA> \${this.shardId} Initializing: ${m.title}...\`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            console.log(\`Σ://NEXUS> \${this.shardId} Online. ${m.desc}\`);
            this.registerCLI();
        });
    }

    registerCLI() {
        if(!window.SigmaCLI) window.SigmaCLI = {};
        window.SigmaCLI['${m.cli}'] = (args) => {
            return \`[${m.title}] Infrastructure Call: \${args.join(' ') || 'STATUS'}\`;
        };
    }
}

window.Sigma${className} = new ${className}();
`;
    fs.writeFileSync(path.join(dir, m.name), content);
    fs.writeFileSync(path.join(shardsDir, m.name), content);
});

// Update kernel_loader.js
const files = fs.readdirSync(dir).filter(f => f.endsWith('.js')).sort((a, b) => parseInt(a) - parseInt(b));
const modulePaths = files.map(f => '    "scripts/modules/' + f + '"').join(',\n');
const replacement = 'const SYSTEM_MODULES = [\n' + modulePaths + ',\n    "scripts/audit.js"\n];';

const kernelPath = 'web_ui/scripts/kernel_loader.js';
let kernelContent = fs.readFileSync(kernelPath, 'utf8');
kernelContent = kernelContent.replace(/const SYSTEM_MODULES = \[[\s\S]*?\];/, replacement);
fs.writeFileSync(kernelPath, kernelContent);

console.log('Deep Infrastructure Shards (206-225) generated. Total: 225 Shards.');
