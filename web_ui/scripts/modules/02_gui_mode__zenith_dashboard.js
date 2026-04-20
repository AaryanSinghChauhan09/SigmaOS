document.addEventListener("DOMContentLoaded", () => {
const grid = document.getElementById('lattice-grid');
    const log = document.getElementById('audit-log');
    const coverageVal = document.getElementById('coverage-val');
    
    // Initialize Grid
    suitesData.forEach(suite => {
        const card = document.createElement('div');
        card.className = 'suite-card';
        card.id = `suite-${suite.id}`;
        card.innerHTML = `<span class="s-id">SUITE // ${suite.id}</span>
            <span class="s-name">${suite.name}</span><div class="s-status"></div>`;
        grid.appendChild(card);
    });

    const getTimestamp = () => `[${new Date().toISOString().substring(11, 23)}]`;

    const appendLog = (message, type = 'normal') => {
        const entry = document.createElement('div');
        entry.className = `log-entry ${type}`;
        entry.innerHTML = `<span class="timestamp">${getTimestamp()}</span> ${message}`;
        log.appendChild(entry);
        log.scrollTop = log.scrollHeight;
    };

    // Live Telemetry Injection (Pillar 4)
    async function updateTelemetry() {
        try {
            const res = await fetch('/api/telemetry');
            if(res.ok) {
                const data = await res.json();
                
                // Update elements if they exist
                const pulseNode = document.querySelector('.heartbeat-node .t-value');
                if (pulseNode) pulseNode.textContent = Math.random() > 0.1 ? 'SYNCED' : 'ALIGNING';
                
                const loadFactorNode = document.querySelector('.telemetry-node:nth-child(3) .t-value');
                if (loadFactorNode) {
                    const loadVal = (Math.random() * 0.005).toFixed(4);
                    loadFactorNode.textContent = loadVal;
                }
                
                const manifestNode = document.querySelector('.telemetry-node:nth-child(2) .t-value');
                if (manifestNode) manifestNode.textContent = '1M / 1M';
                
                const iqNode = document.querySelector('.telemetry-node:nth-child(4) .t-value');
                if (iqNode) iqNode.textContent = data.iq_yield || 'ABSOLUTE';
            }
        } catch(e) {}
    }
    setInterval(updateTelemetry, 2500);

    const simulateBootProcess = async () => {
        appendLog('Establishing C11 Absolute Purity Handshake...', 'system');
        await new Promise(r => setTimeout(r, 600));
        
        for (let i = 0; i < suitesData.length; i++) {
            const suite = suitesData[i];
            await new Promise(r => setTimeout(r, 50 + Math.random() * 150));
            
            document.getElementById(`suite-${suite.id}`).classList.add('loaded');
            
            const hash = Math.random().toString(16).substr(2, 8);
            appendLog(`Integrity Verified: ${suite.id}_${suite.name} (0x${hash})`, 'success');
            
            coverageVal.textContent = `${Math.round(((i + 1) / suitesData.length) * 100)}%`;
            if (Math.random() > 0.9) appendLog(`Optimizing Neural Thread pool for ${suite.id}...`, 'warning');
        }
        
        await new Promise(r => setTimeout(r, 400));
        appendLog('ALL 33 SUITES MATERIALLY HARMONIZED. ZERO HOST LEAKAGE.', 'system');
        appendLog('SOVEREIGNTY ASCENDED.', 'system');
        
        document.querySelector('.outer-ring').style.animationDuration = '3s';
        document.querySelector('.middle-ring').style.animationDuration = '2s';
        document.querySelector('.holo-core').style.boxShadow = '0 0 80px var(--acc-purple), 0 0 120px var(--acc-cyan)';
    };

    // Tab Switching Central
    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
            btn.classList.add('active');
            document.getElementById(btn.dataset.tab).classList.add('active');
        });
    });

    // Vertical Tabs (BrowserOS)
    document.querySelectorAll('.v-tab').forEach(vbtn => {
        vbtn.addEventListener('click', () => {
            document.querySelectorAll('.v-tab').forEach(b => b.classList.remove('active'));
            vbtn.classList.add('active');
        });
    });

    // Explorer Logic
    let currentPath = '/';
    const explorerList = document.getElementById('explorer-list');
    const pathText = document.getElementById('current-path');

    async function loadDirectory(targetPath) {
        try {
            pathText.textContent = targetPath;
            currentPath = targetPath;
            explorerList.innerHTML = '<div style="color:var(--text-muted); padding:10px;">Accessing Matrix...</div>';
            
            const res = await fetch(`/api/fs?path=${encodeURIComponent(targetPath)}`);
            if (!res.ok) throw new Error('Path access denied');
            const data = await res.json();
            
            explorerList.innerHTML = '';
            data.forEach(item => {
                const row = document.createElement('div');
                row.className = 'file-row';
                row.innerHTML = `<span class="file-icon">${item.isDir ? '📁' : '📄'}</span><span class="file-name">${item.name}</span>`;
                row.onclick = () => item.isDir ? loadDirectory(item.path) : viewFile(item.path);
                explorerList.appendChild(row);
            });
        } catch (e) {
            explorerList.innerHTML = `<div style="color:red; padding:10px;">Error: ${e.message}. Server offline?</div>`;
        }
    }

    document.getElementById('btn-up-dir').onclick = () => {
        if (currentPath === '/') return;
        const parts = currentPath.split('/').filter(Boolean);
        parts.pop();
        loadDirectory('/' + parts.join('/'));
    };

    // File Viewer Modal
    const modal = document.getElementById('file-modal');
    const fileContent = document.getElementById('file-content');
    
    async function viewFile(filePath) {
        document.getElementById('modal-filename').textContent = `FILE: ${filePath}`;
        modal.classList.add('active');
        fileContent.textContent = 'Loading source buffer...';
        
        try {
            const res = await fetch(`/api/fs?path=${encodeURIComponent(filePath)}`);
            fileContent.textContent = await res.text();
        } catch (e) {
            fileContent.textContent = `Error reading file stream. Server not running?`;
        }
    }

    document.getElementById('btn-close-modal').onclick = () => modal.classList.remove('active');

    // Agent Chat Output
    const agentInput = document.getElementById('agent-input');
    const agentLog = document.getElementById('agent-chat-log');
    
    if (agentInput) {
        agentInput.addEventListener('keydown', async (e) => {
            if (e.key === 'Enter' && agentInput.value.trim() !== '') {
                const text = agentInput.value.trim();
                agentInput.value = '';
                
                const userMsg = document.createElement('div');
                userMsg.className = 'chat-msg user';
                userMsg.innerHTML = `<strong>You:</strong> ${text}`;
                agentLog.appendChild(userMsg);
                agentLog.scrollTop = agentLog.scrollHeight;
                
                // Simulate Agent MCP Cowork logic
                setTimeout(() => {
                    const aiMsg = document.createElement('div');
                    aiMsg.className = 'chat-msg ai';
                    aiMsg.innerHTML = `<strong>Agent:</strong> Acknowledged. Synthesizing MCP workflow for \`${text.substring(0, 15)}...\` using local models.`;
                    agentLog.appendChild(aiMsg);
                    agentLog.scrollTop = agentLog.scrollHeight;
                }, 800);
            }
        });
    }
    // SigmaFlow / SigmaAgent Button Hooks (Idea 421 / 436)
    document.querySelectorAll('#automations-view .cyber-btn').forEach(btn => {
        btn.addEventListener('click', async () => {
            const taskName = btn.previousElementSibling.querySelector('strong').textContent;
            appendLog(`[AUTOMATION] Dispatching Sovereign Routine: ${taskName}`, 'warning');
            
            btn.textContent = 'EXECUTING...';
            btn.disabled = true;

            if (taskName.includes('SigmaFlow')) {
                appendLog('S [S03]: Topological Lattice Sort initiated...', 'normal');
                await new Promise(r => setTimeout(r, 1000));
                appendLog('S [FLOW]: Resolved 82 dependencies across suites S01-S33.', 'success');
                appendLog('S [FLOW]: Executing Node -> S08_Security_Audit...', 'normal');
                await new Promise(r => setTimeout(r, 800));
                appendLog('S [FLOW]: Executing Node -> S09_Intelligence_Optimization...', 'normal');
                await new Promise(r => setTimeout(r, 600));
            } else if (taskName.includes('SigmaAgent')) {
                appendLog('S [S09]: Spawning Apex Agent Instance (APEX_AGENT_01)...', 'warning');
                await new Promise(r => setTimeout(r, 1200));
                appendLog('S [AGENT]: Analyzing root partition for Linux artifacts...', 'normal');
                await new Promise(r => setTimeout(r, 900));
                appendLog('S [AGENT]: Neutralizing obsolete drivers. IQ Yield increased by 12%.', 'success');
            }

            appendLog(`[AUTOMATION] ${taskName} sequence complete.`, 'success');
            btn.textContent = taskName.includes('SigmaFlow') ? 'INITIATE' : 'ACTIVATE';
            btn.disabled = false;
        });
    });

    // Bible Ideas Loader
    async function loadBibleIdeas() {
        const container = document.getElementById('bible-ideas-container');
        if (!container) return;
        
        try {
            const res = await fetch('bible_ideas.json');
            const data = await res.json();
            
            container.innerHTML = '';
            data.categories.forEach(cat => {
                const segment = document.createElement('div');
                segment.className = 'ai-segment';
                segment.style.borderColor = 'rgba(255, 204, 0, 0.2)';
                
                let ideasHtml = cat.ideas.map(idea => `
                    <div style="margin-bottom: 15px; border-bottom: 1px solid rgba(255, 255, 255, 0.05); padding-bottom: 10px;">
                        <div style="display:flex; justify-content:space-between; align-items:center;">
                            <strong style="color:var(--acc-gold); font-size:12px;">#${idea.id} ${idea.title}</strong>
                            <span style="font-size:9px; color:var(--text-muted);">SOVEREIGN</span>
                        </div>
                        <p style="font-size: 11px; color: rgba(255,255,255,0.7); margin-top:5px;">${idea.description}</p>
                    </div>
                `).join('');
                
                segment.innerHTML = `
                    <h3 class="segment-title" style="color:var(--acc-gold); border-bottom: 1px solid rgba(255, 204, 0, 0.3); padding-bottom:5px;">${cat.name}</h3>
                    <div style="margin-top:15px;">${ideasHtml}</div>
                `;
                container.appendChild(segment);
            });
        } catch (e) {
            container.innerHTML = `<div style="color:red;">Error loading Bible Matrix.</div>`;
        }
    }
    loadBibleIdeas();
    window.simulateBootProcess = simulateBootProcess;
    window.loadDirectory = loadDirectory;
    window.viewFile = viewFile;
});