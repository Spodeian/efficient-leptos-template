// Service worker: makes the site work offline.
//
// - Pages: network first, so a new release shows on the first visit after deploying;
//   the cached copy is used only when the network fails.
// - /pkg/ files (content-hashed, so a given URL never changes) and /fonts/ (served as
//   immutable): cache first, with no network request once cached.
// - Everything else: network first, cached copy as fallback.
//
// The web build (`ssg`) fills in the two placeholders below. In the desktop build they
// stay as they are, which simply means nothing is precached.
const BUILD_ID = '__BUILD_ID__';
const PRECACHE_URLS = /*__PRECACHE_URLS__*/ [];

const CACHE_NAME = `leptos-template-${BUILD_ID}`;

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      // `cache: 'reload'` bypasses the HTTP cache so a new build never precaches stale files.
      .then((cache) => cache.addAll(PRECACHE_URLS.map((url) => new Request(url, { cache: 'reload' }))))
      .then(() => self.skipWaiting())
  );
});

self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys()
      .then((keys) => Promise.all(keys.filter((key) => key !== CACHE_NAME).map((key) => caches.delete(key))))
      .then(() => self.clients.claim())
  );
});

// `waitUntil` keeps the worker alive until the cache write lands, even after the response
// has been returned.
async function networkFirst(event, fallbackUrl) {
  const { request } = event;
  const cache = await caches.open(CACHE_NAME);
  try {
    const response = await fetch(request);
    if (response.ok) {
      event.waitUntil(cache.put(request, response.clone()));
    }
    return response;
  } catch (error) {
    const cached = (await cache.match(request, { ignoreSearch: true }))
      || (fallbackUrl && (await cache.match(fallbackUrl)));
    if (cached) {
      return cached;
    }
    throw error;
  }
}

async function cacheFirst(event) {
  const { request } = event;
  const cache = await caches.open(CACHE_NAME);
  const cached = await cache.match(request);
  if (cached) {
    return cached;
  }
  const response = await fetch(request);
  if (response.ok) {
    event.waitUntil(cache.put(request, response.clone()));
  }
  return response;
}

self.addEventListener('fetch', (event) => {
  const { request } = event;
  const url = new URL(request.url);
  if (request.method !== 'GET' || url.origin !== self.location.origin || url.pathname === '/sw.js') {
    return; // let the browser handle it
  }
  if (request.mode === 'navigate') {
    // Offline and never visited: fall back to the home page, which can route client-side.
    event.respondWith(networkFirst(event, '/'));
  } else if (url.pathname.startsWith('/pkg/') || url.pathname.startsWith('/fonts/')) {
    event.respondWith(cacheFirst(event));
  } else {
    event.respondWith(networkFirst(event));
  }
});
