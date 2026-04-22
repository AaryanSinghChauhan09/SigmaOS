/**
 * gui/frontend/src/components/ShardManager.js
 * Shard management panel — add, remove, list shards visually
 */
import { getShards, addShard, removeShard, buildShard } from '../api.js';

export function ShardManager(mountPoint) {
    let shards = [];

    async function refresh() {
        const res = await getShards();
        shards = Array.isArray(res.data) ? res.data : [];
        render();
    }

    function render() {
        const container = document.getElementById(mountPoint);
        if (!container) return;

        container.innerHTML = `
        <div class="panel-section">
            <div class="panel-toolbar">
                <input id="new-shard-input" type="text" placeholder="New shard name..." />
                <button class="cyber-btn" id="btn-shard-add">⊕ ADD</button>
                <button class="cyber-btn secondary" id="btn-shard-refresh">↺ REFRESH</button>
            </div>
            <div class="shard-list">
                ${shards.length === 0 ? '<p class="muted">No shards discovered. Is SIGMA_ROOT set?</p>' : ''}
                ${shards.map(s => `
                <div class="shard-item" data-name="${s.name}">
                    <span class="shard-item-name">${s.name}</span>
                    <span class="shard-item-lang lang-${s.lang.toLowerCase()}">${s.lang.toUpperCase()}</span>
                    <button class="cyber-btn small-btn" data-build="${s.name}">⚡ BUILD</button>
                    <button class="cyber-btn small-btn danger" data-remove="${s.name}">✕ REMOVE</button>
                </div>`).join('')}
            </div>
        </div>`;

        // Bind events
        document.getElementById('btn-shard-add')?.addEventListener('click', async () => {
            const name = document.getElementById('new-shard-input')?.value.trim();
            if (!name) return;
            const r = await addShard(name);
            window.sigmaNotify?.(r.ok ? `Shard added: ${name}` : r.data, r.ok ? 'OPTIMAL' : 'WARN');
            refresh();
        });

        document.getElementById('btn-shard-refresh')?.addEventListener('click', refresh);

        container.querySelectorAll('[data-build]').forEach(btn => {
            btn.addEventListener('click', async () => {
                const name = btn.dataset.build;
                btn.textContent = '⏳';
                const r = await buildShard(name);
                btn.textContent = r.ok ? '✓' : '✕';
                window.sigmaNotify?.(r.data, r.ok ? 'OPTIMAL' : 'WARN');
            });
        });

        container.querySelectorAll('[data-remove]').forEach(btn => {
            btn.addEventListener('click', async () => {
                const name = btn.dataset.remove;
                if (!confirm(`Remove shard: ${name}?`)) return;
                await removeShard(name);
                window.sigmaNotify?.(`Shard removed: ${name}`, 'WARN');
                refresh();
            });
        });
    }

    // Auto-init
    refresh();
    return { refresh };
}
