/**
 * RustScript Chunk Loader (RSC-013)
 *
 * This module provides lazy loading functionality for code-split RustScript applications.
 * It handles:
 * - Loading chunks on-demand based on route navigation
 * - Preloading chunks based on hints
 * - Managing chunk dependencies
 * - Caching loaded chunks
 *
 * Architecture:
 *
 * +-------------------+     +-------------------+     +-------------------+
 * |   Route Change    | --> |   ChunkLoader     | --> |   WASM Instance   |
 * +-------------------+     +-------------------+     +-------------------+
 *                                    |
 *                                    v
 *                          +-------------------+
 *                          |  chunks.json      |
 *                          |  (manifest)       |
 *                          +-------------------+
 */

(function(global) {
    'use strict';

    /**
     * Chunk loading states
     */
    const ChunkState = {
        PENDING: 'pending',
        LOADING: 'loading',
        LOADED: 'loaded',
        ERROR: 'error'
    };

    /**
     * ChunkLoader manages loading and caching of code-split chunks.
     */
    class ChunkLoader {
        constructor() {
            /** Loaded chunk instances */
            this.loadedChunks = new Map();

            /** Chunk loading promises (for deduplication) */
            this.loadingPromises = new Map();

            /** Chunk manifest (loaded from chunks.json) */
            this.manifest = null;

            /** Base URL for chunk files */
            this.baseUrl = '';

            /** Preload link elements */
            this.preloadLinks = new Map();

            /** Route pattern to chunk mapping */
            this.routeChunks = new Map();

            /** Current route for tracking */
            this.currentRoute = null;

            /** Event listeners for chunk events */
            this.listeners = {
                chunkLoaded: [],
                chunkError: [],
                routeChange: []
            };
        }

        /**
         * Initialize the chunk loader with a manifest.
         * @param {string} manifestUrl - URL to the chunks.json manifest
         * @param {Object} options - Configuration options
         * @returns {Promise<void>}
         */
        async init(manifestUrl = 'chunks.json', options = {}) {
            this.baseUrl = options.baseUrl || '';

            try {
                const response = await fetch(this.baseUrl + manifestUrl);
                if (!response.ok) {
                    throw new Error(`Failed to load chunk manifest: ${response.status}`);
                }
                this.manifest = await response.json();

                // Build route-to-chunk mapping
                if (this.manifest.routes) {
                    for (const [pattern, chunkId] of Object.entries(this.manifest.routes)) {
                        this.routeChunks.set(pattern, chunkId);
                    }
                }

                // Preload hints if specified
                if (options.preload !== false) {
                    this.setupPreloadHints();
                }

                console.log('[ChunkLoader] Initialized with', this.manifest.entries?.length || 0, 'chunks');
            } catch (error) {
                console.error('[ChunkLoader] Failed to initialize:', error);
                throw error;
            }
        }

        /**
         * Load a chunk by name.
         * @param {string} chunkName - Name of the chunk to load
         * @returns {Promise<WebAssembly.Instance>}
         */
        async loadChunk(chunkName) {
            // Check if already loaded
            if (this.loadedChunks.has(chunkName)) {
                return this.loadedChunks.get(chunkName);
            }

            // Check if currently loading (deduplication)
            if (this.loadingPromises.has(chunkName)) {
                return this.loadingPromises.get(chunkName);
            }

            // Find chunk info in manifest
            const chunkInfo = this.findChunk(chunkName);
            if (!chunkInfo) {
                throw new Error(`Chunk not found: ${chunkName}`);
            }

            // Load dependencies first
            if (chunkInfo.dependencies && chunkInfo.dependencies.length > 0) {
                await Promise.all(
                    chunkInfo.dependencies.map(depId => {
                        const depChunk = this.manifest.entries.find(e => e.id === depId);
                        if (depChunk) {
                            return this.loadChunk(depChunk.name);
                        }
                        return Promise.resolve();
                    })
                );
            }

            // Start loading
            const loadPromise = this.fetchAndInstantiate(chunkInfo);
            this.loadingPromises.set(chunkName, loadPromise);

            try {
                const instance = await loadPromise;
                this.loadedChunks.set(chunkName, instance);
                this.loadingPromises.delete(chunkName);

                // Notify listeners
                this.emit('chunkLoaded', { chunk: chunkName, instance });

                return instance;
            } catch (error) {
                this.loadingPromises.delete(chunkName);
                this.emit('chunkError', { chunk: chunkName, error });
                throw error;
            }
        }

        /**
         * Load chunk for a specific route.
         * @param {string} route - Route pattern (e.g., "/users/:id")
         * @returns {Promise<WebAssembly.Instance|null>}
         */
        async loadRouteChunk(route) {
            // Match route pattern
            const chunkId = this.matchRoute(route);
            if (!chunkId) {
                // No specific chunk for this route, use main
                return this.loadChunk('main');
            }

            const chunkInfo = this.manifest.entries.find(e => e.id === chunkId);
            if (!chunkInfo) {
                console.warn('[ChunkLoader] No chunk found for route:', route);
                return null;
            }

            this.currentRoute = route;
            this.emit('routeChange', { route, chunk: chunkInfo.name });

            return this.loadChunk(chunkInfo.name);
        }

        /**
         * Preload a chunk without waiting for it.
         * @param {string} chunkName - Name of the chunk to preload
         */
        preloadChunk(chunkName) {
            // Don't preload if already loaded or loading
            if (this.loadedChunks.has(chunkName) || this.loadingPromises.has(chunkName)) {
                return;
            }

            const chunkInfo = this.findChunk(chunkName);
            if (!chunkInfo) {
                return;
            }

            // Use link preload if supported
            if (typeof document !== 'undefined' && !this.preloadLinks.has(chunkName)) {
                const link = document.createElement('link');
                link.rel = 'preload';
                link.href = this.baseUrl + chunkInfo.file_name;
                link.as = 'fetch';
                link.crossOrigin = 'anonymous';
                document.head.appendChild(link);
                this.preloadLinks.set(chunkName, link);
            }
        }

        /**
         * Preload chunks for likely route transitions.
         * @param {string} currentRoute - Current route
         */
        preloadLikelyRoutes(currentRoute) {
            if (!this.manifest || !this.manifest.entries) {
                return;
            }

            // Find route chunks that might be navigated to
            const routeChunks = this.manifest.entries.filter(e =>
                e.kind && e.kind.Route && e.name !== currentRoute
            );

            // Preload up to 2 nearby routes
            routeChunks.slice(0, 2).forEach(chunk => {
                this.preloadChunk(chunk.name);
            });
        }

        /**
         * Set up preload hints from the manifest.
         * @private
         */
        setupPreloadHints() {
            if (!this.manifest || typeof document === 'undefined') {
                return;
            }

            // Always preload common chunk if it exists
            if (this.manifest.common_chunk) {
                const commonChunk = this.manifest.entries.find(
                    e => e.id === this.manifest.common_chunk
                );
                if (commonChunk) {
                    this.preloadChunk(commonChunk.name);
                }
            }

            // Preload initial route chunks based on current URL
            if (typeof window !== 'undefined') {
                const path = window.location.pathname;
                const routeChunk = this.matchRoute(path);
                if (routeChunk) {
                    const chunkInfo = this.manifest.entries.find(e => e.id === routeChunk);
                    if (chunkInfo) {
                        this.preloadChunk(chunkInfo.name);
                    }
                }
            }
        }

        /**
         * Find a chunk by name in the manifest.
         * @param {string} chunkName - Chunk name
         * @returns {Object|null}
         * @private
         */
        findChunk(chunkName) {
            if (!this.manifest || !this.manifest.entries) {
                return null;
            }
            return this.manifest.entries.find(e => e.name === chunkName);
        }

        /**
         * Match a route path to a chunk ID.
         * @param {string} path - URL path
         * @returns {number|null} - Chunk ID or null
         * @private
         */
        matchRoute(path) {
            if (!this.manifest || !this.manifest.routes) {
                return null;
            }

            // Direct match
            if (this.manifest.routes[path]) {
                return this.manifest.routes[path];
            }

            // Pattern matching (simple implementation)
            for (const [pattern, chunkId] of Object.entries(this.manifest.routes)) {
                const regex = this.patternToRegex(pattern);
                if (regex.test(path)) {
                    return chunkId;
                }
            }

            return null;
        }

        /**
         * Convert a route pattern to a regex.
         * @param {string} pattern - Route pattern (e.g., "/users/:id")
         * @returns {RegExp}
         * @private
         */
        patternToRegex(pattern) {
            const escaped = pattern
                .replace(/[.+?^${}()|[\]\\]/g, '\\$&')
                .replace(/:[a-zA-Z_][a-zA-Z0-9_]*/g, '[^/]+')
                .replace(/\*/g, '.*');
            return new RegExp(`^${escaped}$`);
        }

        /**
         * Fetch and instantiate a WASM chunk.
         * @param {Object} chunkInfo - Chunk metadata
         * @returns {Promise<WebAssembly.Instance>}
         * @private
         */
        async fetchAndInstantiate(chunkInfo) {
            const url = this.baseUrl + chunkInfo.file_name;

            const response = await fetch(url);
            if (!response.ok) {
                throw new Error(`Failed to fetch chunk ${chunkInfo.name}: ${response.status}`);
            }

            const wasmBytes = await response.arrayBuffer();

            // Get imports from the main RustScript runtime
            const imports = global.RustScript ? global.RustScript.createImports() : {};

            const { instance } = await WebAssembly.instantiate(wasmBytes, imports);

            // Initialize chunk if it has an init export
            if (instance.exports.init) {
                instance.exports.init();
            }

            return instance;
        }

        /**
         * Add an event listener.
         * @param {string} event - Event name
         * @param {Function} callback - Event handler
         */
        on(event, callback) {
            if (this.listeners[event]) {
                this.listeners[event].push(callback);
            }
        }

        /**
         * Remove an event listener.
         * @param {string} event - Event name
         * @param {Function} callback - Event handler
         */
        off(event, callback) {
            if (this.listeners[event]) {
                const index = this.listeners[event].indexOf(callback);
                if (index !== -1) {
                    this.listeners[event].splice(index, 1);
                }
            }
        }

        /**
         * Emit an event.
         * @param {string} event - Event name
         * @param {Object} data - Event data
         * @private
         */
        emit(event, data) {
            if (this.listeners[event]) {
                for (const callback of this.listeners[event]) {
                    try {
                        callback(data);
                    } catch (error) {
                        console.error(`[ChunkLoader] Error in ${event} listener:`, error);
                    }
                }
            }
        }

        /**
         * Get loading statistics.
         * @returns {Object}
         */
        getStats() {
            return {
                loadedChunks: this.loadedChunks.size,
                pendingLoads: this.loadingPromises.size,
                totalChunks: this.manifest?.entries?.length || 0,
                preloadedLinks: this.preloadLinks.size
            };
        }

        /**
         * Clear all loaded chunks (useful for hot reload).
         */
        clear() {
            this.loadedChunks.clear();
            this.loadingPromises.clear();

            // Remove preload links
            for (const link of this.preloadLinks.values()) {
                if (link.parentNode) {
                    link.parentNode.removeChild(link);
                }
            }
            this.preloadLinks.clear();
        }
    }

    // Create singleton instance
    const chunkLoader = new ChunkLoader();

    /**
     * Generate preload link tags for HTML.
     * @param {Array<string>} chunks - Chunk filenames to preload
     * @returns {string} - HTML link tags
     */
    function generatePreloadLinks(chunks) {
        return chunks.map(chunk =>
            `<link rel="preload" href="${chunk}" as="fetch" crossorigin="anonymous">`
        ).join('\n');
    }

    /**
     * Load an app with code splitting support.
     * @param {string} wasmSource - Main WASM file path
     * @param {string} rootId - Root element ID
     * @param {Object} options - Options including manifestUrl
     * @returns {Promise<Object>} - App instance
     */
    async function loadAppWithChunks(wasmSource, rootId = 'app', options = {}) {
        // Initialize chunk loader
        const manifestUrl = options.manifestUrl || 'chunks.json';
        await chunkLoader.init(manifestUrl, options);

        // Load main chunk
        const mainChunk = await chunkLoader.loadChunk('main');

        // Set up route change listener if router is available
        if (typeof window !== 'undefined') {
            window.addEventListener('popstate', () => {
                const path = window.location.pathname;
                chunkLoader.loadRouteChunk(path).catch(console.error);
            });
        }

        // Preload likely next routes
        if (typeof window !== 'undefined') {
            chunkLoader.preloadLikelyRoutes(window.location.pathname);
        }

        return {
            mainChunk,
            chunkLoader,
            loadRoute: (route) => chunkLoader.loadRouteChunk(route),
            preload: (chunk) => chunkLoader.preloadChunk(chunk),
            stats: () => chunkLoader.getStats()
        };
    }

    // Export
    const ChunkLoaderAPI = {
        ChunkLoader,
        chunkLoader,
        loadAppWithChunks,
        generatePreloadLinks
    };

    if (typeof module !== 'undefined' && module.exports) {
        module.exports = ChunkLoaderAPI;
    }

    // Attach to RustScript if available
    if (global.RustScript) {
        global.RustScript.ChunkLoader = ChunkLoaderAPI;
        global.RustScript.loadAppWithChunks = loadAppWithChunks;
    } else {
        global.RustScriptChunkLoader = ChunkLoaderAPI;
    }

})(typeof globalThis !== 'undefined' ? globalThis : typeof window !== 'undefined' ? window : this);
