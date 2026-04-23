document.addEventListener("DOMContentLoaded", () => {
    window.guiView = document.getElementById('gui-view');
    window.cliView = document.getElementById('cli-view');
    window.bootOverlay = document.getElementById('boot-overlay');

    selfEvolve() {
        const mutations = [
            "Optimizing lattice resonance...",
            "Expanding semantic context...",
            "Hardening silicon primitives...",
            "Refining cross-kernel synthesis..."
        ];
        const mutation = mutations[Math.floor(Math.random() * mutations.length)];
        console.log(`Σ://EVOLUTION [${this.shardId}]> ${mutation}`);
        this.lastMutation = mutation;
    }
});