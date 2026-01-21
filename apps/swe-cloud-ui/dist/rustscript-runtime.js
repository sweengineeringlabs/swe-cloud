/**
 * RustScript Runtime Polyfill
 *
 * This polyfill bridges the WebAssembly Component Model interfaces defined in
 * rsc-wit to native browser APIs. It enables RustScript applications to run
 * in browsers that don't yet fully support the Component Model.
 *
 * Architecture:
 *
 * ┌─────────────────────────────────────────────────────────────────┐
 * │                    WASM Module (RustScript App)                  │
 * └─────────────────────────────────────────────────────────────────┘
 *                              ▼ imports
 * ┌─────────────────────────────────────────────────────────────────┐
 * │                    RustScript Polyfill                           │
 * ├─────────────────────────────────────────────────────────────────┤
 * │  dom.js      │ events.js    │ http.js      │ storage.js        │
 * │  console.js  │ timers.js    │ core.js      │                   │
 * └─────────────────────────────────────────────────────────────────┘
 *                              ▼ calls
 * ┌─────────────────────────────────────────────────────────────────┐
 * │                    Browser Native APIs                           │
 * └─────────────────────────────────────────────────────────────────┘
 */

(function(global) {
    'use strict';

    // =========================================================================
    // Core Infrastructure
    // =========================================================================

    /**
     * Resource handle manager for tracking WASM resources.
     * Maps handle IDs to JavaScript objects (DOM elements, event handlers, etc.)
     */
    class HandleManager {
        constructor() {
            this.handles = new Map();
            this.nextId = 1;
        }

        /**
         * Allocate a handle for an object.
         * @param {any} obj - The object to track
         * @returns {number} - The handle ID
         */
        alloc(obj) {
            const id = this.nextId++;
            this.handles.set(id, obj);
            return id;
        }

        /**
         * Get an object by handle.
         * @param {number} id - The handle ID
         * @returns {any} - The object, or undefined if not found
         */
        get(id) {
            return this.handles.get(id);
        }

        /**
         * Free a handle.
         * @param {number} id - The handle ID
         * @returns {boolean} - Whether the handle existed
         */
        free(id) {
            return this.handles.delete(id);
        }

        /**
         * Check if a handle exists.
         * @param {number} id - The handle ID
         * @returns {boolean}
         */
        has(id) {
            return this.handles.has(id);
        }
    }

    // Global handle managers
    const elementHandles = new HandleManager();
    const eventHandlerHandles = new HandleManager();
    const timerHandles = new HandleManager();

    // WASM instance reference (set during initialization)
    let wasmInstance = null;
    let wasmMemory = null;

    // =========================================================================
    // Memory Helpers
    // =========================================================================

    /**
     * Read a string from WASM memory.
     * @param {number} ptr - Pointer to string data
     * @param {number} len - String length in bytes
     * @returns {string}
     */
    function readString(ptr, len) {
        const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
        return new TextDecoder().decode(bytes);
    }

    /**
     * Decode a string handle and read the string from WASM memory.
     * String handles encode (offset << 16) | len in a single i32.
     * @param {number} handle - The string handle
     * @returns {string}
     */
    function decodeStringHandle(handle) {
        const ptr = (handle >>> 16) & 0xFFFF;
        const len = handle & 0xFFFF;
        return readString(ptr, len);
    }

    /**
     * Write a string to WASM memory.
     * @param {string} str - The string to write
     * @param {number} ptr - Pointer to write location
     * @param {number} maxLen - Maximum bytes to write
     * @returns {number} - Bytes written
     */
    function writeString(str, ptr, maxLen) {
        const bytes = new TextEncoder().encode(str);
        const len = Math.min(bytes.length, maxLen);
        const view = new Uint8Array(wasmMemory.buffer, ptr, len);
        view.set(bytes.subarray(0, len));
        return len;
    }

    // =========================================================================
    // DOM Interface Implementation
    // =========================================================================

    const dom = {
        /**
         * Get element by ID.
         * @param {number} idPtr - Pointer to ID string
         * @param {number} idLen - Length of ID string
         * @returns {number} - Element handle or 0 if not found
         */
        getElementById(idPtr, idLen) {
            const id = readString(idPtr, idLen);
            const element = document.getElementById(id);
            if (element) {
                return elementHandles.alloc(element);
            }
            return 0;
        },

        /**
         * Query selector.
         * @param {number} selectorPtr - Pointer to selector string
         * @param {number} selectorLen - Length of selector string
         * @returns {number} - Element handle or 0 if not found
         */
        querySelector(selectorPtr, selectorLen) {
            const selector = readString(selectorPtr, selectorLen);
            const element = document.querySelector(selector);
            if (element) {
                return elementHandles.alloc(element);
            }
            return 0;
        },

        /**
         * Create an element.
         * @param {number} tagHandle - String handle for tag name (encodes offset << 16 | len)
         * @returns {number} - Element handle
         */
        createElement(tagHandle) {
            const tag = decodeStringHandle(tagHandle);
            const element = document.createElement(tag);
            return elementHandles.alloc(element);
        },

        /**
         * Create an element with a namespace URI (for SVG, MathML, etc.).
         * @param {number} nsPtr - Pointer to namespace URI
         * @param {number} nsLen - Length of namespace URI
         * @param {number} tagPtr - Pointer to tag name
         * @param {number} tagLen - Length of tag name
         * @returns {number} - Element handle
         */
        createElementNS(nsPtr, nsLen, tagPtr, tagLen) {
            const namespaceURI = readString(nsPtr, nsLen);
            const tag = readString(tagPtr, tagLen);
            const element = document.createElementNS(namespaceURI, tag);
            return elementHandles.alloc(element);
        },

        /**
         * Create a text node.
         * @param {number} textHandle - String handle for text content (encodes offset << 16 | len)
         * @returns {number} - Element handle
         */
        createTextNode(textHandle) {
            const text = decodeStringHandle(textHandle);
            const node = document.createTextNode(text);
            return elementHandles.alloc(node);
        },

        /**
         * Set element attribute.
         * @param {number} handle - Element handle
         * @param {number} nameHandle - String handle for attribute name
         * @param {number} valueHandle - String handle for attribute value
         */
        setAttribute(handle, nameHandle, valueHandle) {
            const element = elementHandles.get(handle);
            if (element && element.setAttribute) {
                const name = decodeStringHandle(nameHandle);
                const value = decodeStringHandle(valueHandle);
                element.setAttribute(name, value);
            }
        },

        /**
         * Get element attribute.
         * @param {number} handle - Element handle
         * @param {number} namePtr - Pointer to attribute name
         * @param {number} nameLen - Length of attribute name
         * @param {number} outPtr - Pointer to output buffer
         * @param {number} outLen - Max output length
         * @returns {number} - Length written or -1 if no attribute
         */
        getAttribute(handle, namePtr, nameLen, outPtr, outLen) {
            const element = elementHandles.get(handle);
            if (element && element.getAttribute) {
                const name = readString(namePtr, nameLen);
                const value = element.getAttribute(name);
                if (value !== null) {
                    return writeString(value, outPtr, outLen);
                }
            }
            return -1;
        },

        /**
         * Remove element attribute.
         * @param {number} handle - Element handle
         * @param {number} namePtr - Pointer to attribute name
         * @param {number} nameLen - Length of attribute name
         */
        removeAttribute(handle, namePtr, nameLen) {
            const element = elementHandles.get(handle);
            if (element && element.removeAttribute) {
                const name = readString(namePtr, nameLen);
                element.removeAttribute(name);
            }
        },

        /**
         * Set text content.
         * @param {number} handle - Element handle
         * @param {number} textHandle - String handle for text content
         */
        setTextContent(handle, textHandle) {
            const element = elementHandles.get(handle);
            if (element) {
                element.textContent = decodeStringHandle(textHandle);
            }
        },

        /**
         * Get text content.
         * @param {number} handle - Element handle
         * @param {number} outPtr - Pointer to output buffer
         * @param {number} outLen - Max output length
         * @returns {number} - Length written
         */
        getTextContent(handle, outPtr, outLen) {
            const element = elementHandles.get(handle);
            if (element) {
                const text = element.textContent || '';
                return writeString(text, outPtr, outLen);
            }
            return 0;
        },

        /**
         * Set inner HTML.
         * @param {number} handle - Element handle
         * @param {number} htmlPtr - Pointer to HTML string
         * @param {number} htmlLen - Length of HTML string
         */
        setInnerHTML(handle, htmlPtr, htmlLen) {
            const element = elementHandles.get(handle);
            if (element) {
                element.innerHTML = readString(htmlPtr, htmlLen);
            }
        },

        /**
         * Append child element.
         * @param {number} parentHandle - Parent element handle
         * @param {number} childHandle - Child element handle
         */
        appendChild(parentHandle, childHandle) {
            const parent = elementHandles.get(parentHandle);
            const child = elementHandles.get(childHandle);
            if (parent && child) {
                parent.appendChild(child);
            }
        },

        /**
         * Insert before element.
         * @param {number} parentHandle - Parent element handle
         * @param {number} newHandle - New element handle
         * @param {number} refHandle - Reference element handle (0 to append)
         */
        insertBefore(parentHandle, newHandle, refHandle) {
            const parent = elementHandles.get(parentHandle);
            const newNode = elementHandles.get(newHandle);
            const refNode = refHandle ? elementHandles.get(refHandle) : null;
            if (parent && newNode) {
                parent.insertBefore(newNode, refNode);
            }
        },

        /**
         * Remove child element.
         * @param {number} parentHandle - Parent element handle
         * @param {number} childHandle - Child element handle
         */
        removeChild(parentHandle, childHandle) {
            const parent = elementHandles.get(parentHandle);
            const child = elementHandles.get(childHandle);
            if (parent && child && child.parentNode === parent) {
                parent.removeChild(child);
            }
        },

        /**
         * Remove element from DOM.
         * @param {number} handle - Element handle
         */
        remove(handle) {
            const element = elementHandles.get(handle);
            if (element && element.remove) {
                element.remove();
            }
        },

        /**
         * Clone element.
         * @param {number} handle - Element handle
         * @param {number} deep - Whether to deep clone (1) or shallow (0)
         * @returns {number} - New element handle
         */
        cloneNode(handle, deep) {
            const element = elementHandles.get(handle);
            if (element) {
                const clone = element.cloneNode(!!deep);
                return elementHandles.alloc(clone);
            }
            return 0;
        },

        /**
         * Get parent element.
         * @param {number} handle - Element handle
         * @returns {number} - Parent element handle or 0
         */
        getParent(handle) {
            const element = elementHandles.get(handle);
            if (element && element.parentElement) {
                return elementHandles.alloc(element.parentElement);
            }
            return 0;
        },

        /**
         * Get first child.
         * @param {number} handle - Element handle
         * @returns {number} - Child element handle or 0
         */
        getFirstChild(handle) {
            const element = elementHandles.get(handle);
            if (element && element.firstChild) {
                return elementHandles.alloc(element.firstChild);
            }
            return 0;
        },

        /**
         * Get next sibling.
         * @param {number} handle - Element handle
         * @returns {number} - Sibling element handle or 0
         */
        getNextSibling(handle) {
            const element = elementHandles.get(handle);
            if (element && element.nextSibling) {
                return elementHandles.alloc(element.nextSibling);
            }
            return 0;
        },

        /**
         * Add CSS class.
         * @param {number} handle - Element handle
         * @param {number} classPtr - Pointer to class name
         * @param {number} classLen - Length of class name
         */
        addClass(handle, classPtr, classLen) {
            const element = elementHandles.get(handle);
            if (element && element.classList) {
                element.classList.add(readString(classPtr, classLen));
            }
        },

        /**
         * Remove CSS class.
         * @param {number} handle - Element handle
         * @param {number} classPtr - Pointer to class name
         * @param {number} classLen - Length of class name
         */
        removeClass(handle, classPtr, classLen) {
            const element = elementHandles.get(handle);
            if (element && element.classList) {
                element.classList.remove(readString(classPtr, classLen));
            }
        },

        /**
         * Toggle CSS class.
         * @param {number} handle - Element handle
         * @param {number} classPtr - Pointer to class name
         * @param {number} classLen - Length of class name
         * @returns {number} - 1 if class is now present, 0 if removed
         */
        toggleClass(handle, classPtr, classLen) {
            const element = elementHandles.get(handle);
            if (element && element.classList) {
                return element.classList.toggle(readString(classPtr, classLen)) ? 1 : 0;
            }
            return 0;
        },

        /**
         * Check if has CSS class.
         * @param {number} handle - Element handle
         * @param {number} classPtr - Pointer to class name
         * @param {number} classLen - Length of class name
         * @returns {number} - 1 if has class, 0 if not
         */
        hasClass(handle, classPtr, classLen) {
            const element = elementHandles.get(handle);
            if (element && element.classList) {
                return element.classList.contains(readString(classPtr, classLen)) ? 1 : 0;
            }
            return 0;
        },

        /**
         * Set style property.
         * @param {number} handle - Element handle
         * @param {number} propPtr - Pointer to property name
         * @param {number} propLen - Length of property name
         * @param {number} valuePtr - Pointer to value
         * @param {number} valueLen - Length of value
         */
        setStyle(handle, propPtr, propLen, valuePtr, valueLen) {
            const element = elementHandles.get(handle);
            if (element && element.style) {
                const prop = readString(propPtr, propLen);
                const value = readString(valuePtr, valueLen);
                element.style.setProperty(prop, value);
            }
        },

        /**
         * Get bounding client rect.
         * @param {number} handle - Element handle
         * @param {number} outPtr - Pointer to output array (8 f64s)
         */
        getBoundingClientRect(handle, outPtr) {
            const element = elementHandles.get(handle);
            if (element && element.getBoundingClientRect) {
                const rect = element.getBoundingClientRect();
                const view = new Float64Array(wasmMemory.buffer, outPtr, 8);
                view[0] = rect.x;
                view[1] = rect.y;
                view[2] = rect.width;
                view[3] = rect.height;
                view[4] = rect.top;
                view[5] = rect.right;
                view[6] = rect.bottom;
                view[7] = rect.left;
            }
        },

        /**
         * Focus element.
         * @param {number} handle - Element handle
         */
        focus(handle) {
            const element = elementHandles.get(handle);
            if (element && element.focus) {
                element.focus();
            }
        },

        /**
         * Blur element.
         * @param {number} handle - Element handle
         */
        blur(handle) {
            const element = elementHandles.get(handle);
            if (element && element.blur) {
                element.blur();
            }
        },

        /**
         * Get input value.
         * @param {number} handle - Element handle
         * @param {number} outPtr - Output buffer pointer
         * @param {number} outLen - Max output length
         * @returns {number} - Bytes written
         */
        getValue(handle, outPtr, outLen) {
            const element = elementHandles.get(handle);
            if (element && 'value' in element) {
                return writeString(element.value, outPtr, outLen);
            }
            return 0;
        },

        /**
         * Set input value.
         * @param {number} handle - Element handle
         * @param {number} valuePtr - Pointer to value
         * @param {number} valueLen - Length of value
         */
        setValue(handle, valuePtr, valueLen) {
            const element = elementHandles.get(handle);
            if (element && 'value' in element) {
                element.value = readString(valuePtr, valueLen);
            }
        },

        /**
         * Get checked state.
         * @param {number} handle - Element handle
         * @returns {number} - 1 if checked, 0 if not
         */
        getChecked(handle) {
            const element = elementHandles.get(handle);
            if (element && 'checked' in element) {
                return element.checked ? 1 : 0;
            }
            return 0;
        },

        /**
         * Set checked state.
         * @param {number} handle - Element handle
         * @param {number} checked - 1 for checked, 0 for unchecked
         */
        setChecked(handle, checked) {
            const element = elementHandles.get(handle);
            if (element && 'checked' in element) {
                element.checked = !!checked;
            }
        },

        /**
         * Free an element handle.
         * @param {number} handle - Element handle
         */
        freeElement(handle) {
            elementHandles.free(handle);
        },

        /**
         * Get document body.
         * @returns {number} - Element handle for body
         */
        getBody() {
            return elementHandles.alloc(document.body);
        },

        /**
         * Get document head.
         * @returns {number} - Element handle for head
         */
        getHead() {
            return elementHandles.alloc(document.head);
        },

        /**
         * Get document element.
         * @returns {number} - Element handle for documentElement
         */
        getDocumentElement() {
            return elementHandles.alloc(document.documentElement);
        }
    };

    // =========================================================================
    // Events Interface Implementation
    // =========================================================================

    // Active event tracking for prevent/stop operations
    let activeEventId = 0;
    const activeEvents = new Map();

    const events = {
        /**
         * Create an event handler.
         * @param {number} callbackId - WASM callback ID
         * @returns {number} - Handler handle
         */
        createHandler(callbackId) {
            const handler = { callbackId };
            return eventHandlerHandles.alloc(handler);
        },

        /**
         * RS-146: Simplified addEventListener for codegen.
         * Takes a callback ID directly without needing separate handler registration.
         * @param {number} targetHandle - Element handle
         * @param {number} eventTypeHandle - String handle for event type
         * @param {number} callbackId - WASM function table index for the event handler
         */
        addEventListenerDirect(targetHandle, eventTypeHandle, callbackId) {
            const target = elementHandles.get(targetHandle);
            if (!target) return;

            const eventType = decodeStringHandle(eventTypeHandle);

            const listener = (event) => {
                // Track this event for prevent/stop operations
                const eventId = ++activeEventId;
                activeEvents.set(eventId, event);

                try {
                    // Package event data and call WASM via indirect call
                    const eventData = packageEvent(event, eventType);
                    if (wasmInstance && wasmInstance.exports.__rsc_handle_event) {
                        wasmInstance.exports.__rsc_handle_event(callbackId, eventId, eventData);
                    }
                } finally {
                    activeEvents.delete(eventId);
                }
            };

            // Register with default options (no capture, not once, not passive)
            target.addEventListener(eventType, listener, {
                capture: false,
                once: false,
                passive: false
            });

            // Store listener for potential cleanup
            // Create a handler record so it can be removed later if needed
            const handler = { callbackId, listener, eventType, target };
            return eventHandlerHandles.alloc(handler);
        },

        /**
         * Add event listener.
         * @param {number} targetHandle - Element handle
         * @param {number} eventTypePtr - Pointer to event type string
         * @param {number} eventTypeLen - Length of event type
         * @param {number} handlerHandle - Handler handle
         * @param {number} capture - Use capture phase
         * @param {number} once - Remove after first call
         * @param {number} passive - Passive listener
         */
        addEventListener(targetHandle, eventTypePtr, eventTypeLen, handlerHandle, capture, once, passive) {
            const target = elementHandles.get(targetHandle);
            const handler = eventHandlerHandles.get(handlerHandle);
            if (!target || !handler) return;

            const eventType = readString(eventTypePtr, eventTypeLen);
            const callbackId = handler.callbackId;

            const listener = (event) => {
                // Track this event for prevent/stop operations
                const eventId = ++activeEventId;
                activeEvents.set(eventId, event);

                try {
                    // Package event data and call WASM
                    const eventData = packageEvent(event, eventType);
                    if (wasmInstance && wasmInstance.exports.__rsc_handle_event) {
                        wasmInstance.exports.__rsc_handle_event(callbackId, eventId, eventData);
                    }
                } finally {
                    activeEvents.delete(eventId);
                }
            };

            // Store listener reference for removal
            handler.listener = listener;
            handler.eventType = eventType;
            handler.target = target;

            target.addEventListener(eventType, listener, {
                capture: !!capture,
                once: !!once,
                passive: !!passive
            });
        },

        /**
         * Remove event listener.
         * @param {number} targetHandle - Element handle
         * @param {number} eventTypePtr - Pointer to event type string
         * @param {number} eventTypeLen - Length of event type
         * @param {number} handlerHandle - Handler handle
         */
        removeEventListener(targetHandle, eventTypePtr, eventTypeLen, handlerHandle) {
            const target = elementHandles.get(targetHandle);
            const handler = eventHandlerHandles.get(handlerHandle);
            if (!target || !handler || !handler.listener) return;

            const eventType = readString(eventTypePtr, eventTypeLen);
            target.removeEventListener(eventType, handler.listener);
            handler.listener = null;
        },

        /**
         * Prevent default action.
         * @param {number} eventId - Event ID from callback
         */
        preventDefault(eventId) {
            const event = activeEvents.get(eventId);
            if (event && event.cancelable) {
                event.preventDefault();
            }
        },

        /**
         * Stop event propagation.
         * @param {number} eventId - Event ID from callback
         */
        stopPropagation(eventId) {
            const event = activeEvents.get(eventId);
            if (event) {
                event.stopPropagation();
            }
        },

        /**
         * Stop immediate propagation.
         * @param {number} eventId - Event ID from callback
         */
        stopImmediatePropagation(eventId) {
            const event = activeEvents.get(eventId);
            if (event) {
                event.stopImmediatePropagation();
            }
        },

        /**
         * Free a handler.
         * @param {number} handle - Handler handle
         */
        freeHandler(handle) {
            const handler = eventHandlerHandles.get(handle);
            if (handler && handler.listener && handler.target) {
                handler.target.removeEventListener(handler.eventType, handler.listener);
            }
            eventHandlerHandles.free(handle);
        }
    };

    /**
     * Package a native event into WASM-compatible data.
     */
    function packageEvent(event, eventType) {
        const base = {
            type: eventType,
            timestamp: event.timeStamp,
            bubbles: event.bubbles,
            cancelable: event.cancelable,
            phase: event.eventPhase,
            isTrusted: event.isTrusted
        };

        if (event instanceof MouseEvent) {
            return {
                ...base,
                kind: 'mouse',
                clientX: event.clientX,
                clientY: event.clientY,
                pageX: event.pageX,
                pageY: event.pageY,
                screenX: event.screenX,
                screenY: event.screenY,
                offsetX: event.offsetX,
                offsetY: event.offsetY,
                button: event.button,
                buttons: event.buttons,
                altKey: event.altKey,
                ctrlKey: event.ctrlKey,
                metaKey: event.metaKey,
                shiftKey: event.shiftKey
            };
        }

        if (event instanceof KeyboardEvent) {
            return {
                ...base,
                kind: 'keyboard',
                key: event.key,
                code: event.code,
                keyCode: event.keyCode,
                repeat: event.repeat,
                altKey: event.altKey,
                ctrlKey: event.ctrlKey,
                metaKey: event.metaKey,
                shiftKey: event.shiftKey,
                location: event.location
            };
        }

        if (event instanceof FocusEvent) {
            return {
                ...base,
                kind: 'focus'
            };
        }

        if (event instanceof InputEvent) {
            return {
                ...base,
                kind: 'input',
                data: event.data,
                inputType: event.inputType,
                isComposing: event.isComposing
            };
        }

        return { ...base, kind: 'generic' };
    }

    // =========================================================================
    // Console Interface Implementation
    // =========================================================================

    const consoleApi = {
        /**
         * Log to console.
         * @param {number} msgPtr - Pointer to message
         * @param {number} msgLen - Length of message
         */
        log(msgPtr, msgLen) {
            console.log(readString(msgPtr, msgLen));
        },

        /**
         * Log warning to console.
         * @param {number} msgPtr - Pointer to message
         * @param {number} msgLen - Length of message
         */
        warn(msgPtr, msgLen) {
            console.warn(readString(msgPtr, msgLen));
        },

        /**
         * Log error to console.
         * @param {number} msgPtr - Pointer to message
         * @param {number} msgLen - Length of message
         */
        error(msgPtr, msgLen) {
            console.error(readString(msgPtr, msgLen));
        },

        /**
         * Log debug to console.
         * @param {number} msgPtr - Pointer to message
         * @param {number} msgLen - Length of message
         */
        debug(msgPtr, msgLen) {
            console.debug(readString(msgPtr, msgLen));
        },

        /**
         * Log info to console.
         * @param {number} msgPtr - Pointer to message
         * @param {number} msgLen - Length of message
         */
        info(msgPtr, msgLen) {
            console.info(readString(msgPtr, msgLen));
        },

        /**
         * Start console group.
         * @param {number} labelPtr - Pointer to label
         * @param {number} labelLen - Length of label
         */
        group(labelPtr, labelLen) {
            console.group(readString(labelPtr, labelLen));
        },

        /**
         * Start collapsed console group.
         * @param {number} labelPtr - Pointer to label
         * @param {number} labelLen - Length of label
         */
        groupCollapsed(labelPtr, labelLen) {
            console.groupCollapsed(readString(labelPtr, labelLen));
        },

        /**
         * End console group.
         */
        groupEnd() {
            console.groupEnd();
        },

        /**
         * Start timer.
         * @param {number} labelPtr - Pointer to label
         * @param {number} labelLen - Length of label
         */
        time(labelPtr, labelLen) {
            console.time(readString(labelPtr, labelLen));
        },

        /**
         * End timer.
         * @param {number} labelPtr - Pointer to label
         * @param {number} labelLen - Length of label
         */
        timeEnd(labelPtr, labelLen) {
            console.timeEnd(readString(labelPtr, labelLen));
        },

        /**
         * Log timer without ending.
         * @param {number} labelPtr - Pointer to label
         * @param {number} labelLen - Length of label
         */
        timeLog(labelPtr, labelLen) {
            console.timeLog(readString(labelPtr, labelLen));
        },

        /**
         * Clear console.
         */
        clear() {
            console.clear();
        }
    };

    // =========================================================================
    // Timers Interface Implementation
    // =========================================================================

    const timers = {
        /**
         * Set timeout.
         * @param {number} callbackId - WASM callback ID
         * @param {number} delay - Delay in milliseconds
         * @returns {number} - Timer handle
         */
        setTimeout(callbackId, delay) {
            const nativeId = setTimeout(() => {
                if (wasmInstance && wasmInstance.exports.__rsc_handle_timer) {
                    wasmInstance.exports.__rsc_handle_timer(callbackId);
                }
            }, delay);
            return timerHandles.alloc({ nativeId, type: 'timeout' });
        },

        /**
         * Clear timeout.
         * @param {number} handle - Timer handle
         */
        clearTimeout(handle) {
            const timer = timerHandles.get(handle);
            if (timer && timer.type === 'timeout') {
                clearTimeout(timer.nativeId);
                timerHandles.free(handle);
            }
        },

        /**
         * Set interval.
         * @param {number} callbackId - WASM callback ID
         * @param {number} interval - Interval in milliseconds
         * @returns {number} - Timer handle
         */
        setInterval(callbackId, interval) {
            const nativeId = setInterval(() => {
                if (wasmInstance && wasmInstance.exports.__rsc_handle_timer) {
                    wasmInstance.exports.__rsc_handle_timer(callbackId);
                }
            }, interval);
            return timerHandles.alloc({ nativeId, type: 'interval' });
        },

        /**
         * Clear interval.
         * @param {number} handle - Timer handle
         */
        clearInterval(handle) {
            const timer = timerHandles.get(handle);
            if (timer && timer.type === 'interval') {
                clearInterval(timer.nativeId);
                timerHandles.free(handle);
            }
        },

        /**
         * Request animation frame.
         * @param {number} callbackId - WASM callback ID
         * @returns {number} - Frame handle
         */
        requestAnimationFrame(callbackId) {
            const nativeId = requestAnimationFrame((timestamp) => {
                if (wasmInstance && wasmInstance.exports.__rsc_handle_animation_frame) {
                    wasmInstance.exports.__rsc_handle_animation_frame(callbackId, timestamp);
                }
            });
            return timerHandles.alloc({ nativeId, type: 'raf' });
        },

        /**
         * Cancel animation frame.
         * @param {number} handle - Frame handle
         */
        cancelAnimationFrame(handle) {
            const timer = timerHandles.get(handle);
            if (timer && timer.type === 'raf') {
                cancelAnimationFrame(timer.nativeId);
                timerHandles.free(handle);
            }
        },

        /**
         * Get current timestamp.
         * @returns {number} - Current timestamp in milliseconds
         */
        now() {
            return performance.now();
        }
    };

    // =========================================================================
    // Storage Interface Implementation
    // =========================================================================

    const storage = {
        /**
         * Get item from localStorage.
         * @param {number} keyPtr - Pointer to key
         * @param {number} keyLen - Length of key
         * @param {number} outPtr - Output buffer pointer
         * @param {number} outLen - Max output length
         * @returns {number} - Bytes written or -1 if not found
         */
        localStorageGet(keyPtr, keyLen, outPtr, outLen) {
            const key = readString(keyPtr, keyLen);
            const value = localStorage.getItem(key);
            if (value !== null) {
                return writeString(value, outPtr, outLen);
            }
            return -1;
        },

        /**
         * Set item in localStorage.
         * @param {number} keyPtr - Pointer to key
         * @param {number} keyLen - Length of key
         * @param {number} valuePtr - Pointer to value
         * @param {number} valueLen - Length of value
         * @returns {number} - 1 on success, 0 on failure
         */
        localStorageSet(keyPtr, keyLen, valuePtr, valueLen) {
            try {
                const key = readString(keyPtr, keyLen);
                const value = readString(valuePtr, valueLen);
                localStorage.setItem(key, value);
                return 1;
            } catch (e) {
                return 0;
            }
        },

        /**
         * Remove item from localStorage.
         * @param {number} keyPtr - Pointer to key
         * @param {number} keyLen - Length of key
         */
        localStorageRemove(keyPtr, keyLen) {
            const key = readString(keyPtr, keyLen);
            localStorage.removeItem(key);
        },

        /**
         * Clear localStorage.
         */
        localStorageClear() {
            localStorage.clear();
        },

        /**
         * Get the number of items in localStorage.
         * @returns {number} - Number of items
         */
        localStorageLength() {
            return localStorage.length;
        },

        /**
         * Get the key at a specific index in localStorage.
         * @param {number} index - Key index
         * @param {number} outPtr - Output buffer pointer
         * @param {number} outLen - Max output length
         * @returns {number} - Bytes written or -1 if index out of bounds
         */
        localStorageKey(index, outPtr, outLen) {
            const key = localStorage.key(index);
            if (key !== null) {
                return writeString(key, outPtr, outLen);
            }
            return -1;
        },

        /**
         * Get all keys in localStorage as JSON array.
         * @param {number} outPtr - Output buffer pointer
         * @param {number} outLen - Max output length
         * @returns {number} - Bytes written
         */
        localStorageKeys(outPtr, outLen) {
            const keys = [];
            for (let i = 0; i < localStorage.length; i++) {
                keys.push(localStorage.key(i));
            }
            const json = JSON.stringify(keys);
            return writeString(json, outPtr, outLen);
        },

        /**
         * Get item from sessionStorage.
         * @param {number} keyPtr - Pointer to key
         * @param {number} keyLen - Length of key
         * @param {number} outPtr - Output buffer pointer
         * @param {number} outLen - Max output length
         * @returns {number} - Bytes written or -1 if not found
         */
        sessionStorageGet(keyPtr, keyLen, outPtr, outLen) {
            const key = readString(keyPtr, keyLen);
            const value = sessionStorage.getItem(key);
            if (value !== null) {
                return writeString(value, outPtr, outLen);
            }
            return -1;
        },

        /**
         * Set item in sessionStorage.
         * @param {number} keyPtr - Pointer to key
         * @param {number} keyLen - Length of key
         * @param {number} valuePtr - Pointer to value
         * @param {number} valueLen - Length of value
         * @returns {number} - 1 on success, 0 on failure
         */
        sessionStorageSet(keyPtr, keyLen, valuePtr, valueLen) {
            try {
                const key = readString(keyPtr, keyLen);
                const value = readString(valuePtr, valueLen);
                sessionStorage.setItem(key, value);
                return 1;
            } catch (e) {
                return 0;
            }
        },

        /**
         * Remove item from sessionStorage.
         * @param {number} keyPtr - Pointer to key
         * @param {number} keyLen - Length of key
         */
        sessionStorageRemove(keyPtr, keyLen) {
            const key = readString(keyPtr, keyLen);
            sessionStorage.removeItem(key);
        },

        /**
         * Clear sessionStorage.
         */
        sessionStorageClear() {
            sessionStorage.clear();
        },

        /**
         * Get the number of items in sessionStorage.
         * @returns {number} - Number of items
         */
        sessionStorageLength() {
            return sessionStorage.length;
        },

        /**
         * Get the key at a specific index in sessionStorage.
         * @param {number} index - Key index
         * @param {number} outPtr - Output buffer pointer
         * @param {number} outLen - Max output length
         * @returns {number} - Bytes written or -1 if index out of bounds
         */
        sessionStorageKey(index, outPtr, outLen) {
            const key = sessionStorage.key(index);
            if (key !== null) {
                return writeString(key, outPtr, outLen);
            }
            return -1;
        },

        /**
         * Get all keys in sessionStorage as JSON array.
         * @param {number} outPtr - Output buffer pointer
         * @param {number} outLen - Max output length
         * @returns {number} - Bytes written
         */
        sessionStorageKeys(outPtr, outLen) {
            const keys = [];
            for (let i = 0; i < sessionStorage.length; i++) {
                keys.push(sessionStorage.key(i));
            }
            const json = JSON.stringify(keys);
            return writeString(json, outPtr, outLen);
        }
    };

    // =========================================================================
    // HTTP Interface Implementation
    // =========================================================================

    // Pending fetch operations and request builders
    const pendingFetches = new Map();
    const requestBuilders = new HandleManager();
    let nextFetchId = 1;

    // HTTP method enum mapping
    const HTTP_METHODS = ['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'HEAD', 'OPTIONS'];

    // Request mode mapping
    const REQUEST_MODES = ['cors', 'no-cors', 'same-origin'];

    // Credentials mode mapping
    const CREDENTIALS_MODES = ['omit', 'same-origin', 'include'];

    // Cache mode mapping
    const CACHE_MODES = ['default', 'no-store', 'reload', 'no-cache', 'force-cache', 'only-if-cached'];

    // Redirect mode mapping
    const REDIRECT_MODES = ['follow', 'error', 'manual'];

    // Referrer policy mapping
    const REFERRER_POLICIES = [
        'no-referrer',
        'no-referrer-when-downgrade',
        'same-origin',
        'origin',
        'strict-origin',
        'origin-when-cross-origin',
        'strict-origin-when-cross-origin',
        'unsafe-url'
    ];

    // Response type mapping
    const RESPONSE_TYPES = ['basic', 'cors', 'default', 'error', 'opaque', 'opaqueredirect'];

    // Fetch error kinds
    const FETCH_ERROR_KINDS = {
        NETWORK: 0,
        TIMEOUT: 1,
        ABORTED: 2,
        CORS: 3,
        INVALID_REQUEST: 4,
        INVALID_URL: 5
    };

    /**
     * Convert a Response object to our response format.
     * @param {Response} response - Native fetch Response
     * @returns {Promise<Object>} - Response data
     */
    async function convertResponse(response) {
        const bodyBytes = await response.arrayBuffer();
        const headers = [];
        response.headers.forEach((value, name) => {
            headers.push({ name, value });
        });

        // Map response type
        let responseType = 2; // default
        const rtIndex = RESPONSE_TYPES.indexOf(response.type);
        if (rtIndex !== -1) {
            responseType = rtIndex;
        }

        return {
            status: response.status,
            statusText: response.statusText,
            headers: headers,
            body: new Uint8Array(bodyBytes),
            url: response.url,
            redirected: response.redirected,
            responseType: responseType
        };
    }

    /**
     * Create a fetch error object.
     * @param {number} kind - Error kind enum value
     * @param {string} message - Error message
     * @returns {Object} - Fetch error
     */
    function createFetchError(kind, message) {
        return { kind, message };
    }

    /**
     * Perform a fetch request with full options.
     * @param {Object} request - Request configuration
     * @returns {Promise<Object>} - Response or error
     */
    async function performFetch(request) {
        try {
            // Build fetch options
            const options = {
                method: HTTP_METHODS[request.method] || 'GET',
                headers: new Headers(),
                mode: REQUEST_MODES[request.mode] || 'cors',
                credentials: CREDENTIALS_MODES[request.credentials] || 'same-origin',
                cache: CACHE_MODES[request.cache] || 'default',
                redirect: REDIRECT_MODES[request.redirect] || 'follow',
                referrerPolicy: REFERRER_POLICIES[request.referrerPolicy] || 'strict-origin-when-cross-origin'
            };

            // Add headers
            if (request.headers && Array.isArray(request.headers)) {
                request.headers.forEach(h => {
                    options.headers.append(h.name, h.value);
                });
            }

            // Add body if present
            if (request.body && request.body.length > 0) {
                options.body = new Uint8Array(request.body);
            }

            // Create abort controller for timeout
            let abortController = null;
            let timeoutId = null;
            if (request.timeoutMs > 0) {
                abortController = new AbortController();
                options.signal = abortController.signal;
                timeoutId = setTimeout(() => {
                    abortController.abort();
                }, request.timeoutMs);
            }

            try {
                const response = await fetch(request.url, options);
                if (timeoutId) clearTimeout(timeoutId);
                return { ok: true, value: await convertResponse(response) };
            } catch (error) {
                if (timeoutId) clearTimeout(timeoutId);

                // Determine error type
                let errorKind = FETCH_ERROR_KINDS.NETWORK;
                if (error.name === 'AbortError') {
                    errorKind = FETCH_ERROR_KINDS.TIMEOUT;
                } else if (error.message && error.message.includes('CORS')) {
                    errorKind = FETCH_ERROR_KINDS.CORS;
                }

                return { ok: false, error: createFetchError(errorKind, error.message || 'Fetch failed') };
            }
        } catch (error) {
            return { ok: false, error: createFetchError(FETCH_ERROR_KINDS.INVALID_REQUEST, error.message) };
        }
    }

    const http = {
        /**
         * Make an HTTP request with full configuration.
         * This is a synchronous wrapper that queues the async operation.
         * @param {Object} request - Request configuration from WASM
         * @returns {Object} - Result with response or error
         */
        async fetch(request) {
            return performFetch(request);
        },

        /**
         * Make a simple GET request.
         * @param {string} url - The URL to fetch
         * @returns {Object} - Result with response or error
         */
        async get(url) {
            return performFetch({
                url: url,
                method: 0, // GET
                headers: [],
                body: null,
                mode: 0,
                credentials: 1,
                cache: 0,
                redirect: 0,
                referrerPolicy: 6,
                timeoutMs: 0
            });
        },

        /**
         * Make a POST request with a body.
         * @param {string} url - The URL to fetch
         * @param {Uint8Array} body - Request body
         * @param {string} contentType - Content-Type header
         * @returns {Object} - Result with response or error
         */
        async post(url, body, contentType) {
            return performFetch({
                url: url,
                method: 1, // POST
                headers: [{ name: 'Content-Type', value: contentType }],
                body: Array.from(body),
                mode: 0,
                credentials: 1,
                cache: 0,
                redirect: 0,
                referrerPolicy: 6,
                timeoutMs: 0
            });
        },

        /**
         * Make a POST request with JSON body.
         * @param {string} url - The URL to fetch
         * @param {string} json - JSON string body
         * @returns {Object} - Result with response or error
         */
        async postJson(url, json) {
            return performFetch({
                url: url,
                method: 1, // POST
                headers: [{ name: 'Content-Type', value: 'application/json' }],
                body: Array.from(new TextEncoder().encode(json)),
                mode: 0,
                credentials: 1,
                cache: 0,
                redirect: 0,
                referrerPolicy: 6,
                timeoutMs: 0
            });
        },

        // === Request Builder Resource ===

        /**
         * Create a new request builder.
         * @param {string} url - The URL for the request
         * @returns {number} - Builder handle
         */
        'request-builder#constructor': function(url) {
            const builder = {
                url: url,
                method: 0, // GET
                headers: [],
                body: null,
                mode: 0,
                credentials: 1,
                cache: 0,
                redirect: 0,
                referrerPolicy: 6,
                timeoutMs: 0
            };
            return requestBuilders.alloc(builder);
        },

        /**
         * Set the HTTP method on a request builder.
         * @param {number} handle - Builder handle
         * @param {number} method - Method enum value
         * @returns {number} - Builder handle (for chaining)
         */
        'request-builder#method': function(handle, method) {
            const builder = requestBuilders.get(handle);
            if (builder) {
                builder.method = method;
            }
            return handle;
        },

        /**
         * Add a header to a request builder.
         * @param {number} handle - Builder handle
         * @param {string} name - Header name
         * @param {string} value - Header value
         * @returns {number} - Builder handle (for chaining)
         */
        'request-builder#header': function(handle, name, value) {
            const builder = requestBuilders.get(handle);
            if (builder) {
                builder.headers.push({ name, value });
            }
            return handle;
        },

        /**
         * Set the body on a request builder.
         * @param {number} handle - Builder handle
         * @param {Uint8Array} body - Body bytes
         * @returns {number} - Builder handle (for chaining)
         */
        'request-builder#body': function(handle, body) {
            const builder = requestBuilders.get(handle);
            if (builder) {
                builder.body = Array.from(body);
            }
            return handle;
        },

        /**
         * Set JSON body on a request builder.
         * @param {number} handle - Builder handle
         * @param {string} json - JSON string
         * @returns {number} - Builder handle (for chaining)
         */
        'request-builder#json': function(handle, json) {
            const builder = requestBuilders.get(handle);
            if (builder) {
                builder.headers.push({ name: 'Content-Type', value: 'application/json' });
                builder.body = Array.from(new TextEncoder().encode(json));
            }
            return handle;
        },

        /**
         * Set form data body on a request builder.
         * @param {number} handle - Builder handle
         * @param {Array<[string, string]>} data - Form data key-value pairs
         * @returns {number} - Builder handle (for chaining)
         */
        'request-builder#form': function(handle, data) {
            const builder = requestBuilders.get(handle);
            if (builder) {
                builder.headers.push({ name: 'Content-Type', value: 'application/x-www-form-urlencoded' });
                const formBody = data
                    .map(([k, v]) => `${encodeURIComponent(k)}=${encodeURIComponent(v)}`)
                    .join('&');
                builder.body = Array.from(new TextEncoder().encode(formBody));
            }
            return handle;
        },

        /**
         * Set the request mode on a request builder.
         * @param {number} handle - Builder handle
         * @param {number} mode - Mode enum value
         * @returns {number} - Builder handle (for chaining)
         */
        'request-builder#mode': function(handle, mode) {
            const builder = requestBuilders.get(handle);
            if (builder) {
                builder.mode = mode;
            }
            return handle;
        },

        /**
         * Set the credentials mode on a request builder.
         * @param {number} handle - Builder handle
         * @param {number} credentials - Credentials enum value
         * @returns {number} - Builder handle (for chaining)
         */
        'request-builder#credentials': function(handle, credentials) {
            const builder = requestBuilders.get(handle);
            if (builder) {
                builder.credentials = credentials;
            }
            return handle;
        },

        /**
         * Set the timeout on a request builder.
         * @param {number} handle - Builder handle
         * @param {number} ms - Timeout in milliseconds
         * @returns {number} - Builder handle (for chaining)
         */
        'request-builder#timeout': function(handle, ms) {
            const builder = requestBuilders.get(handle);
            if (builder) {
                builder.timeoutMs = ms;
            }
            return handle;
        },

        /**
         * Send the request from a builder.
         * @param {number} handle - Builder handle
         * @returns {Promise<Object>} - Result with response or error
         */
        'request-builder#send': async function(handle) {
            const builder = requestBuilders.get(handle);
            if (!builder) {
                return { ok: false, error: createFetchError(FETCH_ERROR_KINDS.INVALID_REQUEST, 'Invalid builder handle') };
            }
            const result = await performFetch(builder);
            requestBuilders.free(handle);
            return result;
        },

        /**
         * Free a request builder handle.
         * @param {number} handle - Builder handle
         */
        'request-builder#drop': function(handle) {
            requestBuilders.free(handle);
        },

        // === Response Helpers ===

        /**
         * Parse response body as UTF-8 text.
         * @param {Object} response - Response object
         * @returns {Object} - Result with text or error
         */
        responseText(response) {
            try {
                const text = new TextDecoder().decode(new Uint8Array(response.body));
                return { ok: true, value: text };
            } catch (error) {
                return { ok: false, error: 'Failed to decode response as UTF-8' };
            }
        },

        /**
         * Get response body as JSON string.
         * @param {Object} response - Response object
         * @returns {Object} - Result with JSON string or error
         */
        responseJson(response) {
            try {
                const text = new TextDecoder().decode(new Uint8Array(response.body));
                // Validate it's valid JSON by parsing
                JSON.parse(text);
                return { ok: true, value: text };
            } catch (error) {
                return { ok: false, error: 'Invalid JSON in response body' };
            }
        },

        // === Legacy low-level interface (for backward compatibility) ===

        /**
         * Start a fetch request (low-level, callback-based).
         * @param {number} urlPtr - Pointer to URL
         * @param {number} urlLen - Length of URL
         * @param {number} methodPtr - Pointer to method
         * @param {number} methodLen - Length of method
         * @param {number} bodyPtr - Pointer to body (0 if none)
         * @param {number} bodyLen - Length of body
         * @returns {number} - Fetch ID
         */
        fetchLowLevel(urlPtr, urlLen, methodPtr, methodLen, bodyPtr, bodyLen) {
            const url = readString(urlPtr, urlLen);
            const method = readString(methodPtr, methodLen);
            const body = bodyPtr ? readString(bodyPtr, bodyLen) : null;

            const fetchId = nextFetchId++;

            fetch(url, {
                method,
                body: body || undefined,
            })
            .then(async (response) => {
                const responseData = await convertResponse(response);
                if (wasmInstance && wasmInstance.exports.__rsc_handle_fetch_response) {
                    wasmInstance.exports.__rsc_handle_fetch_response(
                        fetchId,
                        responseData.status,
                        response.ok ? 1 : 0,
                        JSON.stringify(responseData)
                    );
                }
            })
            .catch((error) => {
                if (wasmInstance && wasmInstance.exports.__rsc_handle_fetch_error) {
                    wasmInstance.exports.__rsc_handle_fetch_error(fetchId, error.message);
                }
            });

            return fetchId;
        }
    };

    // =========================================================================
    // Router Interface Implementation
    // =========================================================================

    // Popstate and hashchange callback tracking
    const popstateCallbacks = new Map();
    const hashchangeCallbacks = new Map();

    const router = {
        /**
         * Get the current URL pathname.
         * @param {number} outPtr - Output buffer pointer
         * @param {number} outLen - Max output length
         * @returns {number} - Bytes written
         */
        getPathname(outPtr, outLen) {
            return writeString(location.pathname, outPtr, outLen);
        },

        /**
         * Get the current URL search/query string.
         * @param {number} outPtr - Output buffer pointer
         * @param {number} outLen - Max output length
         * @returns {number} - Bytes written
         */
        getSearch(outPtr, outLen) {
            return writeString(location.search, outPtr, outLen);
        },

        /**
         * Get the current URL hash.
         * @param {number} outPtr - Output buffer pointer
         * @param {number} outLen - Max output length
         * @returns {number} - Bytes written
         */
        getHash(outPtr, outLen) {
            return writeString(location.hash, outPtr, outLen);
        },

        /**
         * Get the current complete URL.
         * @param {number} outPtr - Output buffer pointer
         * @param {number} outLen - Max output length
         * @returns {number} - Bytes written
         */
        getHref(outPtr, outLen) {
            return writeString(location.href, outPtr, outLen);
        },

        /**
         * Get the current origin.
         * @param {number} outPtr - Output buffer pointer
         * @param {number} outLen - Max output length
         * @returns {number} - Bytes written
         */
        getOrigin(outPtr, outLen) {
            return writeString(location.origin, outPtr, outLen);
        },

        /**
         * Push a new entry onto the browser history stack.
         * @param {number} urlPtr - Pointer to URL string
         * @param {number} urlLen - Length of URL string
         * @param {number} statePtr - Pointer to state string (0 if none)
         * @param {number} stateLen - Length of state string
         */
        pushState(urlPtr, urlLen, statePtr, stateLen) {
            const url = readString(urlPtr, urlLen);
            let state = null;
            if (statePtr && stateLen > 0) {
                try {
                    state = JSON.parse(readString(statePtr, stateLen));
                } catch (e) {
                    // Invalid JSON, keep state as null
                }
            }
            history.pushState(state, '', url);
        },

        /**
         * Replace the current history entry.
         * @param {number} urlPtr - Pointer to URL string
         * @param {number} urlLen - Length of URL string
         * @param {number} statePtr - Pointer to state string (0 if none)
         * @param {number} stateLen - Length of state string
         */
        replaceState(urlPtr, urlLen, statePtr, stateLen) {
            const url = readString(urlPtr, urlLen);
            let state = null;
            if (statePtr && stateLen > 0) {
                try {
                    state = JSON.parse(readString(statePtr, stateLen));
                } catch (e) {
                    // Invalid JSON, keep state as null
                }
            }
            history.replaceState(state, '', url);
        },

        /**
         * Navigate back in history.
         */
        goBack() {
            history.back();
        },

        /**
         * Navigate forward in history.
         */
        goForward() {
            history.forward();
        },

        /**
         * Navigate to a specific point in history.
         * @param {number} delta - Number of entries to move
         */
        go(delta) {
            history.go(delta);
        },

        /**
         * Get the current history state.
         * @param {number} outPtr - Output buffer pointer
         * @param {number} outLen - Max output length
         * @returns {number} - Bytes written or -1 if no state
         */
        getState(outPtr, outLen) {
            if (history.state !== null) {
                const stateJson = JSON.stringify(history.state);
                return writeString(stateJson, outPtr, outLen);
            }
            return -1;
        },

        /**
         * Get the current history length.
         * @returns {number} - History length
         */
        historyLength() {
            return history.length;
        },

        /**
         * Set the URL hash without adding a history entry.
         * @param {number} hashPtr - Pointer to hash string
         * @param {number} hashLen - Length of hash string
         */
        setHash(hashPtr, hashLen) {
            let hash = readString(hashPtr, hashLen);
            // Ensure hash starts with #
            if (!hash.startsWith('#')) {
                hash = '#' + hash;
            }
            location.hash = hash;
        },

        /**
         * Subscribe to popstate events.
         * @param {number} callbackId - WASM callback ID
         */
        onPopstate(callbackId) {
            if (popstateCallbacks.has(callbackId)) {
                return; // Already registered
            }

            const handler = (event) => {
                // Package navigation event data
                const navEvent = {
                    pathname: location.pathname,
                    search: location.search,
                    hash: location.hash,
                    state: event.state ? JSON.stringify(event.state) : null
                };

                if (wasmInstance && wasmInstance.exports.__rsc_handle_popstate) {
                    wasmInstance.exports.__rsc_handle_popstate(callbackId, JSON.stringify(navEvent));
                }
            };

            window.addEventListener('popstate', handler);
            popstateCallbacks.set(callbackId, handler);
        },

        /**
         * Subscribe to hashchange events.
         * @param {number} callbackId - WASM callback ID
         */
        onHashchange(callbackId) {
            if (hashchangeCallbacks.has(callbackId)) {
                return; // Already registered
            }

            const handler = (event) => {
                // Package navigation event data
                const navEvent = {
                    pathname: location.pathname,
                    search: location.search,
                    hash: location.hash,
                    state: history.state ? JSON.stringify(history.state) : null,
                    oldURL: event.oldURL,
                    newURL: event.newURL
                };

                if (wasmInstance && wasmInstance.exports.__rsc_handle_hashchange) {
                    wasmInstance.exports.__rsc_handle_hashchange(callbackId, JSON.stringify(navEvent));
                }
            };

            window.addEventListener('hashchange', handler);
            hashchangeCallbacks.set(callbackId, handler);
        },

        /**
         * Unsubscribe from popstate events.
         * @param {number} callbackId - WASM callback ID
         */
        offPopstate(callbackId) {
            const handler = popstateCallbacks.get(callbackId);
            if (handler) {
                window.removeEventListener('popstate', handler);
                popstateCallbacks.delete(callbackId);
            }
        },

        /**
         * Unsubscribe from hashchange events.
         * @param {number} callbackId - WASM callback ID
         */
        offHashchange(callbackId) {
            const handler = hashchangeCallbacks.get(callbackId);
            if (handler) {
                window.removeEventListener('hashchange', handler);
                hashchangeCallbacks.delete(callbackId);
            }
        }
    };

    // =========================================================================
    // Component Instance Management (RS-150)
    // =========================================================================

    /**
     * Component instance manager.
     * Tracks component instances with their state, props, and lifecycle.
     */
    const componentManager = {
        /** Map of componentId -> component instance data */
        instances: new Map(),

        /** Next available component ID */
        nextId: 1,

        /**
         * Create a new component instance.
         * @param {Function} renderFn - The component's render function
         * @param {Object} props - Initial props for the component
         * @param {Element|number} parentElement - Parent DOM element or handle
         * @returns {number} - Component ID
         */
        create(renderFn, props, parentElement) {
            const componentId = this.nextId++;

            // Resolve parent element from handle if needed
            const parent = typeof parentElement === 'number'
                ? elementHandles.get(parentElement)
                : parentElement;

            const instance = {
                id: componentId,
                renderFn: renderFn,
                props: props || {},
                element: null,          // Root DOM element
                signals: new Map(),     // Signal name -> { value, subscribers }
                effects: new Map(),     // Effect ID -> { cleanup, deps }
                children: new Map(),    // Child component IDs
                parent: parent,
                parentId: null,         // Parent component ID (if nested)
                mounted: false,
                error: null
            };

            this.instances.set(componentId, instance);

            // Render the component within error boundary
            try {
                errorBoundary.push(componentId, null);
                const element = errorBoundary.wrapRender(() => renderFn(props));
                errorBoundary.pop();

                if (element) {
                    instance.element = typeof element === 'number'
                        ? elementHandles.get(element)
                        : element;

                    // Append to parent if available
                    if (parent && instance.element) {
                        parent.appendChild(instance.element);
                    }
                }

                instance.mounted = true;
            } catch (error) {
                instance.error = error;
                errorBoundary.handleError(error);
            }

            return componentId;
        },

        /**
         * Update a component with new props.
         * @param {number} componentId - Component ID
         * @param {Object} newProps - New props to merge
         * @returns {boolean} - Whether update succeeded
         */
        update(componentId, newProps) {
            const instance = this.instances.get(componentId);
            if (!instance) {
                console.warn(`Component ${componentId} not found`);
                return false;
            }

            // Merge props
            const prevProps = instance.props;
            instance.props = { ...prevProps, ...newProps };

            // Re-render within error boundary
            try {
                errorBoundary.push(componentId, null);
                const newElement = errorBoundary.wrapRender(() =>
                    instance.renderFn(instance.props)
                );
                errorBoundary.pop();

                if (newElement) {
                    const resolvedElement = typeof newElement === 'number'
                        ? elementHandles.get(newElement)
                        : newElement;

                    // Replace old element with new one
                    if (instance.element && instance.element.parentNode) {
                        instance.element.parentNode.replaceChild(
                            resolvedElement,
                            instance.element
                        );
                    }
                    instance.element = resolvedElement;
                }

                instance.error = null;
                return true;
            } catch (error) {
                instance.error = error;
                errorBoundary.handleError(error);
                return false;
            }
        },

        /**
         * Unmount and cleanup a component.
         * @param {number} componentId - Component ID
         * @returns {boolean} - Whether unmount succeeded
         */
        unmount(componentId) {
            const instance = this.instances.get(componentId);
            if (!instance) {
                return false;
            }

            // Recursively unmount children first
            for (const childId of instance.children.keys()) {
                this.unmount(childId);
            }

            // Run effect cleanups
            for (const [effectId, effect] of instance.effects) {
                if (effect.cleanup && typeof effect.cleanup === 'function') {
                    try {
                        effect.cleanup();
                    } catch (e) {
                        console.error(`Error in effect cleanup for component ${componentId}:`, e);
                    }
                }
            }
            instance.effects.clear();

            // Clear signals
            for (const [signalName, signal] of instance.signals) {
                signal.subscribers.clear();
            }
            instance.signals.clear();

            // Remove from DOM
            if (instance.element && instance.element.parentNode) {
                instance.element.parentNode.removeChild(instance.element);
            }

            // Remove from parent's children
            if (instance.parentId) {
                const parentInstance = this.instances.get(instance.parentId);
                if (parentInstance) {
                    parentInstance.children.delete(componentId);
                }
            }

            instance.mounted = false;
            this.instances.delete(componentId);

            return true;
        },

        /**
         * Get a component instance by ID.
         * @param {number} componentId - Component ID
         * @returns {Object|null} - Component instance or null
         */
        get(componentId) {
            return this.instances.get(componentId) || null;
        },

        /**
         * Register a child component.
         * @param {number} parentId - Parent component ID
         * @param {number} childId - Child component ID
         */
        addChild(parentId, childId) {
            const parent = this.instances.get(parentId);
            const child = this.instances.get(childId);
            if (parent && child) {
                parent.children.set(childId, child);
                child.parentId = parentId;
            }
        },

        /**
         * Create a signal for a component.
         * @param {number} componentId - Component ID
         * @param {string} name - Signal name
         * @param {any} initialValue - Initial signal value
         * @returns {Object} - Signal accessor { get, set, subscribe }
         */
        createSignal(componentId, name, initialValue) {
            const instance = this.instances.get(componentId);
            if (!instance) {
                throw new Error(`Component ${componentId} not found`);
            }

            const signal = {
                value: initialValue,
                subscribers: new Set()
            };

            instance.signals.set(name, signal);

            return {
                get: () => signal.value,
                set: (newValue) => {
                    const oldValue = signal.value;
                    signal.value = newValue;
                    // Notify subscribers
                    for (const subscriber of signal.subscribers) {
                        try {
                            subscriber(newValue, oldValue);
                        } catch (e) {
                            console.error('Signal subscriber error:', e);
                        }
                    }
                },
                subscribe: (fn) => {
                    signal.subscribers.add(fn);
                    return () => signal.subscribers.delete(fn);
                }
            };
        },

        /**
         * Register an effect for a component.
         * @param {number} componentId - Component ID
         * @param {Function} effectFn - Effect function (returns cleanup)
         * @param {Array} deps - Dependency array
         * @returns {number} - Effect ID
         */
        registerEffect(componentId, effectFn, deps) {
            const instance = this.instances.get(componentId);
            if (!instance) {
                throw new Error(`Component ${componentId} not found`);
            }

            const effectId = lifecycleManager.nextId++;

            const effect = {
                fn: effectFn,
                deps: deps ? [...deps] : null,
                cleanup: null
            };

            instance.effects.set(effectId, effect);

            // Schedule effect execution
            queueMicrotask(() => {
                try {
                    const cleanup = effectFn();
                    if (typeof cleanup === 'function') {
                        effect.cleanup = cleanup;
                    }
                } catch (e) {
                    console.error(`Effect error in component ${componentId}:`, e);
                }
            });

            return effectId;
        },

        /**
         * Get all mounted component IDs.
         * @returns {number[]} - Array of component IDs
         */
        getAllIds() {
            return Array.from(this.instances.keys());
        },

        /**
         * Get component count.
         * @returns {number} - Number of active components
         */
        count() {
            return this.instances.size;
        }
    };

    // =========================================================================
    // Error Boundaries (RS-154)
    // =========================================================================

    /**
     * Error boundary manager.
     * Handles errors during component rendering with fallback support.
     */
    const errorBoundary = {
        /** Stack of active error boundaries */
        boundaryStack: [],

        /** Map of componentId -> error state */
        errorStates: new Map(),

        /** Global error handler callback */
        globalErrorHandler: null,

        /**
         * Push an error boundary onto the stack.
         * @param {number} componentId - Component ID that defines this boundary
         * @param {Function|null} fallbackFn - Fallback render function
         */
        push(componentId, fallbackFn) {
            this.boundaryStack.push({
                componentId: componentId,
                fallback: fallbackFn,
                hasError: false,
                error: null
            });
        },

        /**
         * Pop an error boundary from the stack.
         * @returns {Object|null} - The popped boundary or null
         */
        pop() {
            return this.boundaryStack.pop() || null;
        },

        /**
         * Get the current (top) error boundary.
         * @returns {Object|null} - Current boundary or null
         */
        current() {
            return this.boundaryStack.length > 0
                ? this.boundaryStack[this.boundaryStack.length - 1]
                : null;
        },

        /**
         * Handle an error during render.
         * @param {Error} error - The error that occurred
         * @returns {any} - Fallback result or throws if no boundary
         */
        handleError(error) {
            const boundary = this.current();

            // Log the error
            console.error('RustScript render error:', error);

            if (boundary) {
                boundary.hasError = true;
                boundary.error = error;

                // Store error state for component
                this.errorStates.set(boundary.componentId, {
                    error: error,
                    timestamp: Date.now()
                });

                // Call global error handler if set
                if (this.globalErrorHandler) {
                    try {
                        this.globalErrorHandler(error, boundary.componentId);
                    } catch (e) {
                        console.error('Global error handler threw:', e);
                    }
                }

                // Return fallback if available
                if (boundary.fallback) {
                    try {
                        return boundary.fallback(error, boundary.componentId);
                    } catch (fallbackError) {
                        console.error('Error boundary fallback threw:', fallbackError);
                        // Propagate to parent boundary
                        this.pop();
                        return this.handleError(fallbackError);
                    }
                }

                return null;
            }

            // No boundary - call global handler and throw
            if (this.globalErrorHandler) {
                this.globalErrorHandler(error, null);
            }

            throw error;
        },

        /**
         * Wrap a render function in try/catch with error boundary handling.
         * @param {Function} renderFn - The render function to wrap
         * @returns {any} - Render result or fallback
         */
        wrapRender(renderFn) {
            try {
                return renderFn();
            } catch (error) {
                return this.handleError(error);
            }
        },

        /**
         * Check if a component has an error.
         * @param {number} componentId - Component ID
         * @returns {boolean} - Whether component has error
         */
        hasError(componentId) {
            return this.errorStates.has(componentId);
        },

        /**
         * Get the error for a component.
         * @param {number} componentId - Component ID
         * @returns {Object|null} - Error state or null
         */
        getError(componentId) {
            return this.errorStates.get(componentId) || null;
        },

        /**
         * Clear the error for a component (for retry).
         * @param {number} componentId - Component ID
         */
        clearError(componentId) {
            this.errorStates.delete(componentId);
        },

        /**
         * Clear all error states.
         */
        clearAllErrors() {
            this.errorStates.clear();
        },

        /**
         * Set the global error handler.
         * @param {Function} handler - Error handler function(error, componentId)
         */
        setGlobalErrorHandler(handler) {
            this.globalErrorHandler = handler;
        },

        /**
         * Create an error boundary component wrapper.
         * @param {number} componentId - Component ID for this boundary
         * @param {Function} renderFn - Render function
         * @param {Function} fallbackFn - Fallback function(error)
         * @returns {any} - Render result or fallback
         */
        createBoundary(componentId, renderFn, fallbackFn) {
            this.push(componentId, fallbackFn);
            const result = this.wrapRender(renderFn);
            this.pop();
            return result;
        },

        /**
         * Reset error boundary state for retry.
         * @param {number} componentId - Component ID
         * @param {Function} renderFn - Render function to retry
         * @returns {any} - New render result
         */
        retry(componentId, renderFn) {
            this.clearError(componentId);
            const instance = componentManager.get(componentId);
            if (instance) {
                instance.error = null;
            }
            return this.wrapRender(renderFn);
        }
    };

    // =========================================================================
    // Reactive Re-rendering (RS-147)
    // =========================================================================

    /**
     * Reactive system for fine-grained DOM updates.
     * Tracks signal dependencies during render and batches updates.
     */
    const reactivity = {
        // Current reactive context (component/effect being rendered)
        currentContext: null,

        // Map of signalId -> Set of dependent updaters
        dependencies: new Map(),

        // Pending updates to batch
        pendingUpdates: new Set(),

        // Whether a flush is scheduled
        flushScheduled: false,

        // Current batch depth (for nested batches)
        batchDepth: 0,

        // Virtual DOM cache for diffing
        vdomCache: new WeakMap(),

        /**
         * Schedule an updater function to run.
         * Updates are batched and run in the next microtask.
         * @param {Function} updater - The update function to run
         */
        scheduleUpdate(updater) {
            this.pendingUpdates.add(updater);

            if (!this.flushScheduled && this.batchDepth === 0) {
                this.flushScheduled = true;
                queueMicrotask(() => this.flush());
            }
        },

        /**
         * Run all pending updates in a batch.
         * Updates are deduplicated and run in order.
         */
        flush() {
            this.flushScheduled = false;

            // Copy updates to avoid issues if new updates are scheduled during flush
            const updates = Array.from(this.pendingUpdates);
            this.pendingUpdates.clear();

            // Run all updates
            for (const updater of updates) {
                try {
                    updater();
                } catch (error) {
                    console.error('[RustScript Reactivity] Error in updater:', error);
                }
            }
        },

        /**
         * Start a batch of updates.
         * Updates won't flush until the batch ends.
         */
        startBatch() {
            this.batchDepth++;
        },

        /**
         * End a batch of updates.
         * If this is the outermost batch, flush updates.
         */
        endBatch() {
            this.batchDepth--;
            if (this.batchDepth === 0 && this.pendingUpdates.size > 0) {
                this.flush();
            }
        },

        /**
         * Track a signal dependency in the current context.
         * Called when a signal is read during a reactive context.
         * @param {number|symbol} signalId - The signal identifier
         */
        track(signalId) {
            if (this.currentContext === null) {
                return; // Not in a reactive context
            }

            if (!this.dependencies.has(signalId)) {
                this.dependencies.set(signalId, new Set());
            }

            this.dependencies.get(signalId).add(this.currentContext);
        },

        /**
         * Trigger updates for all dependents of a signal.
         * Called when a signal value changes.
         * @param {number|symbol} signalId - The signal identifier
         */
        trigger(signalId) {
            const deps = this.dependencies.get(signalId);
            if (!deps || deps.size === 0) {
                return;
            }

            for (const updater of deps) {
                this.scheduleUpdate(updater);
            }
        },

        /**
         * Run a function in a reactive context.
         * Dependencies are tracked while the function runs.
         * @param {Function} fn - The function to run
         * @param {Function} updater - The updater to register for dependencies
         * @returns {any} - The return value of fn
         */
        runInContext(fn, updater) {
            const prevContext = this.currentContext;
            this.currentContext = updater;

            try {
                return fn();
            } finally {
                this.currentContext = prevContext;
            }
        },

        /**
         * Create a reactive effect that re-runs when dependencies change.
         * @param {Function} fn - The effect function
         * @returns {Function} - Cleanup function to stop the effect
         */
        createEffect(fn) {
            let cleanup = null;

            const run = () => {
                // Run any previous cleanup
                if (cleanup && typeof cleanup === 'function') {
                    cleanup();
                }

                // Clear old dependencies for this effect
                this.cleanupDependencies(run);

                // Run the effect and track new dependencies
                cleanup = this.runInContext(fn, run);
            };

            // Run immediately
            run();

            // Return cleanup function
            return () => {
                this.cleanupDependencies(run);
                if (cleanup && typeof cleanup === 'function') {
                    cleanup();
                }
            };
        },

        /**
         * Remove an updater from all dependency sets.
         * @param {Function} updater - The updater to remove
         */
        cleanupDependencies(updater) {
            for (const deps of this.dependencies.values()) {
                deps.delete(updater);
            }
        },

        /**
         * Create a computed value that updates when dependencies change.
         * @param {Function} getter - The getter function
         * @returns {Object} - Object with get() method
         */
        createComputed(getter) {
            let value;
            let dirty = true;
            const computedId = Symbol('computed');

            const updater = () => {
                dirty = true;
                // Trigger any dependents of this computed
                this.trigger(computedId);
            };

            return {
                get: () => {
                    if (dirty) {
                        value = this.runInContext(getter, updater);
                        dirty = false;
                    }
                    // Track this computed as a dependency
                    this.track(computedId);
                    return value;
                },
                id: computedId
            };
        },

        /**
         * Create a signal (reactive value).
         * @param {any} initialValue - The initial value
         * @returns {Object} - Object with get() and set() methods
         */
        createSignal(initialValue) {
            const signalId = Symbol('signal');
            let value = initialValue;

            return {
                get: () => {
                    this.track(signalId);
                    return value;
                },
                set: (newValue) => {
                    if (value !== newValue) {
                        value = newValue;
                        this.trigger(signalId);
                    }
                },
                id: signalId
            };
        }
    };

    // =========================================================================
    // Virtual DOM Diff (RS-147)
    // =========================================================================

    /**
     * Simple virtual DOM implementation for fine-grained updates.
     */
    const vdom = {
        /**
         * Create a virtual text node.
         * @param {string} text - The text content
         * @returns {Object} - Virtual node
         */
        text(text) {
            return { type: 'text', value: String(text) };
        },

        /**
         * Create a virtual element node.
         * @param {string} tag - The tag name
         * @param {Object} attrs - Attributes object
         * @param {Array} children - Child virtual nodes
         * @returns {Object} - Virtual node
         */
        element(tag, attrs = {}, children = []) {
            return { type: 'element', tag, attrs, children };
        },

        /**
         * Diff and patch text content.
         * Only updates if the content has changed.
         * @param {Node} node - The DOM node
         * @param {string} newText - The new text content
         * @returns {boolean} - Whether an update was made
         */
        patchText(node, newText) {
            const newTextStr = String(newText);
            if (node.textContent !== newTextStr) {
                node.textContent = newTextStr;
                return true;
            }
            return false;
        },

        /**
         * Diff and patch attributes on an element.
         * Only updates attributes that have changed.
         * @param {Element} element - The DOM element
         * @param {Object} oldAttrs - Previous attributes
         * @param {Object} newAttrs - New attributes
         * @returns {boolean} - Whether any updates were made
         */
        patchAttrs(element, oldAttrs, newAttrs) {
            let updated = false;

            // Remove old attributes not in new
            for (const key of Object.keys(oldAttrs)) {
                if (!(key in newAttrs)) {
                    if (key.startsWith('on')) {
                        // Event handler - skip, handled separately
                    } else if (key === 'style' && typeof oldAttrs[key] === 'object') {
                        // Clear all style properties
                        for (const prop of Object.keys(oldAttrs[key])) {
                            element.style.removeProperty(prop);
                        }
                    } else if (key === 'class' || key === 'className') {
                        element.className = '';
                    } else {
                        element.removeAttribute(key);
                    }
                    updated = true;
                }
            }

            // Add/update new attributes
            for (const [key, value] of Object.entries(newAttrs)) {
                if (key.startsWith('on')) {
                    // Event handlers handled separately
                    continue;
                }

                const oldValue = oldAttrs[key];

                if (key === 'style') {
                    if (typeof value === 'object') {
                        // Style object
                        const oldStyle = typeof oldValue === 'object' ? oldValue : {};
                        for (const [prop, val] of Object.entries(value)) {
                            if (oldStyle[prop] !== val) {
                                element.style.setProperty(prop, val);
                                updated = true;
                            }
                        }
                        // Remove old style properties
                        for (const prop of Object.keys(oldStyle)) {
                            if (!(prop in value)) {
                                element.style.removeProperty(prop);
                                updated = true;
                            }
                        }
                    } else if (typeof value === 'string' && oldValue !== value) {
                        element.setAttribute('style', value);
                        updated = true;
                    }
                } else if (key === 'class' || key === 'className') {
                    if (element.className !== value) {
                        element.className = value;
                        updated = true;
                    }
                } else if (key === 'value' && 'value' in element) {
                    if (element.value !== value) {
                        element.value = value;
                        updated = true;
                    }
                } else if (key === 'checked' && 'checked' in element) {
                    if (element.checked !== value) {
                        element.checked = value;
                        updated = true;
                    }
                } else if (key === 'disabled' || key === 'readonly' || key === 'hidden') {
                    if (value) {
                        if (!element.hasAttribute(key)) {
                            element.setAttribute(key, '');
                            updated = true;
                        }
                    } else {
                        if (element.hasAttribute(key)) {
                            element.removeAttribute(key);
                            updated = true;
                        }
                    }
                } else if (oldValue !== value) {
                    if (value === null || value === undefined || value === false) {
                        element.removeAttribute(key);
                    } else {
                        element.setAttribute(key, value === true ? '' : String(value));
                    }
                    updated = true;
                }
            }

            return updated;
        },

        /**
         * Diff and patch children of an element.
         * Uses a simple keyed reconciliation algorithm.
         * @param {Element} parent - The parent DOM element
         * @param {Array} oldChildren - Previous virtual children
         * @param {Array} newChildren - New virtual children
         * @returns {boolean} - Whether any updates were made
         */
        patchChildren(parent, oldChildren, newChildren) {
            let updated = false;

            // Build key map for old children
            const oldKeyMap = new Map();
            const oldNodes = Array.from(parent.childNodes);

            oldChildren.forEach((child, index) => {
                const key = child.key !== undefined ? child.key : index;
                oldKeyMap.set(key, { vnode: child, node: oldNodes[index], index });
            });

            // Track which old nodes to keep
            const usedOldNodes = new Set();

            // Reconcile new children
            newChildren.forEach((newChild, newIndex) => {
                const key = newChild.key !== undefined ? newChild.key : newIndex;
                const old = oldKeyMap.get(key);

                if (old) {
                    // Reuse existing node
                    usedOldNodes.add(key);
                    const patchResult = this.patch(old.node, old.vnode, newChild);
                    updated = updated || patchResult.updated;

                    // Move if needed
                    const currentNode = parent.childNodes[newIndex];
                    if (currentNode !== old.node) {
                        parent.insertBefore(old.node, currentNode || null);
                        updated = true;
                    }
                } else {
                    // Create new node
                    const newNode = this.create(newChild);
                    const refNode = parent.childNodes[newIndex];
                    parent.insertBefore(newNode, refNode || null);
                    updated = true;
                }
            });

            // Remove unused old nodes
            for (const [key, old] of oldKeyMap) {
                if (!usedOldNodes.has(key) && old.node && old.node.parentNode === parent) {
                    parent.removeChild(old.node);
                    updated = true;
                }
            }

            return updated;
        },

        /**
         * Create a DOM node from a virtual node.
         * @param {Object} vnode - The virtual node
         * @returns {Node} - The DOM node
         */
        create(vnode) {
            if (vnode.type === 'text') {
                return document.createTextNode(vnode.value);
            }

            const element = document.createElement(vnode.tag);

            // Set attributes
            for (const [key, value] of Object.entries(vnode.attrs || {})) {
                if (key.startsWith('on')) {
                    // Event handlers
                    const eventName = key.slice(2).toLowerCase();
                    element.addEventListener(eventName, value);
                } else if (key === 'style' && typeof value === 'object') {
                    for (const [prop, val] of Object.entries(value)) {
                        element.style.setProperty(prop, val);
                    }
                } else if (key === 'class' || key === 'className') {
                    element.className = value;
                } else if (value !== null && value !== undefined && value !== false) {
                    element.setAttribute(key, value === true ? '' : String(value));
                }
            }

            // Add children
            for (const child of vnode.children || []) {
                element.appendChild(this.create(child));
            }

            return element;
        },

        /**
         * Patch a DOM node to match a new virtual node.
         * @param {Node} node - The existing DOM node
         * @param {Object} oldVnode - The old virtual node
         * @param {Object} newVnode - The new virtual node
         * @returns {Object} - { node, updated }
         */
        patch(node, oldVnode, newVnode) {
            // Different types - replace entirely
            if (oldVnode.type !== newVnode.type) {
                const newNode = this.create(newVnode);
                node.parentNode.replaceChild(newNode, node);
                return { node: newNode, updated: true };
            }

            // Text nodes
            if (newVnode.type === 'text') {
                const updated = this.patchText(node, newVnode.value);
                return { node, updated };
            }

            // Element nodes - different tags, replace
            if (oldVnode.tag !== newVnode.tag) {
                const newNode = this.create(newVnode);
                node.parentNode.replaceChild(newNode, node);
                return { node: newNode, updated: true };
            }

            // Same element type - patch attrs and children
            const attrsUpdated = this.patchAttrs(node, oldVnode.attrs || {}, newVnode.attrs || {});
            const childrenUpdated = this.patchChildren(node, oldVnode.children || [], newVnode.children || []);

            return { node, updated: attrsUpdated || childrenUpdated };
        }
    };

    // =========================================================================
    // Reactive DOM Bindings (RS-147)
    // =========================================================================

    /**
     * Reactive DOM binding utilities.
     * Creates fine-grained DOM updates tied to signals.
     */
    const reactiveDOM = {
        /**
         * Bind a signal to a text node's content.
         * Only updates the text when the signal changes.
         * @param {Object} signal - Signal with get() method
         * @param {Node} textNode - The text node to update
         * @returns {Function} - Cleanup function
         */
        bindText(signal, textNode) {
            return reactivity.createEffect(() => {
                const value = signal.get();
                vdom.patchText(textNode, value);
            });
        },

        /**
         * Bind a signal to an attribute.
         * Only updates the attribute when the signal changes.
         * @param {Object} signal - Signal with get() method
         * @param {Element} element - The element
         * @param {string} attrName - The attribute name
         * @returns {Function} - Cleanup function
         */
        bindAttribute(signal, element, attrName) {
            let oldValue = undefined;

            return reactivity.createEffect(() => {
                const newValue = signal.get();
                if (oldValue !== newValue) {
                    if (newValue === null || newValue === undefined || newValue === false) {
                        element.removeAttribute(attrName);
                    } else {
                        element.setAttribute(attrName, newValue === true ? '' : String(newValue));
                    }
                    oldValue = newValue;
                }
            });
        },

        /**
         * Bind a signal to an element's style property.
         * @param {Object} signal - Signal with get() method
         * @param {Element} element - The element
         * @param {string} property - The CSS property
         * @returns {Function} - Cleanup function
         */
        bindStyle(signal, element, property) {
            let oldValue = undefined;

            return reactivity.createEffect(() => {
                const newValue = signal.get();
                if (oldValue !== newValue) {
                    element.style.setProperty(property, newValue);
                    oldValue = newValue;
                }
            });
        },

        /**
         * Bind a signal to an element's class list.
         * @param {Object} signal - Signal with get() method (returns boolean)
         * @param {Element} element - The element
         * @param {string} className - The class name to toggle
         * @returns {Function} - Cleanup function
         */
        bindClass(signal, element, className) {
            return reactivity.createEffect(() => {
                const active = signal.get();
                if (active) {
                    element.classList.add(className);
                } else {
                    element.classList.remove(className);
                }
            });
        },

        /**
         * Bind a signal to a list of children.
         * Efficiently patches the children when the list changes.
         * @param {Object} signal - Signal returning array of items
         * @param {Element} parent - The parent element
         * @param {Function} renderItem - Function to render each item to a vnode
         * @returns {Function} - Cleanup function
         */
        bindList(signal, parent, renderItem) {
            let oldVnodes = [];

            return reactivity.createEffect(() => {
                const items = signal.get();
                const newVnodes = items.map((item, index) => {
                    const vnode = renderItem(item, index);
                    vnode.key = item.key !== undefined ? item.key : (item.id !== undefined ? item.id : index);
                    return vnode;
                });

                vdom.patchChildren(parent, oldVnodes, newVnodes);
                oldVnodes = newVnodes;
            });
        },

        /**
         * Create a reactive component that re-renders when signals change.
         * @param {Element} container - The container element
         * @param {Function} render - Render function returning virtual DOM
         * @returns {Function} - Cleanup function
         */
        mount(container, render) {
            let oldVnode = null;
            let rootNode = null;

            return reactivity.createEffect(() => {
                const newVnode = render();

                if (oldVnode === null) {
                    // First render
                    rootNode = vdom.create(newVnode);
                    container.appendChild(rootNode);
                } else {
                    // Subsequent renders - diff and patch
                    const result = vdom.patch(rootNode, oldVnode, newVnode);
                    rootNode = result.node;
                }

                oldVnode = newVnode;
            });
        }
    };

    // =========================================================================
    // Reactivity Interface for WASM (RS-147)
    // =========================================================================

    /**
     * Reactivity interface for WASM imports.
     * Exposes reactive primitives to RustScript code.
     */
    const reactivityInterface = {
        /**
         * Create a signal with an initial value.
         * @param {number} valuePtr - Pointer to initial value
         * @param {number} valueLen - Length of value
         * @returns {number} - Signal handle
         */
        createSignal(valuePtr, valueLen) {
            const initialValue = valuePtr && valueLen > 0 ? readString(valuePtr, valueLen) : null;
            const signal = reactivity.createSignal(initialValue);
            return elementHandles.alloc(signal);
        },

        /**
         * Get a signal's current value.
         * @param {number} signalHandle - Signal handle
         * @param {number} outPtr - Output buffer pointer
         * @param {number} outLen - Max output length
         * @returns {number} - Bytes written or -1 if not found
         */
        getSignal(signalHandle, outPtr, outLen) {
            const signal = elementHandles.get(signalHandle);
            if (signal && signal.get) {
                const value = signal.get();
                if (value !== null && value !== undefined) {
                    return writeString(String(value), outPtr, outLen);
                }
            }
            return -1;
        },

        /**
         * Set a signal's value.
         * @param {number} signalHandle - Signal handle
         * @param {number} valuePtr - Pointer to new value
         * @param {number} valueLen - Length of value
         */
        setSignal(signalHandle, valuePtr, valueLen) {
            const signal = elementHandles.get(signalHandle);
            if (signal && signal.set) {
                const value = valuePtr && valueLen > 0 ? readString(valuePtr, valueLen) : null;
                signal.set(value);
            }
        },

        /**
         * Create a computed value.
         * @param {number} getterPtr - Pointer to getter function
         * @returns {number} - Computed handle
         */
        createComputed(getterPtr) {
            // In WASM context, we'd call the getter through exports
            const computed = reactivity.createComputed(() => {
                if (wasmInstance && wasmInstance.exports.__rsc_call_getter) {
                    return wasmInstance.exports.__rsc_call_getter(getterPtr);
                }
                return null;
            });
            return elementHandles.alloc(computed);
        },

        /**
         * Get a computed value.
         * @param {number} computedHandle - Computed handle
         * @param {number} outPtr - Output buffer pointer
         * @param {number} outLen - Max output length
         * @returns {number} - Bytes written or -1 if not found
         */
        getComputed(computedHandle, outPtr, outLen) {
            const computed = elementHandles.get(computedHandle);
            if (computed && computed.get) {
                const value = computed.get();
                if (value !== null && value !== undefined) {
                    return writeString(String(value), outPtr, outLen);
                }
            }
            return -1;
        },

        /**
         * Create a reactive effect.
         * @param {number} effectPtr - Pointer to effect function
         * @returns {number} - Effect cleanup handle
         */
        createEffect(effectPtr) {
            const cleanup = reactivity.createEffect(() => {
                if (wasmInstance && wasmInstance.exports.__rsc_call_effect) {
                    wasmInstance.exports.__rsc_call_effect(effectPtr);
                }
            });
            return elementHandles.alloc({ cleanup });
        },

        /**
         * Dispose an effect.
         * @param {number} effectHandle - Effect cleanup handle
         */
        disposeEffect(effectHandle) {
            const effect = elementHandles.get(effectHandle);
            if (effect && effect.cleanup) {
                effect.cleanup();
            }
            elementHandles.free(effectHandle);
        },

        /**
         * Start a batch of updates.
         */
        startBatch() {
            reactivity.startBatch();
        },

        /**
         * End a batch of updates.
         */
        endBatch() {
            reactivity.endBatch();
        },

        /**
         * Bind a signal to a text node.
         * @param {number} signalHandle - Signal handle
         * @param {number} nodeHandle - Text node handle
         * @returns {number} - Binding cleanup handle
         */
        bindText(signalHandle, nodeHandle) {
            const signal = elementHandles.get(signalHandle);
            const node = elementHandles.get(nodeHandle);
            if (signal && node) {
                const cleanup = reactiveDOM.bindText(signal, node);
                return elementHandles.alloc({ cleanup });
            }
            return 0;
        },

        /**
         * Bind a signal to an attribute.
         * @param {number} signalHandle - Signal handle
         * @param {number} elementHandle - Element handle
         * @param {number} attrPtr - Attribute name pointer
         * @param {number} attrLen - Attribute name length
         * @returns {number} - Binding cleanup handle
         */
        bindAttribute(signalHandle, elementHandle, attrPtr, attrLen) {
            const signal = elementHandles.get(signalHandle);
            const element = elementHandles.get(elementHandle);
            const attrName = readString(attrPtr, attrLen);
            if (signal && element) {
                const cleanup = reactiveDOM.bindAttribute(signal, element, attrName);
                return elementHandles.alloc({ cleanup });
            }
            return 0;
        },

        /**
         * Bind a signal to a style property.
         * @param {number} signalHandle - Signal handle
         * @param {number} elementHandle - Element handle
         * @param {number} propPtr - Property name pointer
         * @param {number} propLen - Property name length
         * @returns {number} - Binding cleanup handle
         */
        bindStyle(signalHandle, elementHandle, propPtr, propLen) {
            const signal = elementHandles.get(signalHandle);
            const element = elementHandles.get(elementHandle);
            const property = readString(propPtr, propLen);
            if (signal && element) {
                const cleanup = reactiveDOM.bindStyle(signal, element, property);
                return elementHandles.alloc({ cleanup });
            }
            return 0;
        },

        /**
         * Bind a signal to a class toggle.
         * @param {number} signalHandle - Signal handle
         * @param {number} elementHandle - Element handle
         * @param {number} classPtr - Class name pointer
         * @param {number} classLen - Class name length
         * @returns {number} - Binding cleanup handle
         */
        bindClass(signalHandle, elementHandle, classPtr, classLen) {
            const signal = elementHandles.get(signalHandle);
            const element = elementHandles.get(elementHandle);
            const className = readString(classPtr, classLen);
            if (signal && element) {
                const cleanup = reactiveDOM.bindClass(signal, element, className);
                return elementHandles.alloc({ cleanup });
            }
            return 0;
        },

        /**
         * Dispose a binding.
         * @param {number} bindingHandle - Binding cleanup handle
         */
        disposeBinding(bindingHandle) {
            const binding = elementHandles.get(bindingHandle);
            if (binding && binding.cleanup) {
                binding.cleanup();
            }
            elementHandles.free(bindingHandle);
        },

        /**
         * Flush pending updates immediately.
         */
        flush() {
            reactivity.flush();
        }
    };

    // =========================================================================
    // Component Lifecycle and Effect System (RS-144, RS-148)
    // =========================================================================

    /**
     * Effect system for reactive side effects with automatic dependency tracking.
     * Implements a React-like useEffect/Solid-like createEffect pattern:
     *
     * Key features:
     * - Automatic dependency tracking: signals read during effect execution
     *   are automatically tracked as dependencies
     * - Cleanup functions: effects can return a cleanup function that runs
     *   before re-execution or on disposal
     * - Batched updates: multiple signal changes are batched to avoid
     *   cascading updates
     * - Component-scoped disposal: all effects for a component are cleaned up
     *   when the component unmounts
     */
    class EffectSystem {
        constructor() {
            /** Map of effectId -> EffectState */
            this.effects = new Map();
            /** Currently executing effect (for auto-dependency tracking) */
            this.currentEffect = null;
            /** Effects scheduled for re-execution */
            this.pendingEffects = new Set();
            /** Prevents recursive flushing */
            this.isFlushing = false;
            /** Next effect ID to assign */
            this.nextEffectId = 1;
            /** Map of componentId -> Set<effectId> for disposal */
            this.componentEffects = new Map();
        }

        /**
         * Run an effect function with automatic dependency tracking.
         *
         * @param {Function|number} effectFn - Effect function or WASM callback pointer
         * @param {Array|null} explicitDeps - Explicit dependency signal IDs (null = auto-track)
         * @returns {number} - Effect ID for cleanup/tracking
         *
         * @example
         * // Auto-tracking mode (like Solid's createEffect)
         * runEffect(() => {
         *     console.log(count.get()); // count is auto-tracked as dependency
         * });
         *
         * @example
         * // With explicit deps (like React's useEffect)
         * runEffect(() => {
         *     console.log(count.get());
         * }, [countSignalId]);
         *
         * @example
         * // With cleanup
         * runEffect(() => {
         *     const timer = setInterval(...);
         *     return () => clearInterval(timer); // cleanup
         * });
         */
        runEffect(effectFn, explicitDeps = null) {
            const effectId = this.nextEffectId++;

            // Create effect state
            const effectState = {
                id: effectId,
                fn: effectFn,
                cleanup: null,
                dependencies: new Set(),        // Auto-tracked signal IDs
                explicitDeps: explicitDeps,     // Explicit deps (null = auto-track mode)
                disposed: false
            };

            this.effects.set(effectId, effectState);

            // Execute the effect immediately (like Solid's createEffect)
            this._executeEffect(effectId);

            return effectId;
        }

        /**
         * Execute an effect and track its dependencies.
         * @param {number} effectId - Effect ID to execute
         * @private
         */
        _executeEffect(effectId) {
            const effectState = this.effects.get(effectId);
            if (!effectState || effectState.disposed) return;

            // Run cleanup from previous execution
            if (effectState.cleanup) {
                this._runCleanup(effectState);
            }

            // Clear previous auto-tracked dependencies
            if (!effectState.explicitDeps) {
                effectState.dependencies.clear();
            }

            // Set current effect for dependency tracking
            const prevEffect = this.currentEffect;
            this.currentEffect = effectId;

            try {
                let result;
                if (typeof effectState.fn === 'function') {
                    // JavaScript function
                    result = effectState.fn();
                } else if (wasmInstance && wasmInstance.exports.__rsc_call_effect) {
                    // WASM callback pointer
                    result = wasmInstance.exports.__rsc_call_effect(effectState.fn);
                }

                // If effect returned a cleanup function, store it
                if (typeof result === 'function') {
                    effectState.cleanup = result;
                } else if (typeof result === 'number' && result !== 0) {
                    // WASM returned a cleanup callback pointer
                    effectState.cleanup = result;
                }
            } catch (error) {
                console.error(`Effect ${effectId} execution error:`, error);
            } finally {
                this.currentEffect = prevEffect;
            }
        }

        /**
         * Track a dependency for the current effect.
         * Called when a signal is read during effect execution.
         *
         * @param {number} signalId - Signal ID that was read
         */
        trackDependency(signalId) {
            if (this.currentEffect === null) return;

            const effectState = this.effects.get(this.currentEffect);
            if (!effectState || effectState.explicitDeps) return; // Skip if using explicit deps

            effectState.dependencies.add(signalId);
        }

        /**
         * Notify that a signal changed - schedule dependent effects for re-run.
         *
         * @param {number} signalId - Signal ID that changed
         */
        notifySignalChange(signalId) {
            // Find all effects that depend on this signal
            for (const [effectId, effectState] of this.effects) {
                if (effectState.disposed) continue;

                const deps = effectState.explicitDeps || effectState.dependencies;
                if (deps.has(signalId)) {
                    this.pendingEffects.add(effectId);
                }
            }

            // Schedule flush if not already flushing
            if (!this.isFlushing && this.pendingEffects.size > 0) {
                this.isFlushing = true;
                queueMicrotask(() => this._flushEffects());
            }
        }

        /**
         * Flush pending effect updates (batched re-execution).
         * @private
         */
        _flushEffects() {
            const effects = Array.from(this.pendingEffects);
            this.pendingEffects.clear();
            this.isFlushing = false;

            for (const effectId of effects) {
                this._executeEffect(effectId);
            }
        }

        /**
         * Run cleanup function for an effect.
         * @param {Object} effectState - Effect state object
         * @private
         */
        _runCleanup(effectState) {
            if (!effectState.cleanup) return;

            try {
                if (typeof effectState.cleanup === 'function') {
                    effectState.cleanup();
                } else if (wasmInstance && wasmInstance.exports.__rsc_call_cleanup) {
                    wasmInstance.exports.__rsc_call_cleanup(effectState.cleanup);
                }
            } catch (error) {
                console.error(`Effect cleanup error:`, error);
            }

            effectState.cleanup = null;
        }

        /**
         * Cleanup an effect and unsubscribe from all dependencies.
         *
         * @param {number} effectId - Effect ID to cleanup
         */
        cleanupEffect(effectId) {
            const effectState = this.effects.get(effectId);
            if (!effectState) return;

            // Run cleanup function
            this._runCleanup(effectState);

            // Mark as disposed
            effectState.dependencies.clear();
            effectState.disposed = true;
            this.effects.delete(effectId);
            this.pendingEffects.delete(effectId);
        }

        /**
         * Track an effect for a component (for disposal on unmount).
         *
         * @param {number} componentId - Component ID
         * @param {number} effectId - Effect ID
         */
        trackForComponent(componentId, effectId) {
            if (!this.componentEffects.has(componentId)) {
                this.componentEffects.set(componentId, new Set());
            }
            this.componentEffects.get(componentId).add(effectId);
        }

        /**
         * Dispose all effects for a component (called on unmount).
         *
         * @param {number} componentId - Component ID
         */
        disposeComponentEffects(componentId) {
            const effects = this.componentEffects.get(componentId);
            if (!effects) return;

            for (const effectId of effects) {
                this.cleanupEffect(effectId);
            }

            this.componentEffects.delete(componentId);
        }

        /**
         * Get the currently executing effect ID (for signal tracking).
         * @returns {number|null} - Current effect ID or null
         */
        getCurrentEffect() {
            return this.currentEffect;
        }
    }

    // Create global effect system instance
    const effectSystem = new EffectSystem();

    /**
     * Component lifecycle manager.
     * Handles mount/unmount callbacks and effects.
     */
    const lifecycleManager = {
        mounts: new Map(),
        unmounts: new Map(),
        nextId: 1
    };

    const lifecycle = {
        /**
         * Register a mount callback.
         * @param {number} callbackPtr - Pointer to callback function
         * @returns {number} - Mount ID for cleanup registration
         */
        onMount(callbackPtr) {
            const id = lifecycleManager.nextId++;
            lifecycleManager.mounts.set(id, callbackPtr);

            // Schedule execution after current render
            queueMicrotask(() => {
                if (wasmInstance && wasmInstance.exports.__rsc_call_mount) {
                    wasmInstance.exports.__rsc_call_mount(callbackPtr);
                }
            });

            return id;
        },

        /**
         * Register an unmount cleanup callback.
         * @param {number} mountId - Mount ID from onMount
         * @param {number} cleanupPtr - Pointer to cleanup function
         */
        onUnmount(mountId, cleanupPtr) {
            lifecycleManager.unmounts.set(mountId, cleanupPtr);
        },

        // =====================================================================
        // Effect API (RS-148: Effect Dependency Tracking)
        // =====================================================================

        /**
         * Run an effect with automatic dependency tracking.
         * This is the main entry point for effects from WASM.
         *
         * The effect function will be executed immediately, and any signals
         * read during execution will be automatically tracked as dependencies.
         * When those signals change, the effect will be re-run.
         *
         * If the effect returns a function, it will be called as cleanup
         * before the effect re-runs or when disposed.
         *
         * @param {number} effectFnPtr - Pointer to effect function
         * @param {number} depsPtr - Pointer to explicit dependencies (0 for auto-track)
         * @param {number} depsLen - Number of explicit dependencies
         * @returns {number} - Effect ID for cleanup/tracking
         *
         * @example RustScript usage:
         * ```rustscript
         * effect(|| {
         *     // This runs when count changes
         *     console.log(count.get());
         *
         *     // Optional cleanup
         *     || { console.log("cleanup"); }
         * });
         * ```
         */
        runEffect(effectFnPtr, depsPtr = 0, depsLen = 0) {
            // Parse explicit dependencies if provided
            let explicitDeps = null;
            if (depsPtr !== 0 && depsLen > 0 && wasmMemory) {
                const depsArray = new Uint32Array(wasmMemory.buffer, depsPtr, depsLen);
                explicitDeps = new Set(depsArray);
            }

            return effectSystem.runEffect(effectFnPtr, explicitDeps);
        },

        /**
         * Run an effect with a JavaScript function (for JS interop).
         * Same semantics as runEffect but accepts a JS function directly.
         *
         * @param {Function} effectFn - Effect function
         * @returns {number} - Effect ID
         */
        createEffect(effectFn) {
            return effectSystem.runEffect(effectFn, null);
        },

        /**
         * Cleanup an effect and unsubscribe from dependencies.
         * Call this to manually dispose an effect before component unmount.
         *
         * @param {number} effectId - Effect ID to cleanup
         */
        cleanupEffect(effectId) {
            effectSystem.cleanupEffect(effectId);
        },

        /**
         * Track an effect for a component (for disposal on unmount).
         * Effects tracked this way will be automatically cleaned up
         * when disposeComponent is called.
         *
         * @param {number} componentId - Component ID
         * @param {number} effectId - Effect ID
         */
        trackEffect(componentId, effectId) {
            effectSystem.trackForComponent(componentId, effectId);
        },

        /**
         * Track a signal read (called by signal.get() for auto-tracking).
         * This is used by the signal runtime to notify the effect system
         * that a signal was read during effect execution.
         *
         * @param {number} signalId - Signal ID that was read
         */
        trackSignalRead(signalId) {
            effectSystem.trackDependency(signalId);
        },

        /**
         * Notify that a signal changed (called by signal.set()).
         * This triggers re-execution of all effects that depend on the signal.
         *
         * @param {number} signalId - Signal ID that changed
         */
        notifySignalChange(signalId) {
            effectSystem.notifySignalChange(signalId);
        },

        /**
         * Get the currently executing effect (for signal tracking integration).
         * @returns {number|null} - Current effect ID or null
         */
        getCurrentEffect() {
            return effectSystem.getCurrentEffect();
        },

        /**
         * Dispose component and run cleanup for all effects.
         * This should be called when a component unmounts.
         *
         * @param {number} componentId - Component ID
         */
        disposeComponent(componentId) {
            // Dispose all effects for this component
            effectSystem.disposeComponentEffects(componentId);

            // Run unmount callbacks
            const cleanup = lifecycleManager.unmounts.get(componentId);
            if (cleanup && wasmInstance && wasmInstance.exports.__rsc_call_cleanup) {
                wasmInstance.exports.__rsc_call_cleanup(cleanup);
            }
            lifecycleManager.unmounts.delete(componentId);
            lifecycleManager.mounts.delete(componentId);
        },

        // =====================================================================
        // Legacy API (backward compatibility with RS-144)
        // =====================================================================

        /**
         * @deprecated Use runEffect instead
         * Legacy effect registration API.
         */
        registerEffect(effectPtr, depsPtr, depsLen) {
            return this.runEffect(effectPtr, depsPtr, depsLen);
        }
    };

    // =========================================================================
    // Initialization
    // =========================================================================

    // =========================================================================
    // Components Interface (RS-150, RS-154)
    // =========================================================================

    /**
     * Components interface for WASM imports.
     * Provides component management and error boundary APIs.
     */
    const components = {
        // === Component Instance Management ===

        /**
         * Create a new component instance.
         * @param {number} renderFnPtr - Pointer to render function
         * @param {number} propsPtr - Pointer to props object
         * @param {number} parentHandle - Parent element handle
         * @returns {number} - Component ID
         */
        create(renderFnPtr, propsPtr, parentHandle) {
            // In WASM context, we'd decode the function pointer
            // For JS usage, we allow direct function/props passing
            return componentManager.create(renderFnPtr, propsPtr, parentHandle);
        },

        /**
         * Update a component with new props.
         * @param {number} componentId - Component ID
         * @param {number} propsPtr - Pointer to new props
         * @returns {number} - 1 on success, 0 on failure
         */
        update(componentId, propsPtr) {
            return componentManager.update(componentId, propsPtr) ? 1 : 0;
        },

        /**
         * Unmount and cleanup a component.
         * @param {number} componentId - Component ID
         * @returns {number} - 1 on success, 0 on failure
         */
        unmount(componentId) {
            return componentManager.unmount(componentId) ? 1 : 0;
        },

        /**
         * Get component count.
         * @returns {number} - Number of active components
         */
        count() {
            return componentManager.count();
        },

        /**
         * Check if component exists.
         * @param {number} componentId - Component ID
         * @returns {number} - 1 if exists, 0 otherwise
         */
        exists(componentId) {
            return componentManager.get(componentId) ? 1 : 0;
        },

        /**
         * Get component's root element handle.
         * @param {number} componentId - Component ID
         * @returns {number} - Element handle or 0
         */
        getElement(componentId) {
            const instance = componentManager.get(componentId);
            if (instance && instance.element) {
                return elementHandles.alloc(instance.element);
            }
            return 0;
        },

        /**
         * Register a child component relationship.
         * @param {number} parentId - Parent component ID
         * @param {number} childId - Child component ID
         */
        addChild(parentId, childId) {
            componentManager.addChild(parentId, childId);
        },

        // === Error Boundary ===

        /**
         * Push an error boundary.
         * @param {number} componentId - Component ID
         * @param {number} fallbackPtr - Pointer to fallback function (0 for none)
         */
        pushBoundary(componentId, fallbackPtr) {
            errorBoundary.push(componentId, fallbackPtr || null);
        },

        /**
         * Pop an error boundary.
         */
        popBoundary() {
            errorBoundary.pop();
        },

        /**
         * Check if a component has an error.
         * @param {number} componentId - Component ID
         * @returns {number} - 1 if has error, 0 otherwise
         */
        hasError(componentId) {
            return errorBoundary.hasError(componentId) ? 1 : 0;
        },

        /**
         * Clear error for a component.
         * @param {number} componentId - Component ID
         */
        clearError(componentId) {
            errorBoundary.clearError(componentId);
        },

        /**
         * Set global error handler.
         * @param {number} handlerPtr - Pointer to handler function
         */
        setErrorHandler(handlerPtr) {
            if (handlerPtr) {
                errorBoundary.setGlobalErrorHandler((error, componentId) => {
                    if (wasmInstance && wasmInstance.exports.__rsc_handle_component_error) {
                        wasmInstance.exports.__rsc_handle_component_error(
                            handlerPtr,
                            componentId || 0,
                            error.message || 'Unknown error'
                        );
                    }
                });
            } else {
                errorBoundary.setGlobalErrorHandler(null);
            }
        }
    };

    // =========================================================================
    // Two-way Bindings Interface (RS-153)
    // =========================================================================

    /**
     * Two-way binding interface for WASM imports.
     * Provides bidirectional bindings between form elements and signals.
     */
    const bindings = {
        /**
         * Create a two-way binding between an input element's value and a signal.
         * Sets up bidirectional synchronization:
         * - Sets initial element value from signal
         * - Listens for DOM events to update signal
         * - Subscribes to signal changes to update element
         *
         * @param {number} elementHandle - Element handle from WASM
         * @param {number} signalHandle - Signal handle from WASM
         * @param {number} eventTypeHandle - String handle for event type (0 for default 'input')
         */
        bindValue(elementHandle, signalHandle, eventTypeHandle) {
            const element = elementHandles.get(elementHandle);
            const signal = elementHandles.get(signalHandle);

            if (!element || !signal) {
                console.warn('[RustScript Bindings] Invalid element or signal handle');
                return;
            }

            const eventType = eventTypeHandle !== 0
                ? decodeStringHandle(eventTypeHandle)
                : 'input';

            // Set initial value from signal
            const initialValue = signal.get();
            if (initialValue !== null && initialValue !== undefined) {
                element.value = String(initialValue);
            }

            // Listen for DOM changes and update signal
            const handleDomChange = (e) => {
                const newValue = e.target.value;
                // Avoid infinite loops by checking if value actually changed
                if (signal.get() !== newValue) {
                    signal.set(newValue);
                }
            };

            element.addEventListener(eventType, handleDomChange);

            // Subscribe to signal changes and update element
            // We use createEffect for automatic dependency tracking
            const cleanup = reactivity.createEffect(() => {
                const newValue = signal.get();
                // Only update if different to avoid cursor position issues
                if (element.value !== String(newValue !== null && newValue !== undefined ? newValue : '')) {
                    element.value = newValue !== null && newValue !== undefined ? String(newValue) : '';
                }
            });

            // Store cleanup for potential disposal
            // Note: In real usage, this would be tracked per-component
            return { cleanup: () => {
                element.removeEventListener(eventType, handleDomChange);
                cleanup();
            }};
        },

        /**
         * Create a two-way binding between a checkbox/radio's checked state and a signal.
         * Sets up bidirectional synchronization:
         * - Sets initial checked state from signal
         * - Listens for change events to update signal
         * - Subscribes to signal changes to update checked state
         *
         * @param {number} elementHandle - Element handle (checkbox or radio input)
         * @param {number} signalHandle - Signal handle
         */
        bindChecked(elementHandle, signalHandle) {
            const element = elementHandles.get(elementHandle);
            const signal = elementHandles.get(signalHandle);

            if (!element || !signal) {
                console.warn('[RustScript Bindings] Invalid element or signal handle');
                return;
            }

            // Set initial checked state from signal
            const initialValue = signal.get();
            element.checked = Boolean(initialValue);

            // Listen for DOM changes and update signal
            const handleDomChange = (e) => {
                const newValue = e.target.checked;
                if (signal.get() !== newValue) {
                    signal.set(newValue);
                }
            };

            element.addEventListener('change', handleDomChange);

            // Subscribe to signal changes and update element
            const cleanup = reactivity.createEffect(() => {
                const newValue = signal.get();
                if (element.checked !== Boolean(newValue)) {
                    element.checked = Boolean(newValue);
                }
            });

            return { cleanup: () => {
                element.removeEventListener('change', handleDomChange);
                cleanup();
            }};
        },

        /**
         * Create a two-way binding between a select element's value and a signal.
         * Sets up bidirectional synchronization:
         * - Sets initial selected option from signal
         * - Listens for change events to update signal
         * - Subscribes to signal changes to update selected option
         *
         * @param {number} elementHandle - Select element handle
         * @param {number} signalHandle - Signal handle
         */
        bindSelect(elementHandle, signalHandle) {
            const element = elementHandles.get(elementHandle);
            const signal = elementHandles.get(signalHandle);

            if (!element || !signal) {
                console.warn('[RustScript Bindings] Invalid element or signal handle');
                return;
            }

            // Set initial value from signal
            const initialValue = signal.get();
            if (initialValue !== null && initialValue !== undefined) {
                element.value = String(initialValue);
            }

            // Listen for DOM changes and update signal
            const handleDomChange = (e) => {
                const newValue = e.target.value;
                if (signal.get() !== newValue) {
                    signal.set(newValue);
                }
            };

            element.addEventListener('change', handleDomChange);

            // Subscribe to signal changes and update element
            const cleanup = reactivity.createEffect(() => {
                const newValue = signal.get();
                const stringValue = newValue !== null && newValue !== undefined ? String(newValue) : '';
                if (element.value !== stringValue) {
                    element.value = stringValue;
                }
            });

            return { cleanup: () => {
                element.removeEventListener('change', handleDomChange);
                cleanup();
            }};
        }
    };

    // =========================================================================
    // RS-151: Conditional Rendering Interface
    // =========================================================================

    /**
     * Conditional rendering manager for @if/@else directives.
     *
     * This module manages conditional DOM content that can be reactively
     * updated when the condition signal changes. Each conditional block
     * is represented by a placeholder comment node that marks where the
     * content should be inserted.
     *
     * Usage:
     * 1. create() - Creates a placeholder and returns its handle
     * 2. update() - Updates content based on new condition value
     * 3. clear() - Removes all conditional content (for cleanup)
     */
    const conditionals = {
        /**
         * Map of placeholder handle -> conditional instance state.
         * Each instance tracks:
         * - placeholder: The comment node marking the position
         * - currentCondition: The last known condition value
         * - currentContent: Array of nodes currently rendered
         * - parent: The parent element
         */
        instances: new Map(),

        /** Next placeholder handle to allocate */
        nextHandle: 1,

        /**
         * Create a conditional block placeholder.
         * The placeholder is a comment node that marks where conditional
         * content will be inserted/removed.
         *
         * @param {number} parentHandle - Parent element handle (0 for current context)
         * @returns {number} - Placeholder handle
         */
        create(parentHandle) {
            const handle = this.nextHandle++;

            // Get parent element (default to document.body if not specified)
            let parent;
            if (parentHandle && parentHandle !== 0) {
                parent = elementHandles.get(parentHandle);
            }
            if (!parent) {
                parent = document.body;
            }

            // Create placeholder comment node
            const placeholder = document.createComment(`@if ${handle}`);
            parent.appendChild(placeholder);

            // Store instance state
            this.instances.set(handle, {
                placeholder,
                parent,
                currentCondition: null,
                currentContent: [],
                trueFn: null,
                falseFn: null,
                subscribed: false
            });

            // Allocate handle for placeholder
            elementHandles.alloc(placeholder);

            return handle;
        },

        /**
         * Update conditional content based on condition change.
         * Clears existing content and renders the appropriate branch.
         *
         * @param {number} placeholderHandle - Placeholder handle from create()
         * @param {number} condition - Boolean condition value (0 = false, non-0 = true)
         * @param {number} trueFnId - Callback ID for true branch render function
         * @param {number} falseFnId - Callback ID for false branch render function (0 for none)
         */
        update(placeholderHandle, condition, trueFnId, falseFnId) {
            const instance = this.instances.get(placeholderHandle);
            if (!instance) {
                console.warn(`[RustScript Conditionals] Unknown placeholder: ${placeholderHandle}`);
                return;
            }

            const boolCondition = condition !== 0;

            // Store render function IDs
            instance.trueFn = trueFnId;
            instance.falseFn = falseFnId;

            // Only re-render if condition changed
            if (instance.currentCondition === boolCondition && instance.subscribed) {
                return;
            }

            instance.currentCondition = boolCondition;
            instance.subscribed = true;

            // Clear existing content
            this.clearContent(instance);

            // Render appropriate branch
            const renderFnId = boolCondition ? trueFnId : falseFnId;
            if (renderFnId && renderFnId !== 0) {
                this.renderBranch(instance, renderFnId);
            }
        },

        /**
         * Clear all content from a conditional block.
         * Called during component unmount or when removing conditional entirely.
         *
         * @param {number} placeholderHandle - Placeholder handle
         */
        clear(placeholderHandle) {
            const instance = this.instances.get(placeholderHandle);
            if (!instance) {
                return;
            }

            // Remove all rendered content
            this.clearContent(instance);

            // Remove placeholder node
            if (instance.placeholder && instance.placeholder.parentNode) {
                instance.placeholder.parentNode.removeChild(instance.placeholder);
            }

            // Clean up instance
            this.instances.delete(placeholderHandle);
        },

        /**
         * Internal: Clear rendered content for an instance.
         * @param {Object} instance - The conditional instance
         */
        clearContent(instance) {
            for (const node of instance.currentContent) {
                if (node && node.parentNode) {
                    node.parentNode.removeChild(node);
                }
            }
            instance.currentContent = [];
        },

        /**
         * Internal: Render a branch and insert after placeholder.
         * @param {Object} instance - The conditional instance
         * @param {number} renderFnId - The render function callback ID
         */
        renderBranch(instance, renderFnId) {
            // Call the WASM render function via trampoline
            // The render function should return an element handle
            if (wasmInstance && wasmInstance.exports.__rsc_call_render) {
                try {
                    const resultHandle = wasmInstance.exports.__rsc_call_render(renderFnId);
                    const element = elementHandles.get(resultHandle);

                    if (element) {
                        // Insert after placeholder
                        const nextSibling = instance.placeholder.nextSibling;
                        if (nextSibling) {
                            instance.parent.insertBefore(element, nextSibling);
                        } else {
                            instance.parent.appendChild(element);
                        }
                        instance.currentContent.push(element);
                    }
                } catch (e) {
                    console.error('[RustScript Conditionals] Error rendering branch:', e);
                }
            }
        }
    };

    /**
     * Conditional rendering interface for WASM imports.
     * Wraps the conditionals manager for the import interface.
     */
    const conditionalsInterface = {
        /**
         * Create a conditional placeholder.
         * @param {number} parentHandle - Parent element handle
         * @returns {number} - Placeholder handle
         */
        create(parentHandle) {
            return conditionals.create(parentHandle);
        },

        /**
         * Update conditional content.
         * @param {number} placeholder - Placeholder handle
         * @param {number} condition - Condition value
         * @param {number} trueFn - True branch render function ID
         * @param {number} falseFn - False branch render function ID
         */
        update(placeholder, condition, trueFn, falseFn) {
            conditionals.update(placeholder, condition, trueFn, falseFn);
        },

        /**
         * Clear conditional block.
         * @param {number} placeholder - Placeholder handle
         */
        clear(placeholder) {
            conditionals.clear(placeholder);
        }
    };

    // =========================================================================
    // RS-145: Signal Runtime with Dependency Tracking
    // =========================================================================

    /**
     * Signal runtime for fine-grained reactivity.
     *
     * This module provides the low-level signal primitives that the WASM codegen
     * calls into. Signals are reactive values that:
     *
     * 1. Store a value that can be read and updated
     * 2. Track dependencies when read in a reactive context (effect, derived)
     * 3. Notify all subscribers when the value changes
     *
     * The signal system uses integer handles to reference signals from WASM,
     * with the actual signal data stored in JavaScript.
     *
     * Dependency tracking is automatic: when signalGet is called during an
     * effect or derived computation, the signal is automatically registered
     * as a dependency. When signalSet is called, all dependents are notified.
     */
    const signalRuntime = {
        /** Map of signal handle -> signal state */
        signals: new Map(),

        /** Next signal handle to allocate */
        nextHandle: 1,

        /** Current reactive context (effect or derived computation) */
        currentContext: null,

        /** Map of signal handle -> Set of subscriber callbacks */
        subscribers: new Map(),

        /** Pending updates to batch */
        pendingNotifications: new Set(),

        /** Whether we're in a batch */
        batchDepth: 0,

        /**
         * Create a new signal with an initial value.
         *
         * @param {number} initialValue - The initial value (i32 from WASM)
         * @returns {number} - Signal handle (i32)
         *
         * The signal handle is used for all subsequent operations on this signal.
         * The handle remains valid until the signal is explicitly disposed.
         */
        create(initialValue) {
            const handle = this.nextHandle++;

            this.signals.set(handle, {
                value: initialValue,
                // Dependencies: contexts that read this signal
                dependents: new Set()
            });

            this.subscribers.set(handle, new Set());

            return handle;
        },

        /**
         * Get the current value of a signal.
         *
         * If called within a reactive context (effect or derived), this signal
         * is automatically tracked as a dependency of that context.
         *
         * @param {number} handle - Signal handle
         * @returns {number} - Current value (i32)
         */
        get(handle) {
            const signal = this.signals.get(handle);
            if (!signal) {
                console.warn(`[Signal Runtime] Unknown signal handle: ${handle}`);
                return 0;
            }

            // Track dependency if we're in a reactive context
            if (this.currentContext !== null) {
                signal.dependents.add(this.currentContext);

                // Also track in the global reactivity system for effects
                reactivity.track(handle);
            }

            return signal.value;
        },

        /**
         * Set the value of a signal.
         *
         * This triggers all subscribers and dependent effects/computations
         * to re-run. Updates are batched when inside a batch() call.
         *
         * @param {number} handle - Signal handle
         * @param {number} value - New value (i32)
         */
        set(handle, value) {
            const signal = this.signals.get(handle);
            if (!signal) {
                console.warn(`[Signal Runtime] Unknown signal handle: ${handle}`);
                return;
            }

            // Only update if value actually changed
            if (signal.value === value) {
                return;
            }

            const oldValue = signal.value;
            signal.value = value;

            // Notify all subscribers
            const subs = this.subscribers.get(handle);
            if (subs && subs.size > 0) {
                if (this.batchDepth > 0) {
                    // Queue for batch notification
                    for (const callback of subs) {
                        this.pendingNotifications.add(() => {
                            try {
                                callback(value, oldValue);
                            } catch (e) {
                                console.error('[Signal Runtime] Subscriber error:', e);
                            }
                        });
                    }
                } else {
                    // Immediate notification
                    for (const callback of subs) {
                        try {
                            callback(value, oldValue);
                        } catch (e) {
                            console.error('[Signal Runtime] Subscriber error:', e);
                        }
                    }
                }
            }

            // Trigger dependent effects via the reactivity system
            reactivity.trigger(handle);
        },

        /**
         * Subscribe to signal changes.
         *
         * The callback is called with (newValue, oldValue) whenever the signal
         * changes. Returns a subscription ID that can be used to unsubscribe.
         *
         * @param {number} handle - Signal handle
         * @param {number|Function} callbackPtr - WASM function pointer or JS callback
         * @returns {number} - Subscription ID
         */
        subscribe(handle, callbackPtr) {
            const signal = this.signals.get(handle);
            if (!signal) {
                console.warn(`[Signal Runtime] Unknown signal handle: ${handle}`);
                return 0;
            }

            // Create callback wrapper for WASM function pointers
            let callback;
            if (typeof callbackPtr === 'function') {
                callback = callbackPtr;
            } else {
                // WASM function pointer - call through trampoline
                callback = (newValue, oldValue) => {
                    if (wasmInstance && wasmInstance.exports.__rsc_call_signal_subscriber) {
                        wasmInstance.exports.__rsc_call_signal_subscriber(callbackPtr, newValue, oldValue);
                    }
                };
            }

            // Store with a unique subscription ID
            const subscriptionId = this.nextHandle++;
            callback._subscriptionId = subscriptionId;

            const subs = this.subscribers.get(handle);
            if (subs) {
                subs.add(callback);
            }

            // Return subscription ID for unsubscribe
            return subscriptionId;
        },

        /**
         * Unsubscribe from a signal.
         *
         * @param {number} handle - Signal handle
         * @param {number} subscriptionId - Subscription ID from subscribe()
         */
        unsubscribe(handle, subscriptionId) {
            const subs = this.subscribers.get(handle);
            if (subs) {
                for (const callback of subs) {
                    if (callback._subscriptionId === subscriptionId) {
                        subs.delete(callback);
                        break;
                    }
                }
            }
        },

        /**
         * Start a batch of updates.
         *
         * Updates are deferred until endBatch() is called. This allows multiple
         * signal changes to be batched together, avoiding cascading updates.
         */
        startBatch() {
            this.batchDepth++;
        },

        /**
         * End a batch of updates.
         *
         * If this is the outermost batch, all pending notifications are flushed.
         */
        endBatch() {
            this.batchDepth--;

            if (this.batchDepth === 0 && this.pendingNotifications.size > 0) {
                const notifications = Array.from(this.pendingNotifications);
                this.pendingNotifications.clear();

                for (const notify of notifications) {
                    notify();
                }
            }
        },

        /**
         * Run a function in a reactive context.
         *
         * Any signals read during the function execution will be tracked as
         * dependencies of the provided updater function.
         *
         * @param {Function} fn - Function to run
         * @param {Function} updater - The updater to register for dependencies
         * @returns {any} - Return value of fn
         */
        runInContext(fn, updater) {
            const prevContext = this.currentContext;
            this.currentContext = updater;

            try {
                return fn();
            } finally {
                this.currentContext = prevContext;
            }
        },

        /**
         * Dispose a signal and clean up all its resources.
         *
         * @param {number} handle - Signal handle
         */
        dispose(handle) {
            this.signals.delete(handle);
            this.subscribers.delete(handle);
        }
    };

    /**
     * Signal interface for WASM imports (rustscript:web/signals).
     *
     * These functions are the actual imports called by the WASM code.
     * They provide a simple i32-based API for signal operations.
     */
    const signals = {
        /**
         * Create a signal with an initial value.
         * @param {number} initialValue - Initial value (i32)
         * @returns {number} - Signal handle (i32)
         */
        signalCreate(initialValue) {
            return signalRuntime.create(initialValue);
        },

        /**
         * Get a signal's current value.
         * Also tracks the signal as a dependency if in reactive context.
         * @param {number} handle - Signal handle
         * @returns {number} - Current value (i32)
         */
        signalGet(handle) {
            return signalRuntime.get(handle);
        },

        /**
         * Set a signal's value and notify subscribers.
         * @param {number} handle - Signal handle
         * @param {number} value - New value (i32)
         */
        signalSet(handle, value) {
            signalRuntime.set(handle, value);
        },

        /**
         * Subscribe to signal changes.
         * @param {number} handle - Signal handle
         * @param {number} callbackPtr - WASM function pointer for callback
         * @returns {number} - Subscription ID
         */
        signalSubscribe(handle, callbackPtr) {
            return signalRuntime.subscribe(handle, callbackPtr);
        }
    };

    // =========================================================================
    // List Rendering Runtime (RS-152)
    // =========================================================================

    /**
     * List rendering manager with keyed diffing support.
     *
     * Implements efficient list reconciliation using keys to:
     * - Reuse existing DOM elements for items with matching keys
     * - Minimize DOM operations during list updates
     * - Handle insertions, deletions, and reordering efficiently
     *
     * The algorithm uses a key-based approach similar to React/Vue:
     * 1. Build a map of existing keys to their elements
     * 2. Process new items, reusing elements when keys match
     * 3. Remove elements for keys no longer present
     * 4. Reorder elements to match the new item order
     */
    const lists = {
        /** Map of placeholder handle -> list instance state */
        instances: new Map(),

        /** Next available placeholder handle */
        nextHandle: 1,

        /**
         * Create a list block with a placeholder marker.
         *
         * @param {Element|number} parentElement - Parent DOM element or handle
         * @returns {number} - Placeholder handle for the list
         */
        create(parentElement) {
            // Resolve parent element
            const parent = typeof parentElement === 'number'
                ? elementHandles.get(parentElement)
                : parentElement;

            if (!parent && parentElement !== 0) {
                console.warn('List create: parent element not found');
            }

            // Create a comment node as placeholder/marker
            const placeholder = document.createComment('list-placeholder');

            // Append placeholder to parent (or document body if no parent)
            const actualParent = parent || document.body;
            actualParent.appendChild(placeholder);

            // Allocate handle
            const handle = this.nextHandle++;

            // Initialize list instance state
            this.instances.set(handle, {
                placeholder: placeholder,
                parent: actualParent,
                items: [],
                keys: [],
                elements: new Map(), // key -> element
                renderFn: null,
                keyFn: null
            });

            // Register placeholder in element handles for WASM reference
            return elementHandles.alloc(placeholder);
        },

        /**
         * Update list with new items using keyed diffing.
         *
         * This is the core reconciliation algorithm:
         * 1. Build map of old key -> element
         * 2. For each new item:
         *    - If key exists in old map: reuse element, update content
         *    - If key is new: create new element
         * 3. Remove elements for deleted keys
         * 4. Reorder elements to match new order
         *
         * @param {number} placeholderHandle - List placeholder handle
         * @param {number} itemsPtr - Pointer to items array in WASM memory
         * @param {number} itemsLen - Length of items array
         * @param {number} keyFnPtr - Callback ID for key function (0 for index-based)
         * @param {number} renderFnPtr - Callback ID for item render function
         */
        update(placeholderHandle, itemsPtr, itemsLen, keyFnPtr, renderFnPtr) {
            const placeholder = elementHandles.get(placeholderHandle);
            if (!placeholder) {
                console.warn('List update: placeholder not found');
                return;
            }

            // Find the list instance
            let instance = null;
            for (const [handle, inst] of this.instances) {
                if (inst.placeholder === placeholder || elementHandles.get(handle) === placeholder) {
                    instance = inst;
                    break;
                }
            }

            if (!instance) {
                console.warn('List update: instance not found for placeholder');
                return;
            }

            // Deserialize items from WASM memory
            // For now, this is a simplified implementation
            // Real implementation would read from itemsPtr/itemsLen
            const newItems = this._deserializeItems(itemsPtr, itemsLen);

            // Get keys for new items
            const newKeys = newItems.map((item, index) => {
                if (keyFnPtr && keyFnPtr !== 0) {
                    // Call key function via trampoline
                    return this._callKeyFn(keyFnPtr, item, index);
                }
                // Default to index-based keys
                return String(index);
            });

            // Build map of old keys to elements
            const oldKeyToElement = new Map(instance.elements);

            // Track which elements we've processed
            const processedKeys = new Set();

            // Array to hold elements in new order
            const newElements = [];

            // Process each new item
            for (let i = 0; i < newItems.length; i++) {
                const item = newItems[i];
                const key = newKeys[i];

                processedKeys.add(key);

                if (oldKeyToElement.has(key)) {
                    // Reuse existing element
                    const element = oldKeyToElement.get(key);

                    // Update element content if needed
                    if (renderFnPtr && renderFnPtr !== 0) {
                        this._updateElement(element, item, i, renderFnPtr);
                    }

                    newElements.push({ key, element });
                } else {
                    // Create new element
                    const element = this._createElement(item, i, renderFnPtr);
                    newElements.push({ key, element });
                }
            }

            // Remove elements for deleted keys
            for (const [key, element] of oldKeyToElement) {
                if (!processedKeys.has(key)) {
                    // Key no longer exists - remove element
                    if (element && element.parentNode) {
                        element.parentNode.removeChild(element);
                    }
                }
            }

            // Reorder elements in DOM
            this._reorderElements(instance.parent, instance.placeholder, newElements);

            // Update instance state
            instance.items = newItems;
            instance.keys = newKeys;
            instance.elements = new Map(newElements.map(e => [e.key, e.element]));
        },

        /**
         * Clear all items from a list.
         *
         * @param {number} placeholderHandle - List placeholder handle
         */
        clear(placeholderHandle) {
            const placeholder = elementHandles.get(placeholderHandle);
            if (!placeholder) {
                return;
            }

            // Find the list instance
            for (const [handle, instance] of this.instances) {
                if (instance.placeholder === placeholder || elementHandles.get(handle) === placeholder) {
                    // Remove all elements
                    for (const element of instance.elements.values()) {
                        if (element && element.parentNode) {
                            element.parentNode.removeChild(element);
                        }
                    }

                    // Reset instance state
                    instance.items = [];
                    instance.keys = [];
                    instance.elements = new Map();
                    break;
                }
            }
        },

        /**
         * Deserialize items from WASM memory.
         * @private
         */
        _deserializeItems(ptr, len) {
            if (!ptr || len === 0) {
                return [];
            }

            // For now, return empty array
            // Real implementation would read typed data from WASM memory
            try {
                if (wasmMemory) {
                    const view = new Uint32Array(wasmMemory.buffer, ptr, len);
                    return Array.from(view);
                }
            } catch (e) {
                console.warn('Failed to deserialize items:', e);
            }

            return [];
        },

        /**
         * Call key function via WASM trampoline.
         * @private
         */
        _callKeyFn(keyFnPtr, item, index) {
            // For now, use index as key
            // Real implementation would call into WASM
            if (wasmInstance && wasmInstance.exports.__rsc_call_key_fn) {
                try {
                    return String(wasmInstance.exports.__rsc_call_key_fn(keyFnPtr, item, index));
                } catch (e) {
                    console.warn('Key function error:', e);
                }
            }
            return String(index);
        },

        /**
         * Create a new element for an item.
         * @private
         */
        _createElement(item, index, renderFnPtr) {
            // Create wrapper div for the item
            const element = document.createElement('div');
            element.setAttribute('data-list-item', String(index));

            // Call render function if available
            if (renderFnPtr && renderFnPtr !== 0 && wasmInstance) {
                if (wasmInstance.exports.__rsc_call_render_item) {
                    try {
                        const contentHandle = wasmInstance.exports.__rsc_call_render_item(
                            renderFnPtr, item, index
                        );
                        const content = elementHandles.get(contentHandle);
                        if (content) {
                            element.appendChild(content);
                        }
                    } catch (e) {
                        console.warn('Render item error:', e);
                        element.textContent = String(item);
                    }
                } else {
                    element.textContent = String(item);
                }
            } else {
                element.textContent = String(item);
            }

            return element;
        },

        /**
         * Update an existing element's content.
         * @private
         */
        _updateElement(element, item, index, renderFnPtr) {
            element.setAttribute('data-list-item', String(index));

            // For now, just update text content
            // Real implementation would do more sophisticated updates
            if (!renderFnPtr || renderFnPtr === 0) {
                element.textContent = String(item);
            }
            // If render function is provided, trust that signals will update content
        },

        /**
         * Reorder elements in the DOM to match the new order.
         * @private
         */
        _reorderElements(parent, placeholder, newElements) {
            // Insert elements after the placeholder in order
            let insertBefore = placeholder.nextSibling;

            for (const { element } of newElements) {
                if (!element) continue;

                if (element.nextSibling !== insertBefore) {
                    // Element needs to be moved
                    if (insertBefore) {
                        parent.insertBefore(element, insertBefore);
                    } else {
                        parent.appendChild(element);
                    }
                }

                // Update insert position
                insertBefore = element.nextSibling;
            }
        }
    };

    // =========================================================================
    // Effects Runtime
    // =========================================================================

    /**
     * Effect cleanup interface for WASM.
     * Provides functions to cleanup individual effects and dispose all effects
     * for a component on unmount.
     */
    const effects = {
        /**
         * Cleanup a single effect by its ID.
         * Runs the effect's cleanup function if any and removes it from tracking.
         * @param {number} effectId - Effect ID to cleanup
         */
        cleanup(effectId) {
            effectSystem.cleanupEffect(effectId);
        },

        /**
         * Dispose all effects for a component (called on unmount).
         * Cleans up all effects associated with the given component ID.
         * @param {number} componentId - Component ID
         */
        disposeAll(componentId) {
            effectSystem.disposeComponentEffects(componentId);
        }
    };

    // =========================================================================
    // Type Conversion Runtime
    // =========================================================================

    /**
     * Type conversion utilities for WASM <-> JS interop.
     * Provides functions to convert WASM primitive types to strings.
     */
    const convert = {
        /**
         * Convert an i32 value to a string and return a handle.
         * @param {number} value - The i32 value to convert
         * @returns {number} - Handle to the string in element handles
         */
        i32ToString(value) {
            const str = String(value);
            // Create a text node to hold the string, then return its handle
            const textNode = document.createTextNode(str);
            return elementHandles.alloc(textNode);
        },

        /**
         * Convert an f64 value to a string and return a handle.
         * @param {number} value - The f64 value to convert
         * @returns {number} - Handle to the string in element handles
         */
        f64ToString(value) {
            const str = String(value);
            // Create a text node to hold the string, then return its handle
            const textNode = document.createTextNode(str);
            return elementHandles.alloc(textNode);
        },

        /**
         * Compare two strings for equality.
         * Strings are represented as handles that encode (offset << 16) | len.
         * @param {number} handle1 - First string handle
         * @param {number} handle2 - Second string handle
         * @returns {number} - 1 if strings are equal, 0 otherwise
         */
        stringEq(handle1, handle2) {
            // Fast path: same handle means equal
            if (handle1 === handle2) {
                return 1;
            }
            // Decode handles and compare strings
            const str1 = decodeStringHandle(handle1);
            const str2 = decodeStringHandle(handle2);
            return str1 === str2 ? 1 : 0;
        }
    };

    /**
     * Create the import object for WASM instantiation.
     * @returns {Object} - Import object
     */
    function createImports() {
        return {
            'rustscript:web/dom': dom,
            'rustscript:web/lists': lists,
            'rustscript:web/conditionals': conditionalsInterface,
            'rustscript:web/events': events,
            'rustscript:web/console': consoleApi,
            'rustscript:web/timers': timers,
            'rustscript:web/storage': storage,
            'rustscript:web/http': http,
            'rustscript:router/router': router,
            'rustscript:web/lifecycle': lifecycle,
            'rustscript:web/components': components,
            'rustscript:web/reactivity': reactivityInterface,
            'rustscript:web/bindings': bindings,
            'rustscript:web/signals': signals,
            'rustscript:web/convert': convert,
            'rustscript:web/effects': effects,
        };
    }

    /**
     * Initialize the RustScript runtime.
     * @param {WebAssembly.Instance} instance - WASM instance
     * @param {WebAssembly.Memory} memory - WASM memory
     */
    function initialize(instance, memory) {
        wasmInstance = instance;
        wasmMemory = memory;

        // Call WASM init if available
        if (instance.exports.init) {
            instance.exports.init();
        }
    }

    /**
     * Load and run a RustScript application.
     * @param {string|ArrayBuffer} wasmSource - WASM file path or ArrayBuffer
     * @param {string} rootId - Root element ID to mount
     * @returns {Promise<Object>} - App instance
     */
    async function loadApp(wasmSource, rootId = 'app') {
        let wasmBytes;

        if (typeof wasmSource === 'string') {
            const response = await fetch(wasmSource);
            wasmBytes = await response.arrayBuffer();
        } else {
            wasmBytes = wasmSource;
        }

        const imports = createImports();
        const { instance } = await WebAssembly.instantiate(wasmBytes, imports);

        // Get memory from exports
        const memory = instance.exports.memory;

        // Initialize runtime
        initialize(instance, memory);

        // Mount app - write rootId to WASM memory and call mount(ptr, len)
        if (instance.exports.mount) {
            // Use a scratch buffer area at the end of the data section
            // We use offset 65536 (64KB) as a safe location for argument passing
            // This is after the typical data section but within the first memory page
            const SCRATCH_BUFFER_OFFSET = 65536;
            const encoder = new TextEncoder();
            const rootIdBytes = encoder.encode(rootId);
            const rootIdPtr = SCRATCH_BUFFER_OFFSET;
            const rootIdLen = rootIdBytes.length;

            // Write the rootId string to WASM memory
            const view = new Uint8Array(memory.buffer, rootIdPtr, rootIdLen);
            view.set(rootIdBytes);

            // Call mount with ptr and len
            const result = instance.exports.mount(rootIdPtr, rootIdLen);
            if (result !== 0) {
                throw new Error(`Failed to mount app: error code ${result}`);
            }
        }

        return {
            instance,
            memory,
            unmount: () => {
                if (instance.exports.unmount) {
                    instance.exports.unmount();
                }
            }
        };
    }

    // =========================================================================
    // Exports
    // =========================================================================

    const RustScript = {
        // Core functions
        createImports,
        initialize,
        loadApp,

        // Handle managers (for advanced use)
        elementHandles,
        eventHandlerHandles,
        timerHandles,

        // Interface implementations (for testing/mocking)
        dom,
        events,
        console: consoleApi,
        timers,
        storage,
        http,
        router,
        lifecycle,
        components,

        // Component management (RS-150)
        componentManager,

        // Error boundaries (RS-154)
        errorBoundary,

        // Reactive re-rendering (RS-147)
        reactivity,
        vdom,
        reactiveDOM,
        reactivityInterface,

        // Effect system (RS-148)
        effectSystem,

        // Two-way bindings (RS-153)
        bindings,

        // Signal runtime (RS-145)
        signalRuntime,
        signals,

        // List rendering (RS-152)
        lists,

        // Conditional rendering (RS-151)
        conditionals,
        conditionalsInterface,
    };

    // Export to global scope
    if (typeof module !== 'undefined' && module.exports) {
        module.exports = RustScript;
    } else {
        global.RustScript = RustScript;
    }

})(typeof globalThis !== 'undefined' ? globalThis : typeof window !== 'undefined' ? window : this);
