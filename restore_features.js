const REPOSITORY = [
    { id: 'dev_tools', name: 'Industrial Dev Suite', version: '4.5.0', desc: 'Sovereign C/C++ Compiler & Debugger', icon: '🛠️', installed: true },
    { id: 'net_shield', name: 'Sigma Shield Firewall', version: '1.2.9', desc: 'Kernel-level packet inspection', icon: '🛡️', installed: false },
    { id: 'office_matrix', name: 'Sigma Matrix Office', version: '3.0.2', desc: 'Distributed spreadsheet & docs', icon: '📊', installed: true },
    { id: 'bio_lab', name: 'NCERT Biology Lab', version: '1.0.0', desc: 'Sovereign biology simulations', icon: '🧬', installed: false },
    { id: 'math_lab', name: 'NCERT Maths Lab', version: '1.0.0', desc: 'Advanced math sharding', icon: '📐', installed: false },
    { id: 'android_tools', name: 'Omni Tools Android', version: '2.1.0', desc: 'Sovereign APK sharder', icon: '📱', installed: false },
    { id: 'sentinel', name: 'Sigma Sentinel', version: '3.0.0', desc: 'Real-time threat detection', icon: '👁️', installed: true },
    { id: 'theme_eng', name: 'Apex Theme Engine', version: '2.5.0', desc: 'Dynamic interface sharding', icon: '🎨', installed: true }
];

const DISTROS = [
    { id: 'ubuntu', name: 'Ubuntu Lunar', icon: '🟠', info: 'LTS Industrial Core', url: 'https://copy.sh/v86/?profile=ubuntu' },
    { id: 'arch', name: 'Arch Linux', icon: '🔵', info: 'Sovereign Rolling Release', url: 'https://copy.sh/v86/?profile=archlinux' },
    { id: 'debian', name: 'Debian 12', icon: '🔴', info: 'The Universal OS Shard', url: 'https://copy.sh/v86/?profile=debian' },
    { id: 'opensuse', name: 'openSUSE Tumbleweed', icon: '🟢', info: 'Professional Stability Shard', url: 'https://copy.sh/v86/?profile=opensuse' },
    { id: 'almalinux', name: 'AlmaLinux 9', icon: '⚪', info: 'Community Enterprise Grade', url: 'https://copy.sh/v86/?profile=almalinux' },
    { id: 'rocky', name: 'Rocky Linux 9', icon: '🛡️', info: 'RHEL-Compatible Master', url: 'https://copy.sh/v86/?profile=rocky' },
    { id: 'alpine', name: 'Alpine Linux', icon: '🏔️', info: 'Security-oriented Shard', url: 'https://copy.sh/v86/?profile=alpine' },
    { id: 'gentoo', name: 'Gentoo Linux', icon: '🟣', info: 'Source-based Sovereignty', url: 'https://copy.sh/v86/?profile=gentoo' },
    { id: 'fedora', name: 'Fedora Workstation', icon: '🧢', info: 'Cutting-Edge Sharding', url: 'https://copy.sh/v86/?profile=fedora' },
    { id: 'custom', name: 'Custom ISO/Disk', icon: '💿', info: 'Universal Shard Loader', url: '' }
];

const MATRIX_TOOLS = [
    { id: 'xclicker', name: 'Sigma XClicker', desc: 'Sovereign Auto-Clicking logic.', icon: '🖱️', USP: 'robiot/xclicker' },
    { id: 'autokey', name: 'Sigma AutoKey', desc: 'Industrial Macro Automation.', icon: '⌨️', USP: 'famousshea/autokey' },
    { id: 'merlin_ia', name: 'Sovereign AI Shard', desc: 'Autonomous system balancing.', icon: '🤖', USP: 'N1ghthill/merlin-ia' },
    { id: 'cloud_provision', name: 'vSphere Provisioner', desc: 'Industrial Infrastructure Sharding.', icon: '☁️', USP: 'miladhzzzz/vsphere-infra' },
    { id: 'script_master', name: 'Automation Playbook', desc: 'Universal Bash/Python Matrix.', icon: '📜', USP: 'muhibarshad/Linux-Automation-Scripts' },
    { id: 'ai_orchestrator', name: 'Aether Orchestrator', desc: 'Multi-model AI sharding.', icon: '🤖', USP: 'AI-Orchestrator-v2.0' },
    { id: 'spectrum_terminal', name: 'Spectrum AI Shell', desc: 'Neural command prediction.', icon: '⚡', USP: 'Spectrum-Terminal-V18' }
];

const THEMES = [
    { id: 'zenith', name: 'Zenith-Default', primary: '#00d2ff', blur: '20px' },
    { id: 'crimson', name: 'Hacker-Crimson', primary: '#ff0033', blur: '10px' },
    { id: 'lupus', name: 'Lupus-Minimal', primary: '#ffffff', blur: '5px' },
    { id: 'noir', name: 'OLED-Noir', primary: '#111111', blur: '0px' },
    { id: 'alpine', name: 'Alpine-Lite', primary: '#0d192e', blur: '2px' }
];

const VMS = [
    { id: 'node_alpha', name: 'Sovereign-Alpha-01', status: 'RUNNING', ip: '192.168.10.1' },
    { id: 'node_beta', name: 'Sovereign-Beta-02', status: 'PAUSED', ip: '192.168.10.2' }
];

// Restore Distro Runner
window.launchDistro = (id) => {
    const d = DISTROS.find(x => x.id === id);
    if (!d) return;
    const iframe = document.getElementById('distro-iframe');
    if (iframe) {
        iframe.src = d.url;
        window.SIGMA.spawnToast('Distro Shard Activated: ' + d.name);
    }
}
window.loadDistrosToDOM = () => {
    const list = document.getElementById('distro-selector');
    if (!list) return;
    list.innerHTML = DISTROS.map(d => `<div class="metric-card u-center" onclick="window.launchDistro('${d.id}')"><div>${d.icon}</div><div class="u-bold u-font-size-xs">${d.name}</div><div class="u-muted-text u-font-size-xxs">${d.info}</div></div>`).join('');
};

window.loadFeatures = () => {
    setTimeout(() => {
        window.loadDistrosToDOM();
        window.SIGMA.spawnToast('Legacy Features Fully Remapped into OOP SigmaSystem');
    }, 500);
};

document.addEventListener('DOMContentLoaded', window.loadFeatures);
