const CACHE_NAME = 'sigma-os-v1';
const ASSETS = [
    './index.html',
    './index.css',
    './manifest.json',
    './kernel.js',
    './ui.js',
    './themes.js',
    './apps.js',
    './games.js',
    './telemetry_shield.js',
    './assistant.js'
];

self.addEventListener('install', event => {
    event.waitUntil(
        caches.open(CACHE_NAME).then(cache => {
            console.log('Opened cache');
            return cache.addAll(ASSETS);
        })
    );
});

self.addEventListener('fetch', event => {
    event.respondWith(
        caches.match(event.request).then(response => {
            if (response) {
                return response; // Use cached response
            }
            return fetch(event.request); // Fetch from network if not cached
        }).catch(() => {
            // Fallback for failed network requests (Offline mode)
            if (event.request.headers.get('accept').includes('text/html')) {
                return caches.match('./index.html');
            }
        })
    );
});

self.addEventListener('activate', event => {
    const cacheAllowlist = [CACHE_NAME];
    event.waitUntil(
        caches.keys().then(cacheNames => {
            return Promise.all(
                cacheNames.map(cacheName => {
                    if (!cacheAllowlist.includes(cacheName)) {
                        return caches.delete(cacheName);
                    }
                })
            );
        })
    );
});
