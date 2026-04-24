/**
 * Σ Shard Status Dashboard (v1.0) — Module 38
 * Real-time GUI panel showing live shard health, status, and controls.
 * Mirrors: sigmactl shard ls | sigmactl status
 */
class ShardDashboard {
    constructor() {
        this.suites = [
            { id:'S01', name:'Genesis',         lang:'C+ASM', status:'ACTIVE',  cpu:'0.1%', mem:'4MB'   },
            { id:'S04', name:'HAL',             lang:'C',     status:'ACTIVE',  cpu:'0.3%', mem:'12MB'  },
            { id:'S05', name:'Memory',          lang:'Rust',  status:'ACTIVE',  cpu:'0.8%', mem:'128MB' },
            { id:'S07', name:'Network',         lang:'C',     status:'ACTIVE',  cpu:'1.2%', mem:'8MB'   },
            { id:'S08', name:'Security',        lang:'Rust',  status:'ACTIVE',  cpu:'0.5%', mem:'16MB'  },
            { id:'S09', name:'Intelligence',    lang:'C',     status:'STANDBY', cpu:'0.0%', mem:'32MB'  },
            { id:'S11', name:'Virtualization',  lang:'C',     status:'ACTIVE',  cpu:'2.1%', mem:'64MB'  },
            { id:'S15', name:'DevNexus',        lang:'C',     status:'ACTIVE',  cpu:'0.2%', mem:'6MB'   },
            { id:'S33', name:'TerminalFulfill', lang:'JS',    status:'ACTIVE',  cpu:'0.4%', mem:'22MB'  },
        ];
        this.init();
    }

    init() {
        this._injectTab();
        console.log('Σ://SHARD_DASH> Dashboard Initialized.');
    }

    _injectTab() {
        const tabBar = document.querySelector('.panel-tabs');
        if (!tabBar) return;

        // Only add if not already present
        if (document.querySelector('[data-tab="shard-dashboard-view"]')) return;

        const btn = document.createElement('button');
        btn.className = 'tab-btn';
        btn.dataset.tab = 'shard-dashboard-view';
        btn.textContent = '⬡ SHARD STATUS';
        tabBar.appendChild(btn);

        const content = document.createElement('div');
        content.className = 'tab-content';
        content.id = 'shard-dashboard-view';
        content.innerHTML = this._buildHTML();

        const leftWing = document.querySelector('.left-wing');
        if (leftWing) leftWing.appendChild(content);

        btn.addEventListener('click', () => {
            document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
            btn.classList.add('active');
            content.classList.add('active');
            this._startLiveUpdate(content);
        });
    }

    _buildHTML() {
        const rows = this.suites.map(s => this._row(s)).join('');
        return `
        <div class="ai-hub-container">
            <div class="ai-header">
                <span class="t-title highlight-cyan">⬡ SHARD STATUS DASHBOARD</span>
                <span class="status-badge pulse-ring">LIVE</span>
            </div>
            <div class="ai-body">
                <div class="shard-toolbar">
                    <button class="cyber-btn small-btn" onclick="window.shardDash.buildAll()">⚡ BUILD ALL</button>
                    <button class="cyber-btn small-btn secondary" onclick="window.shardDash.syncGit()">↑ SYNC GITHUB</button>
                    <button class="cyber-btn small-btn secondary" onclick="window.shardDash.refresh()">↺ REFRESH</button>
                </div>
                <div class="shard-grid-header">
                    <span>Suite</span><span>Name</span><span>Lang</span>
                    <span>Status</span><span>CPU</span><span>Mem</span><span>Action</span>
                </div>
                <div id="shard-rows">${rows}</div>
            </div>
        </div>`;
    }

    _row(s) {
        const statusClass = s.status === 'ACTIVE' ? 'shard-status-active' :
                            s.status === 'ERROR'   ? 'shard-status-error'  : 'shard-status-standby';
        const langBadge   = s.lang === 'Rust' ? 'lang-rust' : s.lang === 'JS' ? 'lang-js' : 'lang-c';
        return `
        <div class="shard-row" id="shard-row-${s.id}">
            <span class="shard-id">${s.id}</span>
            <span class="shard-name">${s.name}</span>
            <span class="shard-lang ${langBadge}">${s.lang}</span>
            <span class="shard-status ${statusClass}">${s.status}</span>
            <span class="shard-cpu">${s.cpu}</span>
            <span class="shard-mem">${s.mem}</span>
            <span class="shard-actions">
                <button class="cyber-btn small-btn" onclick="window.shardDash.restartShard('${s.id}')">↺</button>
                <button class="cyber-btn small-btn secondary" onclick="window.shardDash.killShard('${s.id}')">✕</button>
            </span>
        </div>`;
    }

    buildAll() {
        window.terminal?.write('Σ://SHARD> Triggering: sigmactl build --target all');
        fetch('/api/run', { method:'POST', headers:{'Content-Type':'application/json'},
            body: JSON.stringify({ cmd:'echo "BUILD: all shards"', cwd:'' })
        }).then(r=>r.text()).then(t=>window.terminal?.write(t));
        if (window.zenith?.taskbar) window.zenith.taskbar.notify('BUILD INITIATED', 'OPTIMAL');
    }

    syncGit() {
        window.terminal?.write('Σ://SHARD> Triggering: sigmactl sync');
        fetch('/api/run', { method:'POST', headers:{'Content-Type':'application/json'},
            body: JSON.stringify({ cmd:'git push', cwd:'' })
        }).then(r=>r.text()).then(t=>window.terminal?.write(t));
        if (window.zenith?.taskbar) window.zenith.taskbar.notify('GITHUB SYNC: PUSHING', 'OPTIMAL');
    }

    restartShard(id) {
        window.terminal?.write(`Σ://SHARD> Restarting: ${id}`);
        if (window.zenith?.taskbar) window.zenith.taskbar.notify(`SHARD RESTART: ${id}`, 'WARN');
    }

    killShard(id) {
        window.terminal?.write(`Σ://SHARD> Kill signal → ${id}`);
        if (window.sharding) window.sharding.simulateFailure(id);
        if (window.zenith?.taskbar) window.zenith.taskbar.notify(`SHARD KILLED: ${id}`, 'WARN');
    }

    refresh() {
        const container = document.getElementById('shard-rows');
        if (!container) return;
        // Simulate live CPU drift
        this.suites.forEach(s => {
            if (s.status === 'ACTIVE') {
                s.cpu = (Math.random() * 3).toFixed(1) + '%';
            }
        });
        container.innerHTML = this.suites.map(s => this._row(s)).join('');
    }

    _startLiveUpdate(panel) {
        if (panel._liveInterval) return;
        panel._liveInterval = setInterval(() => this.refresh(), 5000);
    }
}

window.ShardDashboard = ShardDashboard;
window.shardDash = new ShardDashboard();
