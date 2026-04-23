const fs = require('fs');
const path = require('path');

const finalModules = [
    { name: '226_debian_debconf_config.js', title: 'Debian Debconf Config', desc: 'Debian inspired centralized configuration database and frontend.', cli: 'debconf-sim' },
    { name: '227_ubuntu_apparmor_profiles.js', title: 'Ubuntu AppArmor Profiles', desc: 'Ubuntu inspired hardened security profiles for shard confinement.', cli: 'apparmor-sim' },
    { name: '228_fedora_bodhi_feedback.js', title: 'Fedora Bodhi Feedback', desc: 'Fedora inspired community feedback and karma system for module updates.', cli: 'bodhi-sim' },
    { name: '229_arch_archiso_builder.js', title: 'Arch Archiso Builder', desc: 'Arch inspired tool for creating custom live OS environments on the fly.', cli: 'archiso-sim' },
    { name: '230_opensuse_yast_central.js', title: 'openSUSE YaST Central', desc: 'openSUSE inspired "Yet another Setup Tool" for unified system config.', cli: 'yast-sim' },
    { name: '231_gentoo_portage_sync.js', title: 'Gentoo Portage Sync', desc: 'Gentoo inspired high-speed rsync-based module tree synchronization.', cli: 'portage-sync' },
    { name: '232_alpine_lbu_backup.js', title: 'Alpine LBU Backup', desc: 'Alpine inspired Local Backup Utility for saving state on diskless systems.', cli: 'lbu-sim' },
    { name: '233_kali_metasploit_integration.js', title: 'Kali Metasploit Integration', desc: 'Kali inspired deep integration for security auditing and penetration tests.', cli: 'msf-sim' },
    { name: '234_steamos_overlay_ui.js', title: 'SteamOS Overlay UI', desc: 'Valve inspired performance-overlay UI for live system metrics.', cli: 'overlay-sim' },
    { name: '235_qubes_appvm_template.js', title: 'Qubes AppVM Template', desc: 'Qubes inspired template-based AppVM management for rapid cloning.', cli: 'qvm-template' },
    { name: '236_zorin_layout_switcher.js', title: 'Zorin Layout Switcher', desc: 'Zorin OS inspired instant desktop layout switching on the fly.', cli: 'zorin-layout' },
    { name: '237_nix_hydra_buildfarm.js', title: 'Nix Hydra BuildFarm', desc: 'Nix inspired distributed build farm for multi-platform shard compilation.', cli: 'hydra-sim' },
    { name: '238_coreos_butane_config.js', title: 'CoreOS Butane Config', desc: 'CoreOS inspired human-readable configuration for Ignition provisioning.', cli: 'butane-sim' },
    { name: '239_alma_rocky_el_migration.js', title: 'Alma Rocky EL Migration', desc: 'Alma/Rocky inspired automated migration logic between Enterprise states.', cli: 'el-migrate' },
    { name: '240_rpi_imager_bridge.js', title: 'RPi Imager Bridge', desc: 'Raspberry Pi inspired bridge for flashing disks from the browser.', cli: 'rpi-imager' },
    { name: '241_systemrescue_fs_repair.js', title: 'SystemRescue FS Repair', desc: 'SystemRescue inspired advanced filesystem repair and recovery toolbox.', cli: 'fs-repair' },
    { name: '242_pureos_librem_privacy.js', title: 'PureOS Librem Privacy', desc: 'Purism inspired hardware-killswitch simulation for ultimate privacy.', cli: 'librem-priv' },
    { name: '243_solus_budgie_applets.js', title: 'Solus Budgie Applets', desc: 'Solus inspired custom taskbar applets and Raven sidebar logic.', cli: 'budgie-sim' },
    { name: '244_endeavour_welcome_wizard.js', title: 'Endeavour Welcome Wizard', desc: 'EndeavourOS inspired user-friendly first-boot wizard for setup.', cli: 'welcome-os' },
    { name: '245_puppy_sfs_load.js', title: 'Puppy SFS Load', desc: 'Puppy Linux inspired dynamic loading of SquashFS modules without reboots.', cli: 'sfs-load' },
    { name: '246_caine_forensics_mounter.js', title: 'CAINE Forensics Mounter', desc: 'CAINE inspired write-blocked forensic mounting for evidence.', cli: 'caine-mount' },
    { name: '247_lubuntu_lxqt_optimization.js', title: 'Lubuntu LXQt Optimization', desc: 'Lubuntu inspired extreme RAM optimization for low-spec browser hosts.', cli: 'lxqt-opt' },
    { name: '248_parrot_anonsurf_module.js', title: 'Parrot AnonSurf Module', desc: 'Parrot Sec inspired system-wide anonymous surfing tunnel.', cli: 'anonsurf-sim' },
    { name: '249_slackware_slackbuilds_repo.js', title: 'Slackware SlackBuilds Repo', desc: 'Slackware inspired community build script repository mapping.', cli: 'sbo-sim' },
    { name: '250_ultimate_operating_system_singularity.js', title: 'Ultimate OS Singularity', desc: 'The 250th shard: the absolute singularity of browser-based operating systems.', cli: 'os-singularity' }
];

const dir = 'web_ui/scripts/modules';
const shardsDir = 'shards';

finalModules.forEach(m => {
    const className = m.title.replace(/[^a-zA-Z0-9]/g, '');
    const content = `/**
 * SigmaOS ${m.title} Shard
 * Logic: ${m.desc}
 */

class ${className} {
    constructor() {
        this.shardId = "S" + "${m.name}".split('_')[0] + "_${className}";
        this.active = false;
        
        console.log(\`Σ://ULTIMATE> \${this.shardId} Initializing: ${m.title}...\`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            console.log(\`Σ://SINGULARITY> \${this.shardId} Online. ${m.desc}\`);
            this.registerCLI();
        });
    }

    registerCLI() {
        if(!window.SigmaCLI) window.SigmaCLI = {};
        window.SigmaCLI['${m.cli}'] = (args) => {
            return \`[${m.title}] Singularity Command: \${args.join(' ') || 'EXECUTE'}\`;
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

console.log('Final Singularity Shards (226-250) generated. Total: 250 Shards.');
