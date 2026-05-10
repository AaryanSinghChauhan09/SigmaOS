// File: zenith_desktop/modules/core/eventBus.js

const listeners = new Map();

export const on = (type, fn) => {
  if (!listeners.has(type)) listeners.set(type, []);
  listeners.get(type).push(fn);
};

export const off = (type, fn) => {
  const arr = listeners.get(type) || [];
  listeners.set(type, arr.filter(f => f !== fn));
};

export const emit = (type, payload) => {
  (listeners.get(type) || []).forEach(fn => {
    try { 
        fn(payload); 
    } catch (e) { 
        console.error('EventBus Error:', e); 
    }
  });
};
