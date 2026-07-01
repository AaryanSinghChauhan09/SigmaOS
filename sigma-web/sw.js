// SPDX-License-Identifier: GPL-2.0-or-later
// sw.js — SigmaOS Service Worker
// Caches kernel WASM + assets for offline use.
// Inspired by: Workbox, PWA offline-first patterns

const CACHE_NAME = 'sigmaos-v1';
const PRECACHE = [
  '/',
  '/index.html',
  '/manifest.json',
  '/sigmaos.wasm',
  '/emulator.html',
];

self.addEventListener('install', e => {
  e.waitUntil(
    caches.open(CACHE_NAME).then(c => c.addAll(PRECACHE)).then(() => self.skipWaiting())
  );
});

self.addEventListener('activate', e => {
  e.waitUntil(
    caches.keys().then(keys =>
      Promise.all(keys.filter(k => k !== CACHE_NAME).map(k => caches.delete(k)))
    ).then(() => self.clients.claim())
  );
});

self.addEventListener('fetch', e => {
  // Cache-first for WASM (large, rarely changes); network-first for everything else
  if (e.request.url.endsWith('.wasm')) {
    e.respondWith(
      caches.match(e.request).then(r => r || fetch(e.request).then(resp => {
        const clone = resp.clone();
        caches.open(CACHE_NAME).then(c => c.put(e.request, clone));
        return resp;
      }))
    );
  } else {
    e.respondWith(
      fetch(e.request).catch(() => caches.match(e.request))
    );
  }
});
