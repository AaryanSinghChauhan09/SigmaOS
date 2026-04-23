/**
 * SigmaOS Sovereign Shell (UI)
 * Module 02: Terminal emulation and interactive Sovereign CLI.
 */

document.addEventListener("DOMContentLoaded", () => {
    const output = document.getElementById('shell-output');
    const input = document.getElementById('shell-input');
    const terminal = document.getElementById('cli-view');

    const appendOutput = (text, type = 'normal') => {
        if (!text) return;
        const div = document.createElement('div');
        div.className = `shell-line ${type}`;
        div.textContent = text;
        output.appendChild(div);
        output.scrollTop = output.scrollHeight;
    };

    const shellInterface = {
        clear: () => { output.innerHTML = ''; }
    };

    input.addEventListener('keydown', (e) => {
        if (e.key === 'Enter' && input.value.trim() !== '') {
            const val = input.value.trim();
            appendOutput(`Σ@SigmaOS:~$ ${val}`, 'command-echo');
            
            const response = SovereignShell.execute(val, shellInterface);
            appendOutput(response, 'success');
            
            input.value = '';
        }
    });

    // Handle view switches
    const observer = new MutationObserver(() => {
        if (!terminal.classList.contains('hidden')) {
            input.focus();
        }
    });
    observer.observe(terminal, { attributes: true, attributeFilter: ['class'] });

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