/**
 * =============================================================================
 * Σ SIGMAOS: <sigma-ai-dispatch> Web Component
 * =============================================================================
 * Zenith UI test harness to visualize the AI Scheduler's dynamic NPU/CPU dispatch.
 * Consumes data from window.SigmaAPI.
 *
 * Usage: <sigma-ai-dispatch></sigma-ai-dispatch>
 * =============================================================================
 */

class SigmaAIDispatch extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this._unsub = null;
        this._jobs = [];
    }

    connectedCallback() {
        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                    font-family: 'Inter', system-ui, sans-serif;
                    background: rgba(0,0,0,0.4);
                    border: 1px solid rgba(167, 139, 250, 0.3);
                    border-radius: 14px;
                    padding: 16px;
                }
                .header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 12px;
                }
                .title {
                    font-size: 0.75rem;
                    letter-spacing: 0.1em;
                    text-transform: uppercase;
                    color: #a78bfa;
                    font-weight: 600;
                }
                .npu-status {
                    font-size: 0.65rem;
                    padding: 4px 8px;
                    border-radius: 20px;
                    background: rgba(16, 185, 129, 0.2);
                    color: #34d399;
                    border: 1px solid rgba(16, 185, 129, 0.4);
                }
                .log-container {
                    display: flex;
                    flex-direction: column;
                    gap: 6px;
                    max-height: 200px;
                    overflow-y: auto;
                    scrollbar-width: thin;
                    scrollbar-color: rgba(255,255,255,0.1) transparent;
                }
                .job-entry {
                    display: flex;
                    justify-content: space-between;
                    font-size: 0.7rem;
                    padding: 6px 10px;
                    background: rgba(255,255,255,0.03);
                    border-radius: 6px;
                    border-left: 3px solid transparent;
                }
                .job-entry.npu { border-left-color: #a78bfa; }
                .job-entry.cpu { border-left-color: #64748b; }
                
                .job-name { color: #e2e8f0; }
                .job-target { font-weight: 600; }
                .job-target.npu { color: #c084fc; }
                .job-target.cpu { color: #94a3b8; }
            </style>
            <div class="header">
                <span class="title">Hardware-Native Intelligence Dispatcher</span>
                <span class="npu-status">● NPU ONLINE (TensorOps Ready)</span>
            </div>
            <div class="log-container" id="jobLog">
                <div style="color: rgba(255,255,255,0.3); font-size: 0.7rem; text-align: center; padding: 10px;">Waiting for tensor workloads...</div>
            </div>
        `;

        // Listen for log events that contain dispatch info
        if (window.SigmaAPI) {
            this._unsub = window.SigmaAPI.subscribe('logs', (entry) => {
                if (entry.msg.includes("Dispatching Tensor OP")) {
                    this._addJob(entry.msg.includes("NPU") ? "NPU (Hardware)" : "CPU (Fallback)", entry.msg.includes("NPU") ? "npu" : "cpu");
                }
            });
        }
    }

    disconnectedCallback() {
        if (this._unsub) this._unsub();
    }

    _addJob(targetText, typeClass) {
        const ops = ["MatMul", "Convolution", "ReLU Activation", "Average Pooling", "Softmax"];
        const op = ops[Math.floor(Math.random() * ops.length)];
        
        this._jobs.push({ name: op, target: targetText, type: typeClass });
        if (this._jobs.length > 20) this._jobs.shift();
        
        this._render();
    }

    _render() {
        const container = this.shadowRoot.getElementById('jobLog');
        container.innerHTML = this._jobs.map(job => `
            <div class="job-entry ${job.type}">
                <span class="job-name">${job.name} Workload</span>
                <span class="job-target ${job.type}">${job.target}</span>
            </div>
        `).join('');
        container.scrollTop = container.scrollHeight;
    }
}

customElements.define('sigma-ai-dispatch', SigmaAIDispatch);
