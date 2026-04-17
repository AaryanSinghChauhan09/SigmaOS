document.addEventListener("DOMContentLoaded", () => {
    // Sigma Nexus App Store Module
    // Zero dependencies (Vanilla JS only)
    
    const storeContainer = document.getElementById('pkg-manager-view');
    if (!storeContainer) return;

    // We clear the inner grid logic and render dynamically
    const appsList = [
        {
            id: 'app-fx-quantum',
            icon: '🦊',
            name: 'Firefox Quantum',
            meta: 'Sovereign Hook Enabled • 120MB',
            desc: 'A natively accelerated, 0-copy network browser compiled against the SigmaOS graphics lattice.',
            delay: 1500
        },
        {
            id: 'app-vscode',
            icon: '💻',
            name: 'VS Code Sovereign',
            meta: 'Development • 300MB',
            desc: 'Fully integrated IDE accessing all hardware shards directly with zero overhead.',
            delay: 2000
        },
        {
            id: 'app-spotify',
            icon: '🎵',
            name: 'Spotify Native',
            meta: 'Media • 80MB',
            desc: 'Direct audio lattice client. Bypasses ALSA/PulseAudio entirely for bit-perfect rendering.',
            delay: 1200
        },
        {
            id: 'app-docker',
            icon: '🐋',
            name: 'Docker Shard',
            meta: 'Virtualization • 150MB',
            desc: 'Emulate traditional Linux containers dynamically inside native SigmaOS hardware memory namespacing.',
            delay: 0,
            installed: true
        }
    ];

    function renderStore() {
        let gridHtml = '';
        appsList.forEach(app => {
            const btnState = app.installed 
                ? '<button class="cyber-btn small-btn secondary" style="align-self: flex-start; padding: 6px 16px; font-size: 12px; border-radius: 2px;">INSTALLED</button>'
                : `<button class="cyber-btn small-btn" style="align-self: flex-start; padding: 6px 16px; font-size: 12px; border-radius: 2px;" onclick="this.innerHTML='DOWNLOADING...'; setTimeout(() => { this.innerHTML='INSTALLED'; this.classList.add('secondary'); }, ${app.delay});">DOWNLOAD</button>`;
                
            gridHtml += `
            <div class="glass-panel" style="padding: 15px; display: flex; flex-direction: column; gap: 10px; background: rgba(0, 0, 0, 0.4);">
                <div style="display: flex; gap: 15px; align-items: center;">
                    <div style="font-size: 30px;">${app.icon}</div>
                    <div>
                        <h4 style="color: #fff; font-size: 14px;">${app.name}</h4>
                        <span style="font-size: 10px; color: var(--text-muted);">${app.meta}</span>
                    </div>
                </div>
                <p style="font-size: 11px; color: #ccc;">${app.desc}</p>
                ${btnState}
            </div>`;
        });

        const storeHtml = `
        <div class="ai-hub-container">
            <div class="ai-header" style="justify-content: flex-start; gap: 20px;">
                <span class="t-title highlight-cyan" style="font-size: 16px;">SIGMA NEXUS STORE</span>
                <input type="text" class="cli-input-box" placeholder="Search for applications, libraries, suites..." style="flex:1; max-width: 300px; padding: 5px 10px; border-radius: 20px;">
            </div>
            <div class="ai-body" style="padding-top:10px;">
                <h3 class="segment-title" style="margin-bottom: 10px;">FEATURED SOVEREIGN APPS</h3>
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 15px;" id="nexus-app-grid">
                    ${gridHtml}
                </div>
            </div>
        </div>`;
        
        storeContainer.innerHTML = storeHtml;
    }

    // Initialize Store
    renderStore();
});
