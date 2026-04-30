/**
 * SigmaOS Paradigm Selection (UI)
 * Module 02: Booting into the Sovereign Lattice.
 */

document.addEventListener("DOMContentLoaded", () => {
    const list = document.querySelector('.paradigm-list');
    if (!list) return;

    // Render Paradigms from Engine
    list.innerHTML = '';
    ParadigmEngine.paradigms.forEach(p => {
        const item = document.createElement('div');
        item.className = 'paradigm-item glass-panel';
        item.innerHTML = `
            <div class="p-icon">${p.icon}</div>
            <div class="p-info">
                <h3>${p.name}</h3>
                <p>${p.desc}</p>
            </div>
        `;
        item.onclick = () => ParadigmEngine.switchTo(p.id);
        list.appendChild(item);
    });

    // Auto-enter logic (Demo)
    setTimeout(() => {
        UIUtils.appendLog('audit-log', 'Lattice Ready. Select Paradigm to Transcend.', 'system');
    }, 1000);
});
