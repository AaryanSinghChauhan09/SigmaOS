/**
 * SigmaOS Filesystem Interface
 * Module 02: Sovereign Matrix access and file buffering.
 */

class PathValidator {
    static validate(path) {
        if (!path) throw new Error('Empty path');
        // Prevent directory traversal and normalize
        const normalized = path.split('/').filter(p => p && p !== '..' && p !== '.').join('/');
        return '/' + normalized;
    }
}

const SovereignFS = {
    currentPath: '/',

    async loadDirectory(targetPath, listContainerId, pathDisplayId) {
        const list = document.getElementById(listContainerId);
        const display = document.getElementById(pathDisplayId);
        
        try {
            const validated = PathValidator.validate(targetPath);
            if (!list) {
                console.error(`Σ://FS> List container not found: ${listContainerId}`);
                return;
            }
            if (display) display.textContent = validated;
            this.currentPath = validated;
            list.innerHTML = '<div style="color:var(--text-muted); padding:10px;">Accessing Matrix...</div>';
            
            const res = await fetch(`/api/fs?path=${encodeURIComponent(validated)}`);
            if (!res.ok) throw new Error('Path access denied');
            const data = await res.json();
            
            list.innerHTML = '';
            data.forEach(item => {
                const row = document.createElement('div');
                row.className = 'file-row';
                row.innerHTML = `<span class="file-icon">${item.isDir ? '📁' : '📄'}</span><span class="file-name">${item.name}</span>`;
                row.onclick = () => item.isDir ? 
                    this.loadDirectory(item.path, listContainerId, pathDisplayId) : 
                    this.viewFile(item.path);
                list.appendChild(row);
            });
        } catch (e) {
            list.innerHTML = `<div style="color:red; padding:10px;">Error: ${e.message}</div>`;
        }
    },

    async viewFile(filePath) {
        if (!window.createWindow) {
            alert("Window Manager not found. Sovereign direct-buffer access only.");
            return;
        }

        const win = window.createWindow(`VIEW: ${filePath.split('/').pop()}`, 'Loading sovereign buffer...', {
            width: '700px', height: '500px', top: (50 + Math.random() * 50) + 'px', left: (100 + Math.random() * 100) + 'px'
        });
        
        try {
            const res = await fetch(`/api/fs?path=${encodeURIComponent(filePath)}`);
            const text = await res.text();
            win.querySelector('.window-content').innerHTML = `<pre style="white-space:pre-wrap; word-break:break-all;">${text.replace(/</g, '&lt;').replace(/>/g, '&gt;')}</pre>`;
        } catch (e) {
            win.querySelector('.window-content').textContent = `Error: ${e.message}`;
        }
    },

    goUp(listContainerId, pathDisplayId) {
        if (this.currentPath === '/') return;
        const parts = this.currentPath.split('/').filter(Boolean);
        parts.pop();
        this.loadDirectory('/' + parts.join('/'), listContainerId, pathDisplayId);
    }
};

window.SovereignFS = SovereignFS;
