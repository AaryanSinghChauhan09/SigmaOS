/**
 * SigmaOS Neural Search (Omnibox)
 * Module 02: High-performance UI for Sovereign Search Engine.
 */

document.addEventListener("DOMContentLoaded", () => {
    const commandBar = document.getElementById('command-bar');
    const commandInput = document.getElementById('command-input');
    const commandResults = document.getElementById('command-results');
    
    let selectedIndex = -1;
    let currentResults = [];

    const toggleCommandBar = (show) => {
        commandBar.classList.toggle('hidden', !show);
        if (show) {
            commandInput.focus();
        } else {
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
        currentResults = SovereignSearch.query(commandInput.value);
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
        if (res.type === 'DIRECTIVE') {
            if (res.action) res.action();
        } else if (res.type === 'SUITE') {
            const el = document.getElementById(`suite-${res.item.id}`);
            if (el) {
                el.scrollIntoView({ behavior: 'smooth', block: 'center' });
                UIUtils.pulseElement(el);
            }
        }
        toggleCommandBar(false);
    };

    commandInput.addEventListener('keydown', (e) => {
        if (e.key === 'ArrowDown') {
            selectedIndex = (selectedIndex + 1) % (currentResults.length || 1);
            renderResults();
        } else if (e.key === 'ArrowUp') {
            selectedIndex = (selectedIndex - 1 + currentResults.length) % (currentResults.length || 1);
            renderResults();
        } else if (e.key === 'Enter') {
            if (selectedIndex >= 0) executeResult(currentResults[selectedIndex]);
        }
    });
});
