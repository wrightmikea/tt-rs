// Service Worker to bypass GitHub Pages caching
// Forces network-first strategy for all requests

const CACHE_VERSION = 'v1';

self.addEventListener('install', (event) => {
  // Skip waiting to activate immediately
  self.skipWaiting();
});

self.addEventListener('activate', (event) => {
  // Claim all clients immediately
  event.waitUntil(clients.claim());
});

self.addEventListener('fetch', (event) => {
  // Network-first strategy: always try network, never use cache
  event.respondWith(
    fetch(event.request, { cache: 'no-store' })
      .catch(() => {
        // If network fails, try cache as fallback (offline support)
        return caches.match(event.request);
      })
  );
});
