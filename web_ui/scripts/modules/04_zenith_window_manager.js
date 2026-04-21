/**
 * SigmaOS Sovereign Window Manager (v1.1)
 * Module 04: High-fidelity window orchestration and interaction.
 */

document.addEventListener("DOMContentLoaded", () => {
    const desktop = document.body;
    let zIndexCounter = 1000;

    window.createWindow = (title, content, options = {}) => {
        const win = document.createElement('div');
        win.className = 'sovereign-window glass-panel';
        win.style.width = options.width || '600px';
        win.style.height = options.height || '400px';
        win.style.top = options.top || '100px';
        win.style.left = options.left || '100px';
        win.style.zIndex = ++zIndexCounter;

        win.innerHTML = `
            <div class="window-header">
                <div class="window-title">
                    <span class="window-icon">💠</span>
                    <span>${title}</span>
                </div>
                <div class="window-controls">
                    <span class="w-tile" title="Auto-Tile Layout" style="cursor:pointer; margin-right:8px; font-size:10px;">🔳</span>
                    <span class="w-min">_</span>
                    <span class="w-max">▢</span>
                    <span class="w-close">✕</span>
                </div>
            </div>
            <div class="window-content">
                ${content}
            </div>
            <div class="window-resizer"></div>
        `;

        desktop.appendChild(win);
        
        // Control Hooks
        win.querySelector('.w-close').onclick = () => win.remove();
        win.querySelector('.w-tile').onclick = () => SovereignLayout.tileWindows();
        
        makeDraggable(win);
        UIUtils.appendLog('audit-log', `Window Manifested: ${title}`, 'success');
        
        return win;
    };

    function makeDraggable(el) {
        const header = el.querySelector('.window-header');
        let pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
        
        header.onmousedown = dragMouseDown;

        function dragMouseDown(e) {
            if (e.target.closest('.window-controls')) return;
            e.preventDefault();
            zIndexCounter++;
            el.style.zIndex = zIndexCounter;
            pos3 = e.clientX;
            pos4 = e.clientY;
            document.onmouseup = closeDragElement;
            document.onmousemove = elementDrag;
        }

        function elementDrag(e) {
            e.preventDefault();
            pos1 = pos3 - e.clientX;
            pos2 = pos4 - e.clientY;
            pos3 = e.clientX;
            pos4 = e.clientY;
            el.style.top = (el.offsetTop - pos2) + "px";
            el.style.left = (el.offsetLeft - pos1) + "px";
        }

        function closeDragElement() {
            document.onmouseup = null;
            document.onmousemove = null;
        }
    }
});
