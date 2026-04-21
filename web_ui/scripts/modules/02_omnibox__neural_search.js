document.addEventListener("DOMContentLoaded", () => {
    const commandBar = document.getElementById('command-bar');
    const commandInput = document.getElementById('command-input');
    const commandResults = document.getElementById('command-results');
    
    let selectedIndex = -1;
    let currentResults = [];

    const toggleCommandBar = (show) => {
        if (show) {
            commandBar.classList.remove('hidden');
            commandInput.focus();
        } else {
            commandBar.classList.add('hidden');
            commandInput.value = '';
            commandResults.innerHTML = '';
        }
    };

    window.addEventListener('keydown', (e) => {
        if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
            e.preventDefault();
            toggleCommandBar(commandBar.classList.contains('hidden'));
        }
        if (e.key === 'Escape') toggleCommandBar(false);
    });

    commandInput.addEventListener('input', () => {
        const query = commandInput.value.toLowerCase().trim();
        if (!query) {
            commandResults.innerHTML = '';
            return;
        }

        // Search logic
        currentResults = [];

        // 1. Search Suites
        suitesData.forEach(s => {
            if (s.name.toLowerCase().includes(query) || s.id.toLowerCase().includes(query)) {
                currentResults.push({ type: 'SUITE', icon: '🧩', item: s });
            }
        });

        // 2. Search Static Actions
        const actions = [
            { name: 'Start Zenith GUI', icon: '🚀', action: () => window.simulateBootProcess() },
            { name: 'Drop to Shell', icon: '🐚', action: () => { 
                document.getElementById('gui-view').classList.add('hidden');
                document.getElementById('cli-view').classList.remove('hidden');
                document.getElementById('boot-overlay').classList.add('hidden');
            }},
            { name: 'Open Mission Control', icon: '📊', action: () => document.getElementById('mission-control-overlay').classList.remove('hidden') }
        ];

        actions.forEach(a => {
            if (a.name.toLowerCase().includes(query)) {
                currentResults.push({ type: 'ACTION', icon: a.icon, item: a });
            }
        });

        // 3. SigmaNLP (Natural Language Directives)
        const nlpShortcuts = [
            { phrase: 'install', cmd: 'sigpkg install', desc: 'Sovereign Package Installation' },
            { phrase: 'update',  cmd: 'sigupdate',      desc: 'System Universal Sync' },
            { phrase: 'firewall',cmd: 'sigwall --gui',  desc: 'Open Firewall Orchestrator' },
            { phrase: 'audit',   cmd: 'sig-audit',      desc: 'Security Purity Sweep' }
        ];

        nlpShortcuts.forEach(n => {
            if (query.includes(n.phrase)) {
                currentResults.push({ 
                    type: 'DIRECTIVE', 
                    icon: '⚡', 
                    item: { name: `${n.cmd}...`, desc: n.desc },
                    action: () => alert(`Sovereign NLP Executing: ${n.cmd}`) 
                });
            }
        });

        renderResults();
    });

    const renderResults = () => {
        commandResults.innerHTML = '';
        selectedIndex = currentResults.length > 0 ? 0 : -1;

        currentResults.forEach((res, i) => {
            const div = document.createElement('div');
            div.className = `command-item ${i === selectedIndex ? 'active' : ''}`;
            div.innerHTML = `
                <div class="item-info">
                    <span class="item-icon">${res.icon}</span>
                    <span class="item-name">${res.item.name}</span>
                </div>
                <span class="item-tag">${res.type}</span>
            `;
            div.onclick = () => executeResult(res);
            commandResults.appendChild(div);
        });
    };

    const executeResult = (res) => {
        if (res.type === 'ACTION' || res.type === 'DIRECTIVE') {
            if (res.action) res.action();
            else if (res.item.action) res.item.action();
        } else if (res.type === 'SUITE') {
            // Focus suite in grid
            const el = document.getElementById(`suite-${res.item.id}`);
            if (el) el.scrollIntoView({ behavior: 'smooth', block: 'center' });
            // Maybe pulse it?
            el.style.boxShadow = '0 0 30px var(--acc-cyan)';
            setTimeout(() => el.style.boxShadow = '', 1000);
        }
        toggleCommandBar(false);
    };

    commandInput.addEventListener('keydown', (e) => {
        if (e.key === 'ArrowDown') {
            selectedIndex = (selectedIndex + 1) % currentResults.length;
            renderResults();
        } else if (e.key === 'ArrowUp') {
            selectedIndex = (selectedIndex - 1 + currentResults.length) % currentResults.length;
            renderResults();
        } else if (e.key === 'Enter') {
            if (selectedIndex >= 0) executeResult(currentResults[selectedIndex]);
        }
    });
});
