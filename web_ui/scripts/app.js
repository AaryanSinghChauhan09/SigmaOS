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
    const grid = document.getElementById('lattice-grid');
    const log = document.getElementById('audit-log');
    const coverageVal = document.getElementById('coverage-val');
    
    // Initialize Grid
    suitesData.forEach(suite => {
        const card = document.createElement('div');
        card.className = 'suite-card';
        card.id = `suite-${suite.id}`;
        
        card.innerHTML = `
            <span class="s-id">SUITE // ${suite.id}</span>
            <span class="s-name">${suite.name}</span>
            <div class="s-status"></div>
        `;
        
        grid.appendChild(card);
    });

    const getTimestamp = () => {
        const date = new Date();
        return `[${date.toISOString().substring(11, 23)}]`;
    };

    const appendLog = (message, type = 'normal') => {
        const entry = document.createElement('div');
        entry.className = `log-entry ${type}`;
        entry.innerHTML = `<span class="timestamp">${getTimestamp()}</span> ${message}`;
        log.appendChild(entry);
        
        // Auto-scroll
        log.scrollTop = log.scrollHeight;
    };

    const simulateBootProcess = async () => {
        appendLog('Establishing C11 Absolute Purity Handshake...', 'system');
        await new Promise(r => setTimeout(r, 600));
        
        for (let i = 0; i < suitesData.length; i++) {
            const suite = suitesData[i];
            
            // Random delay for realism
            const delay = 50 + Math.random() * 150;
            await new Promise(r => setTimeout(r, delay));
            
            // UI Update
            const card = document.getElementById(`suite-${suite.id}`);
            card.classList.add('loaded');
            
            // Ripple effect on neighbor cards could be added here in a real reactive system
            
            // Log Update
            const hash = Math.random().toString(16).substr(2, 8);
            appendLog(`Integrity Verified: ${suite.id}_${suite.name} (0x${hash})`, 'success');
            
            // Coverage Update
            const completion = Math.round(((i + 1) / suitesData.length) * 100);
            coverageVal.textContent = `${completion}%`;
            
            // Glitch effect randomly
            if (Math.random() > 0.9) {
                appendLog(`Optimizing Neural Thread pool for ${suite.id}...`, 'warning');
            }
        }
        
        await new Promise(r => setTimeout(r, 400));
        appendLog('ALL 33 SUITES MATERIALLY HARMONIZED. ZERO HOST LEAKAGE.', 'system');
        appendLog('SOVEREIGNTY ASCENDED.', 'system');
        
        // Accelerate Hologram
        document.querySelector('.outer-ring').style.animationDuration = '3s';
        document.querySelector('.middle-ring').style.animationDuration = '2s';
        document.querySelector('.holo-core').style.boxShadow = '0 0 80px var(--acc-purple), 0 0 120px var(--acc-cyan)';
    };

    // Tab Switching
    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
            btn.classList.add('active');
            document.getElementById(btn.dataset.tab).classList.add('active');
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
                row.onclick = () => {
                    if (item.isDir) loadDirectory(item.path);
                    else viewFile(item.path);
                };
                explorerList.appendChild(row);
            });
        } catch (e) {
            explorerList.innerHTML = `<div style="color:red; padding:10px;">Error: ${e.message}</div>`;
        }
    }

    document.getElementById('btn-up-dir').onclick = () => {
        if (currentPath === '/') return;
        const parts = currentPath.split('/').filter(Boolean);
        parts.pop();
        loadDirectory('/' + parts.join('/'));
    };

    // File Viewer
    const modal = document.getElementById('file-modal');
    const modalClose = document.getElementById('btn-close-modal');
    const modalFilename = document.getElementById('modal-filename');
    const fileContent = document.getElementById('file-content');

    async function viewFile(filePath) {
        modalFilename.textContent = `FILE: ${filePath}`;
        modal.classList.add('active');
        fileContent.textContent = 'Loading source buffer...';
        
        try {
            const res = await fetch(`/api/fs?path=${encodeURIComponent(filePath)}`);
            const text = await res.text();
            fileContent.textContent = text;
        } catch (e) {
            fileContent.textContent = `Error reading file stream.`;
        }
    }

    modalClose.onclick = () => { modal.classList.remove('active'); };

    // Trigger boot & loading root dir
    setTimeout(simulateBootProcess, 1000);
    loadDirectory('/');
});
