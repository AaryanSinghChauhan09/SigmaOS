const suitesData = [
    { id: "S01", name: "Genesis" }, { id: "S02", name: "ZenithUI" }, { id: "S03", name: "Orchestrator" },
    { id: "S04", name: "HAL" }, { id: "S05", name: "Memory" }, { id: "S06", name: "Storage" },
    { id: "S07", name: "Network" }, { id: "S08", name: "Security" }, { id: "S09", name: "Intelligence" },
    { id: "S10", name: "Registry" }, { id: "S11", name: "Virtualization" }, { id: "S12", name: "Ecosystem" },
    { id: "S13", name: "Sentience" }, { id: "S14", name: "Transcendence" }, { id: "S15", name: "DevNexus" },
    { id: "S16", name: "SoulMolding" }, { id: "S17", name: "BioNexus" }, { id: "S18", name: "QuantumLink" },
    { id: "S19", name: "SelfEvolution" }, { id: "S20", name: "GlobalVFS" }, { id: "S21", name: "EternalState" },
    { id: "S22", name: "SimulationNexus" }, { id: "S23", name: "OmniNexus" }, { id: "S24", name: "GlobalDebugger" },
    { id: "S25", name: "ZeroKernel" }, { id: "S26", name: "OmniFabric" }, { id: "S27", name: "NeuralLink" },
    { id: "S28", name: "OmniBus" }, { id: "S29", name: "LatticeMerge" }, { id: "S30", name: "Supremacy" },
    { id: "S31", name: "GlobalGovernance" }, { id: "S32", name: "UnifiedSovereignty" }, { id: "S33", name: "TerminalFulfillment" }
];

