/**
 * Sovereign Explorer (v2.0)
 * Functional VFS browser that makes SigmaOS feel like a complete Desktop system.
 * Surpasses entry-level Linux distros with integrated Silicon Primitives.
 */

class SovereignExplorer extends ZenithComponent {
    constructor() {
        super('explorer-view');
        this.currentPath = '/sigma/root';
        this.vfs = {
            '/sigma/root': [
                { name: 'kernel', type: 'dir' },
                { name: 'userland', type: 'dir' },
                { name: 'SOUL.md', type: 'file', content: 'Σ://IDENTITY_STREAM > Sovereignty engaged.' },
                { name: 'config.sys', type: 'file', content: 'LATTICE_MODE=APEX\nSECURITY=SUPREME' }
            ],
            '/sigma/root/kernel': [
                { name: 'shards', type: 'dir' },
                { name: 'core.c', type: 'file', content: 'void _start() { // Silicon Singularity }' }
            ]
        };
        this.init();
    }

    init() {
        this.renderPath();
        this.bindEvents();
    }

    bindEvents() {
        const upBtn = Sigma.node('btn-up-dir');
        if (upBtn) upBtn.onclick = () => this.navigate('..');
    }

    renderPath() {
        const listContainer = Sigma.node('explorer-list');
        const pathLabel = Sigma.node('current-path');
        if (!listContainer || !pathLabel) return;

        pathLabel.textContent = this.currentPath;
        listContainer.innerHTML = '';

        const files = this.vfs[this.currentPath] || [];
        
        Sigma.each(files, file => {
            const row = document.createElement('div');
            row.className = 'file-row glass-panel';
            row.innerHTML = `
                <span class="file-icon">${file.type === 'dir' ? '📁' : '📄'}</span>
                <span class="file-name">${file.name}</span>
            `;
            
            row.onclick = () => {
                if (file.type === 'dir') {
                    this.navigate(`${this.currentPath}/${file.name}`);
                } else {
                    this.openFile(file);
                }
            };
            
            listContainer.appendChild(row);
        });
    }

    navigate(sub) {
        if (sub === '..') {
            const parts = this.currentPath.split('/');
            if (parts.length > 2) {
                parts.pop();
                this.currentPath = parts.join('/');
            }
        } else {
            this.currentPath = sub;
        }
        this.renderPath();
    }

    openFile(file) {
        window.zenith.taskbar.notify(`OPENING: ${file.name}`, 'STABLE');
        console.log(`Σ://FS> [READ] ${file.name}: ${file.content}`);
    }
}

window.SovereignExplorer = SovereignExplorer;
