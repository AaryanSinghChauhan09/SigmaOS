/**
 * SigmaOS Sovereign Notification Engine
 * Module 00: Lattice-wide alert dispatch and non-intrusive HUD feedback.
 */

const Notifications = {
    container: null,

    init() {
        this.container = document.createElement('div');
        this.container.id = 'notification-container';
        this.container.style.position = 'fixed';
        this.container.style.top = '20px';
        this.container.style.right = '20px';
        this.container.style.zIndex = '9999';
        this.container.style.display = 'flex';
        this.container.style.flexDirection = 'column';
        this.container.style.gap = '10px';
        document.body.appendChild(this.container);
        
        console.log("Σ Notification Engine: HUD Active.");
    },

    push(message, type = 'normal', duration = 5000) {
        const toast = document.createElement('div');
        toast.className = `notification-toast glass-panel pulse-${type}`;
        toast.innerHTML = `
            <div class="nt-icon">${type === 'success' ? '✅' : type === 'warning' ? '⚠️' : 'ℹ️'}</div>
            <div class="nt-msg">${message}</div>
        `;
        
        this.container.prepend(toast);
        
        UIUtils.appendLog('audit-log', `[NOTIFY] ${message}`, type);

        setTimeout(() => {
            toast.style.opacity = '0';
            toast.style.transform = 'translateX(20px)';
            setTimeout(() => toast.remove(), 500);
        }, duration);
    }
};

window.Notifications = Notifications;