document.addEventListener('DOMContentLoaded', () => {

    /* ==============================================================
     * PARADIGM SELECTION (BOOT MENU)
     * ============================================================== */
    const overlay = document.getElementById('boot-overlay');
    const guiView = document.getElementById('gui-view');
    const cliView = document.getElementById('cli-view');

    document.getElementById('btn-gui').addEventListener('click', () => {
        overlay.classList.add('hidden');
        guiView.classList.remove('hidden');
        setTimeout(simulateBootProcess, 500);
        loadDirectory('/'); // Init GUI Explorer
    });

    document.getElementById('btn-cli').addEventListener('click', () => {
        overlay.classList.add('hidden');
        cliView.classList.remove('hidden');
        document.getElementById('cli-input').focus();
    });


    /* ==============================================================
     * GUI MODE: ZENITH DASHBOARD
     * ============================================================== */
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

    /* ==============================================================
     * BROWSER-USE: DOM SWEEP PROTOCOL
     * ============================================================== */
    function triggerHeuristicSweep() {
        const interactables = document.querySelectorAll('button, input, select, textarea, .suite-card, .v-tab, .file-row');
        const markers = [];
        interactables.forEach((el, index) => {
            const rect = el.getBoundingClientRect();
            if(rect.width === 0 || rect.height === 0 || rect.top < 0) return;
            
            const marker = document.createElement('div');
            marker.className = 'heuristic-marker';
            marker.style.top = rect.top + 'px';
            marker.style.left = rect.left + 'px';
            marker.style.width = rect.width + 'px';
            marker.style.height = rect.height + 'px';
            marker.innerHTML = `<span class="h-label">${index}</span>`;
            document.body.appendChild(marker);
            markers.push(marker);
        });
        
        // Remove after visual scan completes
        setTimeout(() => markers.forEach(m => m.remove()), 2500);
    }

    // Browser-Use Sim Logic
    const buTaskInput = document.getElementById('bu-task-input');
    const buStatusLog = document.getElementById('bu-status-log');
    const btnBuStart = document.getElementById('btn-bu-start');

    if (btnBuStart) {
        btnBuStart.addEventListener('click', () => {
            const task = buTaskInput.value.trim();
            if (!task) return;
            
            buStatusLog.innerHTML = `<div class="chat-msg" style="color:var(--acc-magenta)">[SYSTEM] Initialising browser-use Agent...</div>`;
            buStatusLog.scrollTop = buStatusLog.scrollHeight;
            buTaskInput.value = '';
            
            setTimeout(() => {
                buStatusLog.innerHTML += `<div class="chat-msg" style="color:var(--text-primary)">[AGENT] Spawning stealth browser session via Cloud...</div>`;
                buStatusLog.scrollTop = buStatusLog.scrollHeight;
            }, 800);
            
            setTimeout(() => {
                buStatusLog.innerHTML += `<div class="chat-msg ai">[ACTION] Executing DOM evaluations for heuristic task: "${task}"...</div>`;
                buStatusLog.scrollTop = buStatusLog.scrollHeight;
                triggerHeuristicSweep(); // Launch visual overlay!
            }, 1600);
            
            setTimeout(() => {
                buStatusLog.innerHTML += `<div class="chat-msg" style="color:#27c93f">[SUCCESS] Task completed. Autopilot Standing By.</div>`;
                buStatusLog.scrollTop = buStatusLog.scrollHeight;
            }, 3200);
        });
    }

    // Bytebot Sim Logic
    const btnBbConnect = document.getElementById('btn-bytebot-connect');
    const btnBbTakeover = document.getElementById('btn-bytebot-takeover');
    const bbCanvas = document.getElementById('bytebot-canvas');
    const bbTerm = document.getElementById('bb-term');

    if (btnBbConnect) {
        btnBbConnect.addEventListener('click', () => {
            btnBbConnect.style.display = 'none';
            bbCanvas.style.display = 'block';
            
            setTimeout(() => {
                bbTerm.innerHTML = `bytebot@desktop:~$ apt update<br>Hit:1 http://archive.ubuntu.com/ubuntu jammy InRelease<br>bytebot@desktop:~$ _`;
            }, 500);

            setTimeout(() => {
                bbTerm.innerHTML += `<br>[Automated] Navigating to target portal...<br>Downloading invoices... DONE.`;
                btnBbTakeover.style.display = 'block';
            }, 2000);
        });
    }

    if (btnBbTakeover) {
        btnBbTakeover.addEventListener('click', () => {
            btnBbTakeover.style.backgroundColor = 'var(--acc-magenta)';
            btnBbTakeover.style.color = '#fff';
            btnBbTakeover.textContent = 'YOU HAVE FULL CONTROL';
            bbTerm.innerHTML += `<br><span style="color:var(--acc-magenta)">[SYSTEM] Human Takeover Initated! Mouse & Keyboard unlocked.</span>`;
            document.querySelector('.takeover-indicator').style.color = '#27c93f';
        });
    }

    /* ==============================================================
     * CLI MODE: SOVEREIGN SHELL
     * ============================================================== */
    const cliOutput = document.getElementById('cli-output');
    const cliInput = document.getElementById('cli-input');
    let cliCurrentDir = '/';

    const renderCli = (html) => {
        const div = document.createElement('div');
        div.innerHTML = html;
        cliOutput.appendChild(div);
        cliOutput.scrollTop = cliOutput.scrollHeight;
    };

    cliInput.addEventListener('keydown', async (e) => {
        if (e.key === 'Enter') {
            const cmdText = cliInput.value.trim();
            cliInput.value = '';
            renderCli(`<div><span style="color:var(--acc-cyan)">root@sigma-zenith:${cliCurrentDir}#</span> ${cmdText}</div>`);
            
            if (!cmdText) return;
            const args = cmdText.split(' ');
            const cmd = args[0].toLowerCase();

            switch (cmd) {
                case 'help':
                    renderCli(`<div style="color:#aaa">SigmaOS Shell Commands:<br>ls [dir] - List files<br>cat &lt;file&gt; - Read file contents<br>cd &lt;dir&gt; - Move directory<br>clear - Clear shell<br>gui - Switch to Zenith GUI mode</div><br>`);
                    break;
                case 'clear':
                    cliOutput.innerHTML = '';
                    break;
                case 'gui':
                    cliView.classList.add('hidden');
                    guiView.classList.remove('hidden');
                    setTimeout(simulateBootProcess, 500);
                    loadDirectory('/');
                    break;
                case 'cd': {
                    let dir = args[1] || '/';
                    if (dir === '..') {
                        if (cliCurrentDir !== '/') {
                            const p = cliCurrentDir.split('/').filter(Boolean);
                            p.pop();
                            cliCurrentDir = '/' + p.join('/');
                        }
                    } else {
                        if (!dir.startsWith('/')) {
                            dir = cliCurrentDir === '/' ? `/${dir}` : `${cliCurrentDir}/${dir}`;
                        }
                        cliCurrentDir = dir;
                    }
                    try {
                        const res = await fetch(`/api/fs?path=${encodeURIComponent(cliCurrentDir)}`);
                        if (!res.ok) {
                            renderCli(`<div style="color:#ff5f56">bash: cd: ${dir}: No such file or directory</div><br>`);
                            cliCurrentDir = '/';
                        }
                    } catch(e) {
                         renderCli(`<div style="color:#ff5f56">Network error communicating with file orchestrator.</div><br>`);
                    }
                    document.querySelector('.cli-prompt').textContent = `root@sigma-zenith:${cliCurrentDir}#`;
                    break;
                }
                case 'clear':
                    cliOutput.innerHTML = '';
                    break;
                default:
                    // PILLAR 2: Run Real Native Commands
                    try {
                        const resp = await fetch('/api/run', {
                            method: 'POST',
                            body: JSON.stringify({ cmd: cmdText, cwd: cliCurrentDir })
                        });
                        
                        const reader = resp.body.getReader();
                        const decoder = new TextDecoder("utf-8");
                        
                        let currentLine = document.createElement('div');
                        currentLine.style.whiteSpace = 'pre-wrap';
                        cliOutput.appendChild(currentLine);
                        
                        while(true) {
                            const { done, value } = await reader.read();
                            if (done) break;
                            currentLine.innerHTML += decoder.decode(value).replace(/</g, '&lt;').replace(/>/g, '&gt;');
                            cliOutput.scrollTop = cliOutput.scrollHeight;
                        }
                        renderCli('<br>');
                    } catch(e) {
                        renderCli(`<div style="color:#ff5f56">Engine Execution Error: ${e.message}</div><br>`);
                    }
            }
        }
    });

    // Make sure click on CLI empty space focuses input
    cliView.addEventListener('click', () => cliInput.focus());
});
