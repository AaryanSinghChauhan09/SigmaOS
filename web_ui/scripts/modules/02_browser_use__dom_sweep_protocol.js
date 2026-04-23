document.addEventListener("DOMContentLoaded", () => {
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

    // Pillar 1: Browser-Use Python Bridging
    const buTaskInput = document.getElementById('bu-task-input');
    const buStatusLog = document.getElementById('bu-status-log');
    const btnBuStart = document.getElementById('btn-bu-start');

    if (btnBuStart) {
        btnBuStart.addEventListener('click', async () => {
            const task = buTaskInput.value.trim();
            if (!task) return;
            
            buStatusLog.innerHTML = `<div class="chat-msg" style="color:var(--acc-magenta)">[SYSTEM] Activating Python Orchestrator...</div>`;
            buTaskInput.value = '';
            
            triggerHeuristicSweep(); // Visual overlay

            try {
                const resp = await fetch('/api/run', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ cmd: `python tools/agent_orchestrator.py --mode browser-use --task "${task}"`, cwd: '/' })
                });
                
                const reader = resp.body.getReader();
                const decoder = new TextDecoder("utf-8");
                
                while(true) {
                    const { done, value } = await reader.read();
                    if (done) break;
                    const textLines = decoder.decode(value).split('\n').filter(l => l.trim().length > 0);
                    textLines.forEach(line => {
                        // Colour-code outputs based on tags mimicking real logs
                        let color = "var(--text-primary)";
                        if(line.includes('[ACTION]')) color = "var(--acc-cyan)";
                        if(line.includes('[SUCCESS]')) color = "#27c93f";
                        if(line.includes('[SYSTEM]')) color = "var(--text-muted)";
                        
                        buStatusLog.innerHTML += `<div class="chat-msg" style="color:${color}">${line}</div>`;
                        buStatusLog.scrollTop = buStatusLog.scrollHeight;
                    });
                }
            } catch(e) {
                buStatusLog.innerHTML += `<div class="chat-msg" style="color:red">Backend API connection severed.</div>`;
            }
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

    // Pillar 5: Sigma Vault (App Store) Loader
    const vaultContainer = document.querySelector('.vault-container');
    if (vaultContainer) {
        fetch('/sigma_vault.json')
            .then(res => res.json())
            .then(data => {
                data.packages.forEach(pkg => {
                    const card = document.createElement('div');
                    card.style.cssText = `
                        background: rgba(20, 20, 25, 0.7);
                        border: 1px solid var(--glass-border);
                        border-radius: 12px;
                        padding: 15px;
                        display: flex;
                        flex-direction: column;
                        justify-content: space-between;
                        transition: all 0.3s ease;
                        cursor: pointer;
                    `;
                    card.onmouseover = () => card.style.borderColor = 'var(--acc-magenta)';
                    card.onmouseout = () => card.style.borderColor = 'var(--glass-border)';

                    card.innerHTML = `
                        <div>
                            <div style="font-size:0.8rem; color:var(--acc-cyan); margin-bottom:5px;">${pkg.category}</div>
                            <div style="font-weight:bold; font-size:1.1rem; margin-bottom:10px;">${pkg.name}</div>
                            <div style="font-size:0.85rem; color:var(--text-muted); line-height:1.4;">${pkg.description}</div>
                        </div>
                        <div style="display:flex; justify-content:space-between; align-items:center; margin-top:20px;">
                            <span style="font-size:0.8rem; color:var(--glass-border);">[ ${pkg.size_mb} MB ] &nbsp; <span style="color:var(--acc-magenta); font-size:0.75rem;">DORMANT</span></span>
                            <button class="sys-btn glow-cyan vault-emulate-btn" data-pkg-id="${pkg.id}" data-pkg-name="${pkg.name}" style="padding: 5px 15px; font-size:0.8rem;">EMULATE</button>
                        </div>
                    `;
                    vaultContainer.appendChild(card);
                });
            })
            .catch(err => console.log('Vault DB offline.'));

        // Event delegation for EMULATE buttons
        vaultContainer.addEventListener('click', async (e) => {
            const btn = e.target.closest('.vault-emulate-btn');
            if (!btn) return;
            const id = btn.dataset.pkgId;
            const name = btn.dataset.pkgName;
            btn.disabled = true;
            btn.textContent = 'Fetching...';
            try {
                const resp = await fetch(`/api/download/${encodeURIComponent(id)}`);
                if (resp.ok) {
                    btn.textContent = 'Decompressing...';
                    await resp.arrayBuffer(); // consume the stream
                    btn.textContent = '✓ Emulating';
                    btn.style.borderColor = '#27c93f';
                    btn.style.color = '#27c93f';
                } else {
                    btn.textContent = 'Error';
                    btn.style.color = 'red';
                }
            } catch (err) {
                btn.textContent = 'Offline';
                btn.disabled = false;
            }
        });
    }

    selfEvolve() {
        const mutations = [
            "Optimizing lattice resonance...",
            "Expanding semantic context...",
            "Hardening silicon primitives...",
            "Refining cross-kernel synthesis..."
        ];
        const mutation = mutations[Math.floor(Math.random() * mutations.length)];
        console.log(`Σ://EVOLUTION [${this.shardId}]> ${mutation}`);
        this.lastMutation = mutation;
    }
});