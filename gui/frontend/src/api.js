/**
 * gui/frontend/src/api.js
 * SigmaOS REST API client — zero dependencies, pure fetch()
 */
const API_BASE = "http://127.0.0.1:8080";

async function apiCall(path, method = "GET") {
    try {
        const res = await fetch(`${API_BASE}${path}`, { method });
        const text = await res.text();
        return { ok: res.ok, status: res.status, data: text };
    } catch (e) {
        return { ok: false, status: 0, data: `Server offline: ${e.message}` };
    }
}

export const build        = ()     => apiCall("/build", "POST");
export const buildShard   = (name) => apiCall(`/build/${name}`, "POST");
export const sync         = ()     => apiCall("/sync", "POST");
export const addShard     = (name) => apiCall(`/shard/add/${name}`, "POST");
export const removeShard  = (name) => apiCall(`/shard/remove/${name}`, "DELETE");
export const setProfile   = (name) => apiCall(`/profile/set/${name}`, "POST");
export const getStatus    = ()     => apiCall("/status");
export const getShards    = ()     => apiCall("/shards").then(r => { try { r.data = JSON.parse(r.data); } catch{} return r; });
export const healthCheck  = ()     => apiCall("/health");
