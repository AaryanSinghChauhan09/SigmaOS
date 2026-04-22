const fs = require('fs');
const path = require('path');

const modules = [
    { name: '101_container_orchestrator.js', title: 'Container Orchestrator', desc: 'Kubernetes inspired containerized tab isolation and orchestration.', cli: 'kubectl-sim' },
    { name: '102_git_version_control.js', title: 'Git Version Control', desc: 'Git inspired snapshotting and branching for workspace states.', cli: 'git-sim' },
    { name: '103_in_memory_cache.js', title: 'In-Memory Cache', desc: 'Redis inspired high-speed key-value memory for the OS.', cli: 'redis-cli' },
    { name: '104_streaming_compositor.js', title: 'Streaming Compositor', desc: 'OBS Studio inspired screen recording and broadcasting built-in.', cli: 'obs-sim' },
    { name: '105_media_transcoder.js', title: 'Media Transcoder', desc: 'FFmpeg inspired on-the-fly media manipulation.', cli: 'ffmpeg-sim' },
    { name: '106_3d_render_engine.js', title: '3D Render Engine', desc: 'Blender inspired WebGL spatial UI elements.', cli: 'render3d' },
    { name: '107_relational_database.js', title: 'Relational Database', desc: 'PostgreSQL inspired local structured data storage.', cli: 'psql-sim' },
    { name: '108_search_indexer.js', title: 'Search Indexer', desc: 'ElasticSearch inspired full-text searching across all tabs and notes.', cli: 'elastic-sim' },
    { name: '109_packet_analyzer.js', title: 'Packet Analyzer', desc: 'Wireshark inspired network traffic monitoring for tabs.', cli: 'wireshark-sim' },
    { name: '110_distributed_storage.js', title: 'Distributed Storage', desc: 'IPFS inspired peer-to-peer file sharing and storage.', cli: 'ipfs-sim' },
    { name: '111_firewall_rules_engine.js', title: 'Firewall Rules Engine', desc: 'iptables inspired granular permission control for web requests.', cli: 'iptables-sim' },
    { name: '112_message_broker.js', title: 'Message Broker', desc: 'Kafka inspired event pub/sub system between shards.', cli: 'kafka-sim' },
    { name: '113_continuous_integration.js', title: 'Continuous Integration', desc: 'Jenkins inspired automated workflow runners.', cli: 'ci-runner' },
    { name: '114_configuration_management.js', title: 'Configuration Management', desc: 'Ansible inspired declarative setup of environments.', cli: 'ansible-sim' },
    { name: '115_metrics_dashboard.js', title: 'Metrics Dashboard', desc: 'Grafana inspired advanced telemetry visualization.', cli: 'grafana-sim' },
    { name: '116_reverse_proxy.js', title: 'Reverse Proxy', desc: 'Nginx inspired local request routing and load balancing.', cli: 'nginx-sim' },
    { name: '117_machine_learning_pipeline.js', title: 'Machine Learning Pipeline', desc: 'TensorFlow inspired running local models via WebNN.', cli: 'tf-sim' },
    { name: '118_accessibility_reader.js', title: 'Accessibility Reader', desc: 'NVDA inspired advanced screen reading and navigation.', cli: 'nvda-sim' },
    { name: '119_password_manager.js', title: 'Password Manager', desc: 'Bitwarden inspired encrypted vault for credentials.', cli: 'vault-sim' },
    { name: '120_hypervisor_manager.js', title: 'Hypervisor Manager', desc: 'QEMU inspired managing virtualized sub-OS instances.', cli: 'qemu-sim' }
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
            console.log(\`Σ://OSS_ABSORB> \${this.shardId} Online. ${m.desc}\`);
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

console.log('Created Open Source inspired modules (101-120) and updated kernel_loader.js');
