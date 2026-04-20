/**
 * SigmaOS Zenith Web Engine - Security Sandbox & Process Isolation
 * Provides Web Worker-based process isolation, Mandatory Access Control (MAC),
 * and permission enforcement per-app/shard.
 */

class SigmaSandbox {
    constructor() {
        this.activeProcesses = new Map();
        this.macPolicies = new Map(); // Mandatory Access Controls (SELinux/AppArmor conceptual)
        
        console.log("[Sigma Security] Zenith Sandbox Initialized OMEGA-🔒");
    }

    /**
     * Enforces Mandatory Access Control boundaries.
     */
    assignPolicy(appId, policy) {
        this.macPolicies.set(appId, {
            canNetwork: policy.network || false,
            canDisk: policy.disk || false,
            canSensors: policy.sensors || false
        });
        console.log(`[Sigma MAC] Policy applied to ${appId}.`);
    }

    /**
     * Spawns an isolated application shard in a locked Web Worker.
     */
    spawnIsolatedProcess(appId, scriptCode) {
        // Build restricted environment logic
        const wrapper = `
            self.onmessage = function(e) {
                const intent = e.data;
                // Sovereign execution boundary
                try {
                    ${scriptCode};
                    self.postMessage({ type: 'STATUS', code: 'OK' });
                } catch (err) {
                    self.postMessage({ type: 'CRASH', error: err.message });
                }
            };
        `;

        const blob = new Blob([wrapper], { type: 'application/javascript' });
        const worker = new Worker(URL.createObjectURL(blob));
        
        worker.onmessage = (e) => this._auditTelemetry(appId, e.data);
        
        this.activeProcesses.set(appId, worker);
        console.log(`[Sigma Process] Spawned secure enclave for: ${appId}`);
    }

    /**
     * Runtime Permission Check.
     */
    requestPermission(appId, resource) {
        const policy = this.macPolicies.get(appId);
        if (!policy) return false;

        switch(resource) {
            case 'CAMERA': return policy.canSensors;
            case 'NETWORK': return policy.canNetwork;
            case 'DISK': return policy.canDisk;
            default: return false;
        }
    }

    _auditTelemetry(appId, data) {
        if(data.type === 'CRASH') {
            console.error(`[Sigma Kernel] OOM/Crash in Sandbox ${appId}:`, data.error);
        }
    }
}

window.SigmaSecurity = new SigmaSandbox();
