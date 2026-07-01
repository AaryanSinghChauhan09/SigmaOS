/**
 * sigma_api_service.js
 * SigmaOS Kernel API Service — bridges web UI to sigma-bus IPC
 */

export const SigmaApiVersion = '15.0.0';

/**
 * Post a command to the sigma-bus IPC layer.
 * @param {string} endpoint  - e.g. 'kernel.sysinfo', 'fs.read'
 * @param {object} payload
 * @returns {Promise<object>}
 */
export async function sigmaCall(endpoint, payload = {}) {
  if (typeof window.__sigma_ipc__ !== 'undefined') {
    return window.__sigma_ipc__.call(endpoint, payload);
  }
  // Fallback: simulate response in browser preview / QEMU framebuffer
  console.warn(`[SigmaAPI] sigma_ipc not available, stubbing ${endpoint}`);
  return { ok: false, stub: true, endpoint };
}

/**
 * Register a listener on a sigma-bus event channel.
 * @param {string} channel
 * @param {function} handler
 * @returns {function} unsubscribe
 */
export function sigmaOn(channel, handler) {
  if (typeof window.__sigma_ipc__ !== 'undefined') {
    return window.__sigma_ipc__.on(channel, handler);
  }
  return () => {};
}

// Expose as global for non-module scripts that haven't been migrated yet
window.SigmaApiService = { sigmaCall, sigmaOn, version: SigmaApiVersion };
