document.addEventListener("DOMContentLoaded", () => {
    // SigmaOS Sovereign Window Manager (v1.0)
    // Inspiration: Puter.js, OS.js
    
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
                    <span class="w-min">_</span>
                    <span class="w-max">▢</span>
                    <span class="w-close" onclick="this.closest('.sovereign-window').remove()">✕</span>
                </div>
            </div>
            <div class="window-content">
                ${content}
            </div>
            <div class="window-resizer"></div>
        `;

        desktop.appendChild(win);
        makeDraggable(win);
        return win;
    };

    function makeDraggable(el) {
        const header = el.querySelector('.window-header');
        let pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
        
        header.onmousedown = dragMouseDown;

        function dragMouseDown(e) {
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
