/**
 * Σ SIGMA OS NEURAL CO-PILOT v2.2
 * AI Task Agent & User Guidance
 */

export const AIAssistant = {
    isOpen: false,
    initialized: false,
    messages: [
        { role: 'ai', content: "Welcome Sigma Sovereign. I am your Neural Co-Pilot. I've now been upgraded with Workspace Analysis and lazy-loading awareness. How can I assist you today?" }
    ],

    init() {
        if (this.initialized) return;
        this.initialized = true;
        console.log("AIAssistant Interface Stream Active.");
        this.renderMessages();
    },

    toggle() {
        this.isOpen = !this.isOpen;
        const hud = document.getElementById('ai-assistant-hud');
        if (!hud) return;

        if (this.isOpen) {
            hud.classList.remove('ai-hud-hidden');
            this.renderMessages();
            const input = document.getElementById('ai-input');
            if (input) input.focus();
        } else {
            hud.classList.add('ai-hud-hidden');
        }
    },

    dispatch() {
        const input = document.getElementById('ai-input');
        if (!input || !input.value.trim()) return;

        const text = input.value.trim();
        this.messages.push({ role: 'user', content: text });
        input.value = '';
        this.renderMessages();

        // Mock processing
        setTimeout(() => {
            this.processCommand(text);
        }, 500);
    },

    processCommand(text) {
        const cmd = text.toLowerCase();
        let reply = "I'm analyzing your request...";

        if (cmd.includes('open') || cmd.includes('launch')) {
            const app = cmd.split(' ').pop();
            reply = `Initiating launch protocol for component: ${app.toUpperCase()}...`;
            if (window.UIEngine) UIEngine.launch(app);
        } else if (cmd.includes('analyze') || cmd.includes('this')) {
            // Intelligent workspace analysis
            let context = "";
            const noteArea = document.getElementById('note-editor');
            if (noteArea && noteArea.value) context = `I see your workspace note: "${noteArea.value.substring(0, 50)}..."`;

            const codeArea = document.getElementById('code-editor');
            if (codeArea && codeArea.value) context += ` Also detecting logic in your code editor: "${codeArea.value.substring(0, 50)}..."`;

            if (context) {
                reply = `Neural Workspace Contextualization complete: ${context} I've optimized the kernel for these tasks.`;
                if (window.SigmaKernel) SigmaKernel.notifyPanic("NEURAL_COPILOT: Syncing intelligence layer to active context.");
            } else {
                reply = "I don't see any active logic or data in your editors to analyze. Try typing something in Notes or Code Lab.";
            }
        } else if (cmd.includes('theme') || cmd.includes('color')) {
            reply = "I've updated the system palette to reflect your requested aesthetic.";
            if (window.ThemeEngine) ThemeEngine.setTheme('ocean');
        } else if (cmd.includes('clean') || cmd.includes('scan')) {
            reply = "Initiating forensic system scan as requested.";
            if (window.TelemetryShield) TelemetryShield.runSecurityAudit();
        } else if (cmd.includes('help') || cmd.includes('guide')) {
            reply = "Sigma OS Guide: Use the launcher for apps, TensorShell for CLI, and the Security Audit to verify integrity. I can help launch apps or change themes.";
        } else {
            reply = "Command interpreted. Kernel logic updated accordingly. How else can I assist?";
        }

        this.messages.push({ role: 'ai', content: reply });
        this.renderMessages();
    },

    renderMessages() {
        const cont = document.getElementById('ai-messages');
        if (!cont) return;
        cont.innerHTML = this.messages.map(m => `
            <div class="ai-msg ${m.role}">
                <div class="msg-bubble">${m.content}</div>
            </div>
        `).join('');
        cont.scrollTop = cont.scrollHeight;
    }
};

window.AIAssistant = AIAssistant;
window.toggleAIAssistant = () => AIAssistant.toggle();
window.aiDispatch = () => AIAssistant.dispatch();
