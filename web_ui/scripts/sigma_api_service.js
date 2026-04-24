/**
 * =============================================================================
 * Σ SIGMAOS: VERSIONED KERNEL API SERVICE (v1.0)
 * =============================================================================
 * Strict, versioned REST-like API layer between the bare-metal kernel
 * and the Zenith Web UI.
 *
 * The Web UI communicates ONLY through this service — never by directly
 * reading kernel globals. All data is serialized as flat JSON buffers
 * (lightweight zero-copy-friendly format).
 *
 * Endpoints:
 *   /api/v1/vitals       — CPU, memory, process count, uptime
 *   /api/v1/shards       — Active shard list with status
 *   /api/v1/pools        — Per-module memory pool stats
 *   /api/v1/capabilities — Zero-trust token audit
 *   /api/v1/hal          — Registered HAL drivers
 *   /api/v1/config       — Feature flags and build metadata
 *   /api/v1/logs         — Kernel log ring buffer (last N entries)
 *
 * Design: EventSource (SSE) for real-time push, fetch() for pull.
 * =============================================================================
 */

class SigmaKernelAPI {
    constructor() {
        this.version   = '1.0.0';
        this.baseUrl   = '/api/v1';
        this._listeners = new Map();
        this._pollInterval = null;
        this._state = {
            vitals: { cpu_usage: 0, memory_usage: 0, process_count: 0, uptime_ms: 0, active_shards: 0 },
            shards: [],
            pools: [],
            hal: {},
            config: {},
            logs: [],
        };

        // Boot the simulated kernel data source
        this._bootSimulator();
    }

    /* ── Public: Fetch endpoints (Promise-based) ─────────────────────────── */

    async getVitals()       { return structuredClone(this._state.vitals); }
    async getShards()       { return structuredClone(this._state.shards); }
    async getPools()        { return structuredClone(this._state.pools); }
    async getHAL()          { return structuredClone(this._state.hal); }
    async getConfig()       { return structuredClone(this._state.config); }
    async getLogs(count=50) { return this._state.logs.slice(-count); }

    /* ── Public: Real-time subscriptions (SSE-style) ─────────────────────── */

    subscribe(channel, callback) {
        if (!this._listeners.has(channel)) {
            this._listeners.set(channel, new Set());
        }
        this._listeners.get(channel).add(callback);
        return () => this._listeners.get(channel)?.delete(callback);
    }

    _emit(channel, data) {
        this._listeners.get(channel)?.forEach(cb => {
            try { cb(data); } catch(e) { console.error(`[API] Listener error on ${channel}:`, e); }
        });
    }

    /* ── Simulated Kernel Data Source ─────────────────────────────────────── */

    _bootSimulator() {
        // Feature flags & build metadata
        this._state.config = {
            version: '12.5.0-SOVEREIGN',
            channel: 'rolling',
            arch: 'x86_64',
            features: {
                network: true, gui: true, ai: false, web3: false,
                hypervisor: false, zkp: false, observability: true, slab_pools: true,
            },
        };

        // HAL registry
        this._state.hal = {
            display: { driver: 'vga_driver', status: 'active' },
            input:   { driver: 'ps2_driver', status: 'active' },
            storage: { driver: 'ata_driver', status: 'active' },
            network: { driver: 'e1000_driver', status: 'active' },
            timer:   { driver: 'hpet_driver', status: 'active' },
            serial:  { driver: 'uart_driver', status: 'active' },
        };

        // Generate shard list from known suites
        const coreShards = [
            'S01_Genesis', 'S02_Silicon', 'S02_ZenithUI', 'S03_Orchestrator',
            'S04_HAL', 'S05_Memory', 'S06_Storage', 'S07_Network', 'S07_Scheduling',
            'S08_Security', 'S09_Intelligence', 'S10_Registry', 'S11_Virtualization',
            'S12_Ecosystem', 'S14_Transcendence', 'S15_DevNexus', 'S16_SoulMolding',
            'S17_BioNexus', 'S17_Observability', 'S21_SafeCode', 'S23_OmniNexus',
            'S24_GlobalDebugger', 'S25_ZeroKernel', 'S30_Supremacy',
        ];
        this._state.shards = coreShards.map((id, i) => ({
            id, index: i, status: 'active', pool_id: i,
            mem_used_kb: Math.floor(Math.random() * 512),
            mem_limit_kb: 4096,
        }));

        // Memory pools
        this._state.pools = coreShards.slice(0, 10).map((name, i) => ({
            id: i, name, block_size: 64,
            used_blocks: Math.floor(Math.random() * 100),
            total_blocks: 1024,
            peak_blocks: Math.floor(Math.random() * 200),
            alloc_calls: Math.floor(Math.random() * 10000),
            free_calls: Math.floor(Math.random() * 9000),
        }));

        // Start real-time vitals push
        this._startVitalsPush();
    }

    _startVitalsPush() {
        const startTime = Date.now();
        this._pollInterval = setInterval(() => {
            this._state.vitals = {
                cpu_usage:     Math.min(100, Math.max(0, 12 + Math.sin(Date.now() / 3000) * 8 + Math.random() * 5)),
                memory_usage:  Math.floor(384 + Math.random() * 128),
                process_count: 24 + Math.floor(Math.random() * 6),
                uptime_ms:     Date.now() - startTime,
                active_shards: this._state.shards.length,
                ai_burst_pred: Math.floor(8 + Math.random() * 14),
                ai_confidence: Math.floor(85 + Math.random() * 14),
            };
            this._emit('vitals', this._state.vitals);

            // Occasionally push a log entry
            if (Math.random() < 0.3) {
                const isDispatch = Math.random() < 0.5;
                let msg = '';
                let level = 'INFO';
                
                if (isDispatch) {
                    const isNpu = Math.random() < 0.7; // 70% chance NPU is available
                    msg = isNpu ? "Dispatching Tensor OP to Hardware NPU..." : "NPU busy. Falling back to CPU tensor math...";
                    level = isNpu ? 'INFO' : 'WARN';
                } else if (Math.random() < 0.2) {
                    msg = `[PROFILE] NPU: ${Math.floor(Math.random()*1000)} (240ns) | CPU: ${Math.floor(Math.random()*100)} (4600ns) | Fused Kernels: ${Math.floor(Math.random()*50)}`;
                    level = 'DEBUG';
                } else {
                    msg = ['Pool audit OK', 'Capability renewed', 'HAL heartbeat', 'Slab compaction', 'Page table walk'][Math.floor(Math.random() * 5)];
                    level = ['INFO', 'DEBUG', 'WARN'][Math.floor(Math.random() * 3)];
                }
                
                const entry = {
                    ts: new Date().toISOString(),
                    level: level,
                    module: isDispatch ? 'S07_Scheduling' : this._state.shards[Math.floor(Math.random() * this._state.shards.length)].id,
                    msg: msg,
                };
                this._state.logs.push(entry);
                if (this._state.logs.length > 500) this._state.logs.shift();
                this._emit('logs', entry);
            }
        }, 1000);
    }

    destroy() {
        if (this._pollInterval) clearInterval(this._pollInterval);
    }
}

// Singleton — the Web UI imports this
window.SigmaAPI = new SigmaKernelAPI();
