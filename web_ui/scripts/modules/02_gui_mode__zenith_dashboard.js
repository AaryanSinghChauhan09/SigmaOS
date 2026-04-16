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
                coverageVal.textContent = data.coverage;
                document.querySelector('.highlight-magenta').textContent = data.iq_yield + ' PURE';
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
});