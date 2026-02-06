// Service Worker for Magic Square Generator PWA
const CACHE_NAME = 'magic-square-v1.06';
const ASSETS_TO_CACHE = [
    './',
    './index.html',
    './style.css',
    './ui.js',
    './pkg/magic_squares.js',
    './pkg/magic_squares_bg.wasm',
    './icons/icon-192x192.png',
    './icons/icon-512x512.png',
    './manifest.json'
];

// Install event - cache all assets
self.addEventListener('install', (event) => {
    console.log('[Service Worker] Installing...');
    event.waitUntil(
        caches.open(CACHE_NAME)
            .then((cache) => {
                console.log('[Service Worker] Caching all assets');
                return cache.addAll(ASSETS_TO_CACHE);
            })
            .then(() => self.skipWaiting())
    );
});

// Activate event - clean up old caches
self.addEventListener('activate', (event) => {
    console.log('[Service Worker] Activating...');
    event.waitUntil(
        caches.keys().then((cacheNames) => {
            return Promise.all(
                cacheNames.map((cacheName) => {
                    if (cacheName !== CACHE_NAME) {
                        console.log('[Service Worker] Deleting old cache:', cacheName);
                        return caches.delete(cacheName);
                    }
                })
            );
        }).then(() => self.clients.claim())
    );
});

// Fetch event - serve from cache, fallback to network
self.addEventListener('fetch', (event) => {
    // We only want to call event.respondWith once
    event.respondWith(
        caches.match(event.request)
            .then((response) => {
                // Cache hit - return response from cache
                if (response) {
                    // console.log('[Service Worker] Serving from cache:', event.request.url);
                    return response;
                }

                // Make network request
                return fetch(event.request)
                    .then((networkResponse) => {
                        // Check if valid response
                        if (!networkResponse || networkResponse.status !== 200 || networkResponse.type !== 'basic') {
                            return networkResponse;
                        }

                        // Clone and cache
                        const responseToCache = networkResponse.clone();
                        caches.open(CACHE_NAME)
                            .then((cache) => {
                                cache.put(event.request, responseToCache);
                            });

                        return networkResponse;
                    })
                    .catch(() => {
                        // Network failure (Offline)
                        console.log('[Service Worker] Fetch failed; checking for offline fallback.');

                        // For navigation requests (loading the page), return the app shell (index.html)
                        if (event.request.mode === 'navigate') {
                            return caches.match('./index.html');
                        }

                        // You could also return placeholder images etc. here
                    });
            })
    );
});
