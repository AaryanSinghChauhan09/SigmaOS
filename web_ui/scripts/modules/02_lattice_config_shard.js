/**
 * SigmaOS Sovereign Lattice Config (v1.0)
 * Module 02: Declarative state management inspired by NixOS.
 * Allows viewing/editing the entire OS state as a JSON manifest.
 */

const LatticeConfig = {
    state: {
        version: "EXTINCTION-1",
        suites: 33,
        securityLevel: "QUANTUM_STRICT",
        networking: "AETHER_MESH",
        theme: "ZENITH_DARK",
        shards: []
    },

    init() {
        console.log("Σ Lattice Config: Declarative Engine Online.");
    },

    openEditor() {
        // Sync with ShardOrchestrator manifest
        if (window.ShardOrchestrator) {
            this.state.shards = Array.from(ShardOrchestrator.manifest.keys());
        }

        const jsonView = JSON.stringify(this.state, null, 4);
        const content = `
            <div class="lattice-config-editor">
                <p class="t-title highlight-cyan">SOVEREIGN BLUEPRINT (DECLARATIVE)</p>
                <textarea id="lattice-json-editor" style="width:100%; height:300px; background:#000; color:var(--acc-cyan); border:1px solid #333; font-family:var(--f-mono); padding:10px;">${jsonView}</textarea>
                <div style="margin-top:10px; display:flex; gap:10px;">
                    <button class="cyber-btn" onclick="LatticeConfig.apply()">APPLY CHANGES (ATOMIC)</button>
                    <button class="cyber-btn secondary" onclick="LatticeConfig.export()">EXPORT BLUEPRINT</button>
                </div>
            </div>
        `;

        if (window.createWindow) {
            createWindow("Sovereign Blueprint", content, { width: '700px', height: '500px', icon: '📜' });
        }
    },

    apply() {
        const editor = document.getElementById('lattice-json-editor');
        try {
            const newState = JSON.parse(editor.value);
            this.state = newState;
            UIUtils.appendLog('audit-log', 'Lattice: Atomic state update SUCCESSFUL.', 'success');
            if (window.EventBus) EventBus.publish('lattice_state_modified', newState);
        } catch (e) {
            UIUtils.appendLog('audit-log', 'Lattice: Atomic update FAILED. Syntax error in blueprint.', 'danger');
        }
    },

    export() {
        const blob = new Blob([JSON.stringify(this.state, null, 2)], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'sovereign_blueprint.json';
        a.click();
    }
};

window.LatticeConfig = LatticeConfig;
document.addEventListener('DOMContentLoaded', () => LatticeConfig.init());
