/**
 * =============================================================================
 * Σ SIGMAOS: <sigma-neural-demo> Web Component
 * =============================================================================
 * Zenith UI Visual Showcase for Hardware-Native Intelligence.
 * Simulates a CNN inference pipeline (e.g., MNIST digit recognition)
 * and visualizes the dynamic dispatch decisions made by the SigmaOS AI Scheduler.
 *
 * Usage: <sigma-neural-demo></sigma-neural-demo>
 * =============================================================================
 */

class SigmaNeuralDemo extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this._interval = null;
        this._isProcessing = false;
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
                    padding: 20px;
                    margin-top: 24px;
                }
                .header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 20px;
                }
                .title {
                    font-size: 0.9rem;
                    letter-spacing: 0.1em;
                    text-transform: uppercase;
                    color: #a78bfa;
                    font-weight: 700;
                }
                .btn {
                    background: #a78bfa;
                    color: #000;
                    border: none;
                    padding: 8px 16px;
                    border-radius: 6px;
                    font-weight: 600;
                    cursor: pointer;
                    transition: all 0.2s;
                }
                .btn:hover { background: #c084fc; box-shadow: 0 0 10px rgba(192,132,252,0.5); }
                .btn:disabled { background: #475569; color: #94a3b8; cursor: not-allowed; box-shadow: none; }
                
                .pipeline {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    gap: 12px;
                }
                
                .stage {
                    flex: 1;
                    background: rgba(255,255,255,0.03);
                    border: 1px solid rgba(255,255,255,0.1);
                    border-radius: 10px;
                    padding: 16px;
                    text-align: center;
                    position: relative;
                    transition: all 0.3s ease;
                }
                .stage.active {
                    border-color: #6ee7b7;
                    background: rgba(110, 231, 183, 0.05);
                    box-shadow: 0 0 15px rgba(110, 231, 183, 0.2);
                }
                
                .stage-title {
                    font-size: 0.7rem;
                    color: #94a3b8;
                    margin-bottom: 8px;
                    text-transform: uppercase;
                }
                
                .op-type {
                    font-size: 1rem;
                    font-weight: 600;
                    color: #e2e8f0;
                }
                
                .dispatch-badge {
                    display: inline-block;
                    margin-top: 10px;
                    font-size: 0.65rem;
                    padding: 4px 8px;
                    border-radius: 12px;
                    background: rgba(255,255,255,0.1);
                    color: #cbd5e1;
                    opacity: 0;
                    transition: opacity 0.3s;
                }
                .dispatch-badge.visible { opacity: 1; }
                .dispatch-badge.npu { background: rgba(167, 139, 250, 0.2); color: #c084fc; border: 1px solid #a78bfa; }
                .dispatch-badge.cpu { background: rgba(100, 116, 139, 0.2); color: #94a3b8; border: 1px solid #64748b; }
                
                .arrow { color: rgba(255,255,255,0.2); font-size: 1.5rem; }
                
                .result-area {
                    margin-top: 24px;
                    padding-top: 20px;
                    border-top: 1px solid rgba(255,255,255,0.1);
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                }
                .result-text { font-size: 1.2rem; color: #e2e8f0; }
                .metrics { font-size: 0.8rem; color: #94a3b8; text-align: right; }
                .highlight { color: #10b981; font-weight: bold; }
            </style>

            <div class="header">
                <span class="title">Live CNN Inference Demo (TensorOps Dispatch)</span>
                <button class="btn" id="runBtn">Run Inference</button>
            </div>

            <div class="pipeline">
                <div class="stage" id="stage1">
                    <div class="stage-title">Layer 1</div>
                    <div class="op-type">Convolution 3x3</div>
                    <div class="dispatch-badge" id="badge1">WAITING</div>
                </div>
                <div class="arrow">→</div>
                <div class="stage" id="stage2">
                    <div class="stage-title">Layer 2</div>
                    <div class="op-type">ReLU Activation</div>
                    <div class="dispatch-badge" id="badge2">WAITING</div>
                </div>
                <div class="arrow">→</div>
                <div class="stage" id="stage3">
                    <div class="stage-title">Layer 3</div>
                    <div class="op-type">Max Pooling</div>
                    <div class="dispatch-badge" id="badge3">WAITING</div>
                </div>
                <div class="arrow">→</div>
                <div class="stage" id="stage4">
                    <div class="stage-title">Layer 4</div>
                    <div class="op-type">Dense MatMul</div>
                    <div class="dispatch-badge" id="badge4">WAITING</div>
                </div>
            </div>

            <div class="result-area">
                <div class="result-text" id="finalResult">Ready to classify workload.</div>
                <div class="metrics" id="timingMetrics">Total Time: -- ms <br> NPU Accelerated: --</div>
            </div>
        `;

        this.shadowRoot.getElementById('runBtn').addEventListener('click', () => this.runInference());
    }

    async runInference() {
        if (this._isProcessing) return;
        this._isProcessing = true;
        
        const btn = this.shadowRoot.getElementById('runBtn');
        const res = this.shadowRoot.getElementById('finalResult');
        const met = this.shadowRoot.getElementById('timingMetrics');
        
        btn.disabled = true;
        res.textContent = "Processing tensor pipeline...";
        met.innerHTML = "Total Time: -- ms <br> NPU Accelerated: --";

        // Reset badges
        for(let i=1; i<=4; i++) {
            const b = this.shadowRoot.getElementById('badge' + i);
            b.className = 'dispatch-badge';
            b.textContent = 'WAITING';
            this.shadowRoot.getElementById('stage' + i).classList.remove('active');
        }

        let totalTime = 0;
        let npuCount = 0;

        // Simulate the pipeline stages sequentially
        for (let i = 1; i <= 4; i++) {
            const stage = this.shadowRoot.getElementById('stage' + i);
            const badge = this.shadowRoot.getElementById('badge' + i);
            
            stage.classList.add('active');
            
            // Scheduler makes a decision (Simulated: 70% chance of NPU availability)
            const isNpu = Math.random() < 0.7;
            const execTime = isNpu ? (Math.floor(Math.random() * 5) + 2) : (Math.floor(Math.random() * 25) + 15); // NPU is faster
            
            // Push a fake log event to the global API so the other dispatch viewer sees it too
            if (window.SigmaAPI && window.SigmaAPI._state) {
                window.SigmaAPI._state.logs.push({
                    ts: new Date().toISOString(),
                    level: isNpu ? 'INFO' : 'WARN',
                    module: 'S07_Scheduling',
                    msg: isNpu ? \`Dispatching Stage \${i} to Hardware NPU...\` : \`NPU busy. Stage \${i} fallback to CPU...\`
                });
                window.SigmaAPI._emit('logs', window.SigmaAPI._state.logs[window.SigmaAPI._state.logs.length-1]);
            }

            await this._sleep(600); // Visual delay for demo purposes

            badge.textContent = isNpu ? \`NPU (\${execTime}ms)\` : \`CPU (\${execTime}ms)\`;
            badge.className = \`dispatch-badge visible \${isNpu ? 'npu' : 'cpu'}\`;
            
            if (isNpu) npuCount++;
            totalTime += execTime;
            
            stage.classList.remove('active');
        }

        // Final result
        const digits = [0,1,2,3,4,5,6,7,8,9];
        const pred = digits[Math.floor(Math.random() * digits.length)];
        
        res.innerHTML = \`Classification Complete: Digit <span class="highlight">\${pred}</span> (99.8% conf)\`;
        met.innerHTML = \`Total Execution Time: <span class="highlight">\${totalTime}ms</span> <br> NPU Accelerated: \${npuCount}/4 stages\`;
        
        this._isProcessing = false;
        btn.disabled = false;
    }

    _sleep(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }
}

customElements.define('sigma-neural-demo', SigmaNeuralDemo);
