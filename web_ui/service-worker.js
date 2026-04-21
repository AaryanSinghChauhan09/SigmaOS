/**
 * SigmaOS Sovereign Service Worker
 * Principle: Offline Resilience & Shard Caching
 */

const CACHE_NAME = 'sovereign-v1';
const ASSETS = [
    'index.html',
    'styles/style.css',
    'styles/modules/compatibility.css',
    'scripts/modules/00_sovereign_framework.js'
];

self.addEventListener('install', (event) => {
    event.waitUntil(
        caches.open(CACHE_NAME).then((cache) => cache.addAll(ASSETS))
    );
});

self.addEventListener('fetch', (event) => {
    event.respondWith(
        caches.match(event.request).then((response) => {
            return response || fetch(event.request);
        })
    );
});
