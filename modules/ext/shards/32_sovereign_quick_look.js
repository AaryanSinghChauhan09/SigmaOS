/**
 * Sovereign Quick Look (v1.0)
 * Competitor USP: Instant File Preview (macOS Quick Look).
 * Enables instant inspection of VFS architecture items without spawning heavy window modules.
 * Written with zero-dependency primitive logic.
 */

class SovereignQuickLook extends ZenithComponent {
    constructor() {
        super('gui-view');
        this.overlay = null;
        this.init();
    }

    init() {
        // Create the overlay container using pure primitives
        this.overlay = document.createElement('div');
        this.overlay.className = 'quicklook-overlay hidden professional-glass';
        this.overlay.style.position = 'fixed';
        this.overlay.style.top = '10%';
        this.overlay.style.left = '10%';
        this.overlay.style.width = '80%';
        this.overlay.style.height = '80%';
        this.overlay.style.zIndex = '99999';
        this.overlay.style.padding = '20px';
        this.overlay.style.display = 'flex';
        this.overlay.style.flexDirection = 'column';
        
        Sigma.node('gui-view').appendChild(this.overlay);
        console.log('Σ://UI> Sovereign Quick Look Matrix Initialized.');
    }

    peek(path) {
        // Primitive VFS traversal (bypasses high-level find/filter)
        let resolved = 'ERROR: SHARD OR FILE NOT FOUND';
        let found = false;
        
        // Very basic mock resolution for demonstration
        if (window.explorer && window.explorer.vfs) {
            let keys = Object.keys(window.explorer.vfs);
            for (let i = 0; i < keys.length; i++) {
                let dir = window.explorer.vfs[keys[i]];
                for (let j = 0; j < dir.length; j++) {
                    if (path.indexOf(dir[j].name) !== -1 && dir[j].type === 'file') {
                        resolved = dir[j].content || '[BINARY BLOB]';
                        found = true;
                        break;
                    }
                }
                if (found) break;
            }
        }

        this.overlay.innerHTML = '<div style="display:flex; justify-content:space-between; border-bottom:1px solid rgba(255,255,255,0.2); padding-bottom:10px; margin-bottom:10px;"><h2 style="margin:0; font-family:\'Outfit\'">Σ QUICK LOOK: ' + path + '</h2><button id="ql-close" class="cyber-btn small-btn">CLOSE [Esc]</button></div><pre style="flex:1; overflow:auto; color:var(--acc-cyan); font-family:\'JetBrains Mono\'">' + resolved + '</pre>';
        
        this.overlay.classList.remove('hidden');
        
        Sigma.node('ql-close').onclick = () => {
            this.close();
        };

        window.zenith.taskbar.notify('QUICK LOOK: ' + path, 'OPTIMAL');
    }

    close() {
        this.overlay.classList.add('hidden');
        this.overlay.innerHTML = '';
        window.zenith.taskbar.notify('QUICK LOOK PURGED', 'STABLE');
    }
}

window.SovereignQuickLook = SovereignQuickLook;
