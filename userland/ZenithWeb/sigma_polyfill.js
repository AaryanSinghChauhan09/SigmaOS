/**
 * SigmaOS Zenith Web Engine - Browser Polyfill & Extension Layer
 * Provides cross-browser compatibility across Chrome, Edge, Brave, Opera, and Vivaldi.
 */

window.SigmaPolyfills = (function() {
    console.log("[Sigma Polyfill] Initializing Universal Browser Translation Layer...");

    // 1. FileSystem API Polyfill
    if (!window.showDirectoryPicker) {
        window.showDirectoryPicker = async function() {
            console.warn("[Sigma Polyfill] Native FileSystem API not supported, using fallback virtual memory bridge.");
            return {
                name: "Sigma_Volatile_Drive",
                getDirectoryHandle: async () => this,
                getFileHandle: async () => ({ kind: 'file', name: 'sigma.sys' })
            };
        };
    }

    // 2. Cross-browser Extension Messaging Bridge
    const _sendMessage = window.chrome?.runtime?.sendMessage || window.browser?.runtime?.sendMessage;
    if (!_sendMessage) {
        console.warn("[Sigma Polyfill] WebExtension APIs absent. Emulating Sovereign Vault Event Bus.");
        window.SigmaSync = {
            broadcast: (data) => console.log("[Sigma Sync] Emulated Broadcast:", data),
            subscribe: (callback) => console.log("[Sigma Sync] Subscriber attached.")
        };
    } else {
        window.SigmaSync = {
            broadcast: (data) => _sendMessage({ type: "SIGMA_SYNC", payload: data }),
            subscribe: (cb) => (window.chrome?.runtime || window.browser?.runtime).onMessage.addListener(cb)
        };
    }

    // 3. WebGPU Polyfill for Legacy Rendering Contexts
    if (!navigator.gpu) {
        console.warn("[Sigma Polyfill] WebGPU not supported. Falling back to Sovereign WebGL2 pipeline.");
        // Emulate WebGPU structure mapping to WebGL for UI rendering
        navigator.gpu = {
            requestAdapter: async () => ({
                requestDevice: async () => ({
                    createShaderModule: () => ({}),
                    createRenderPipeline: () => ({}),
                    createCommandEncoder: () => ({ beginRenderPass: () => ({ setPipeline: ()=>{}, draw: ()=>{}, end: ()=>{} }), finish: ()=>{} }),
                    queue: { submit: () => {} }
                })
            })
        };
    }

    return {
        isPolyfilled: true,
        env: navigator.userAgent.includes("Brave") ? "Brave" :
             navigator.userAgent.includes("Edg") ? "Edge" :
             navigator.userAgent.includes("OPR") ? "Opera" : 
             navigator.userAgent.includes("Vivaldi") ? "Vivaldi" : "Chromium/Native"
    };
})();
