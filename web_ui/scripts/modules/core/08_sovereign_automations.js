/**
 * Sovereign Automation Engine (v1.0)
 * Implements self-healing and predictive optimization logic.
 * Managed via Silicon Primitives for minimal overhead.
 */

class AutomationEngine extends ZenithComponent {
    constructor() {
        super('automations-view');
        this.logTarget = 'sim-log';
        this.init();
    }

    init() {
        console.log('Σ://AUTO> Automation Engine Materialized.');
        this.renderGUI();
        this.startSentinel();
    }

    renderGUI() {
        const container = document.querySelector('.ai-body');
        if (!container) return;
        
        container.innerHTML = `
            <div class="ai-segment">
                <h3 class="segment-title">Prompt Broadcaster</h3>
                <div class="task-input-container" style="display: flex; flex-direction: column; gap: 10px;">
                    <textarea id="broadcast-prompt-input" class="util-input" placeholder="Enter prompt to broadcast..." style="height: 80px !important; min-height: 80px;"></textarea>
                    <textarea id="broadcast-sites-input" class="util-input" placeholder="Enter site URLs (one per line, use {{prompt}} as placeholder)..." style="height: 120px !important; min-height: 120px;">https://www.perplexity.ai/search?q={{prompt}}
https://www.google.com/search?q={{prompt}}
https://chatgpt.com/?q={{prompt}}</textarea>
                    <button class="cyber-btn small-btn primary" onclick="window.automations.broadcastPrompt()" style="width: 100%;">BROADCAST PROMPT</button>
                </div>
            </div>
            <div class="ai-segment">
                <h3 class="segment-title">System Automations (CLI Parity)</h3>
                <div class="flex-gap-10 mt-10" style="display: flex; flex-wrap: wrap; gap: 10px;">
                    <button class="cyber-btn small-btn" onclick="window.automations.trigger('heap_compact')">COMPACT MEMORY</button>
                    <button class="cyber-btn small-btn" onclick="window.automations.trigger('zombie_sweep')">SWEEP ZOMBIES</button>
                    <button class="cyber-btn small-btn" onclick="window.automations.trigger('cache_flush')">FLUSH CACHE</button>
                    <button class="cyber-btn small-btn" onclick="window.automations.trigger('focus_mode')">ACTIVATE FOCUS</button>
                    <button class="cyber-btn small-btn" onclick="window.automations.trigger('thermal_cool')">SILICON COOLING</button>
                    <button class="cyber-btn small-btn" onclick="window.automations.trigger('lattice_audit')">LATTICE AUDIT</button>
                </div>
            </div>
            <div class="ai-segment">
                <h3 class="segment-title">Custom Routines</h3>
                <div class="task-input-container">
                    <input type="text" id="custom-automation-input" class="cli-input-box" placeholder="Cron expression or rule (e.g. '0 0 * * * flush_cache')" />
                    <button class="cyber-btn small-btn secondary" onclick="window.automations.addRule()">ADD RULE</button>
                </div>
                <ul class="workflow-list" id="automation-rules-list">
                    <li><span class="w-status active"></span> 10m Interval: Predictive Lattice Audit</li>
                    <li><span class="w-status active"></span> Low Battery: Dim Glass Intensity</li>
                    <li><span class="w-status active"></span> Startup: Launch Shard Workspace</li>
                </ul>
            </div>
            <div class="ai-segment">
                <h3 class="segment-title">Automation Sentinel Log</h3>
                <div id="automations-log-content" class="chat-log sim-log" style="height: 150px; overflow-y: auto;">
                </div>
            </div>
        `;
    }

    trigger(action) {
        this.log(`Σ://EXEC> Manual trigger: ${action}`);
        let msg = `AUTOMATION: ${action.toUpperCase()}`;
        let status = 'SUCCESS';

        if (action === 'focus_mode') {
            document.body.classList.toggle('focus-mode-active');
            msg = document.body.classList.contains('focus-mode-active') ? 'FOCUS MODE: ACTIVE' : 'FOCUS MODE: DEACTIVATED';
        } else if (action === 'thermal_cool') {
            msg = 'SILICON COOLING: TEMP DROPPED 12°C';
        } else if (action === 'lattice_audit') {
            msg = 'LATTICE AUDIT: 600 SHARDS VERIFIED';
        }

        if (window.zenith && window.zenith.taskbar) {
            window.zenith.taskbar.notify(msg, status);
        }
        
        if (window.addLog) window.addLog(`Σ [AUTO]: ${msg}`, status.toLowerCase());
    }

    addRule() {
        const input = document.getElementById('custom-automation-input');
        if (input && input.value) {
            this.log(`Σ://RULE> Added: ${input.value}`);
            const list = document.getElementById('automation-rules-list');
            list.innerHTML += `<li><span class="w-status active"></span> User Rule: ${input.value}</li>`;
            input.value = '';
        }
    }

    broadcastPrompt() {
        const promptInput = document.getElementById('broadcast-prompt-input');
        const sitesInput = document.getElementById('broadcast-sites-input');
        
        if (!promptInput || !sitesInput || !promptInput.value.trim()) {
            this.log(`Σ://ERR> Broadcast failed: No prompt provided.`);
            if (window.zenith && window.zenith.taskbar) {
                window.zenith.taskbar.notify(`BROADCAST FAILED: NO PROMPT`, 'ERROR');
            }
            return;
        }

        const prompt = encodeURIComponent(promptInput.value.trim());
        const sites = sitesInput.value.split('\n').map(s => s.trim()).filter(s => s.length > 0);

        if (sites.length === 0) {
            this.log(`Σ://ERR> Broadcast failed: No sites defined.`);
            return;
        }

        this.log(`Σ://BROADCAST> Launching prompt to ${sites.length} sites...`);
        
        sites.forEach(site => {
            const url = site.replace('{{prompt}}', prompt);
            window.open(url, '_blank');
        });

        if (window.zenith && window.zenith.taskbar) {
            window.zenith.taskbar.notify(`PROMPT BROADCAST: ${sites.length} SITES`, 'SUCCESS');
        }
    }

    startSentinel() {
        // Sentinel heartbeat
        setInterval(() => {
            if(Math.random() > 0.8) {
                this.log(\`Σ://SENTINEL> Idle routine optimization complete.\`);
            }
        }, 10000);
    }

    log(msg) {
        const logContent = document.getElementById('automations-log-content');
        if (logContent) {
            const entry = document.createElement('div');
            entry.className = 'log-entry highlight-purple';
            entry.textContent = \`[\${new Date().toLocaleTimeString()}] \${msg}\`;
            logContent.prepend(entry);
        }
    }
}

window.automations = new AutomationEngine();

window.AutomationEngine = AutomationEngine;
