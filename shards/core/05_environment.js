/**
 * SigmaOS Sovereign Environment Engine
 * Module 05: Real-time environmental awareness and UI adaptation.
 */

const EnvEngine = {
    init() {
        this.updateEnvironment();
        setInterval(() => this.updateEnvironment(), 60000);
    },

    updateEnvironment() {
        const time = new Date().getHours();
        const display = document.getElementById('env-display');
        if (!display) return;

        let status = "OPTIMAL";
        let icon = "☀️";
        
        if (time >= 18 || time < 6) {
            status = "DARK_MODE_ACTIVE";
            icon = "🌙";
            document.body.classList.add('low-light');
        } else {
            document.body.classList.remove('low-light');
        }

        display.innerHTML = `
            <span class="env-icon">${icon}</span>
            <span class="env-status">${status}</span>
        `;
        
        console.log(`Σ Environment: System state set to [${status}]`);
    },

    simulateWeather(type) {
        UIUtils.appendLog('audit-log', `Env: Simulating environmental shift [${type}]`, 'warning');
        if (type === 'STORM') {
             UIUtils.pulseElement(document.body, '0 0 50px rgba(0, 100, 255, 0.2)');
        }
    }
};

window.EnvEngine = EnvEngine;
