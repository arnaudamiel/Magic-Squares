import init, { generate_magic_square } from "./pkg/magic_squares.js";

/**
 * Main application entry point.
 * Initializes the WASM module and sets up event listeners.
 */
async function run() {
    // Initialize the WASM module and capture the exports (which includes memory)
    // init() returns the module instance, which we need to access 'memory'
    const wasm = await init();

    const orderInput = document.getElementById('order-n');
    const generateBtn = document.getElementById('generate-btn');
    const gridContainer = document.getElementById('grid-container');
    const statsContainer = document.getElementById('stats');
    const magicConstantValue = document.getElementById('magic-constant');

    // Limit the maximum order to prevent browser hanging.
    const MAX_ORDER = 100;

    const magicForm = document.getElementById('magic-form');

    // --- Form Submission (Handles Enter Key automatically) ---
    magicForm.addEventListener('submit', async (event) => {
        event.preventDefault();

        const n = parseInt(orderInput.value);

        // --- Input Validation ---
        if (isNaN(n) || n < 1) {
            alert("Please enter a positive integer for the order.");
            return;
        }

        if (n === 2) {
            alert("Order 2 magic squares are mathematically impossible.");
            return;
        }

        if (n > MAX_ORDER) {
            if (!confirm(`Orders larger than ${MAX_ORDER} may cause performance issues or memory errors. Proceed?`)) {
                return;
            }
        }

        // --- UI State: Loading ---
        generateBtn.disabled = true;
        generateBtn.innerText = "Generating...";
        gridContainer.innerHTML = '<p class="placeholder-text">Generating magic...</p>';
        statsContainer.classList.add('hidden');

        // Wrap in setTimeout to allow the browser to paint the "Generating" state
        // before blocking the main thread with heavy WASM computation.
        setTimeout(() => {
            try {
                // Call WASM function
                // The Rust function now returns Result<MagicSquareResult, JsError>.
                // In JS, this means it will either return the object or THROW an error.
                const result = generate_magic_square(n);

                // --- ZERO-COPY MEMORY ACCESS ---
                // 1. Get raw pointer and length from WASM
                const gridPtr = result.get_grid_ptr();
                const gridLen = result.get_grid_len();

                // 2. Create a view into WASM memory
                // IMPORTANT: This view is only valid until the next WASM allocation (wasm.memory.grow).
                // Since we render immediately, this is safe.
                const wasmMemoryBuffer = wasm.memory.buffer;
                const flatGrid = new Uint32Array(wasmMemoryBuffer, gridPtr, gridLen);

                // If we get here, generation was successful!
                // Pass the view (not a copy!) to the renderer
                renderGrid(flatGrid, result.n);

                // Calculate Magic Constant: M = n * (n^2 + 1) / 2
                // Use BigInt to avoid overflow for large n.
                const constant = (BigInt(result.n) * (BigInt(result.n) * BigInt(result.n) + 1n)) / 2n;
                magicConstantValue.innerText = constant.toString();
                statsContainer.classList.remove('hidden');

                // Important: We must keep 'result' alive if we need the data later, 
                // but since we rendered it, we can let it be garbage collected by JS 
                // (and its partial rust destructor called). 
                // The actual Vec<u32> inside Rust will be dropped when 'result.free()' is called 
                // or if we add a .free() method. wasm-bindgen handles struct memory.
                result.free(); // Manually free the struct wrapper to clean up WASM side

            } catch (error) {
                console.error("Error generating square:", error);

                // The error thrown from Rust will be a generic Error object with the message we constructed.
                // We display that message directly to the user.
                let errorMessage = "An unexpected error occurred.";

                // Check if it's an error from our Rust code (which comes as an Error object/string)
                if (typeof error === 'string') {
                    errorMessage = error;
                } else if (error.message) {
                    errorMessage = error.message;
                }

                alert(`Generation Failed:\n${errorMessage}`);
                gridContainer.innerHTML = `<p class="error-text">${errorMessage}</p>`;
            } finally {
                // Restore UI State
                generateBtn.disabled = false;
                generateBtn.innerText = "Generate Square";
            }
        }, 50);
    });

    /**
     * Renders the flattened magic square grid into the DOM.
     * @param {Uint32Array} flatGrid - The 1D array representing the square.
     * @param {number} n - The order of the square.
     */
    function renderGrid(flatGrid, n) {
        gridContainer.innerHTML = '';

        // --- Sizing Logic ---
        const maxNumber = n * n;
        const maxNumberStr = maxNumber.toString();

        // Base configuration
        const baseFontSize = 18;
        const padding = 16; // 8px on each side
        const minCellSize = 40;

        // Measure text width to determine optimal cell size
        // We use a temporary 1x1 canvas to measure text
        const tempCanvas = document.createElement('canvas');
        const tempCtx = tempCanvas.getContext('2d');
        tempCtx.font = `700 ${baseFontSize}px "Inter", sans-serif`;
        const textMetrics = tempCtx.measureText(maxNumberStr);
        const textWidth = textMetrics.width;

        // Calculate cell size: Must fit text + padding, but at least minCellSize
        // We round up to avoid sub-pixel rendering issues
        let cellSize = Math.ceil(Math.max(textWidth + padding, minCellSize));
        let fontSize = baseFontSize;

        const totalSize = n * cellSize;

        // --- Virtual Scrolling Setup ---

        // 1. Spacer: Forces the browser to show scrollbars for the full size
        const spacer = document.createElement('div');
        spacer.style.width = `${totalSize}px`;
        spacer.style.height = `${totalSize}px`;
        spacer.style.position = 'absolute';
        spacer.style.top = '0';
        spacer.style.left = '0';
        spacer.style.zIndex = '0';

        // 2. Canvas: The viewport that stays visible
        const canvas = document.createElement('canvas');
        canvas.className = 'magic-canvas';
        // Make canvas sticky so it stays in view while scrolling
        canvas.style.position = 'sticky';
        canvas.style.top = '0';
        canvas.style.left = '0';
        canvas.style.zIndex = '1';

        gridContainer.appendChild(spacer);
        gridContainer.appendChild(canvas);

        // Optimize for no transparency
        const ctx = canvas.getContext('2d', { alpha: false });

        // High DPI Support
        const dpr = window.devicePixelRatio || 1;

        // State
        let hoveredCellIndex = -1;
        let gridOffsetX = 0; // Offset for centering
        let gridOffsetY = 0; // Offset for centering

        // Resize Handler
        function updateCanvasSize() {
            // Adjust for Client Rect including scrollbars, use clientWidth/Height of container
            const width = gridContainer.clientWidth;
            const height = gridContainer.clientHeight;

            // Limit canvas to totalSize if container is bigger (e.g. small n)
            const displayWidth = Math.min(width, totalSize);
            const displayHeight = Math.min(height, totalSize);

            // Calculate centering offsets if the grid is smaller than the container
            gridOffsetX = width > totalSize ? Math.floor((width - totalSize) / 2) : 0;
            gridOffsetY = height > totalSize ? Math.floor((height - totalSize) / 2) : 0;

            // Apply offsets via margins to center the whole unit
            // This works better with sticky/absolute positioning interaction
            canvas.style.marginLeft = `${gridOffsetX}px`;
            canvas.style.marginTop = `${gridOffsetY}px`;

            // Note: Since spacer is absolute, margins might not center it relative to flow.
            // But since container is block and relative, absolute positioning is relative to padding box.
            // We set spacer left/top to match margins.
            spacer.style.left = `${gridOffsetX}px`;
            spacer.style.top = `${gridOffsetY}px`;

            canvas.style.width = `${displayWidth}px`;
            canvas.style.height = `${displayHeight}px`;
            canvas.width = displayWidth * dpr;
            canvas.height = displayHeight * dpr;

            ctx.scale(dpr, dpr);

            requestAnimationFrame(draw);
        }

        // Render Function
        function draw() {
            // Get scroll position
            const scrollLeft = gridContainer.scrollLeft;
            const scrollTop = gridContainer.scrollTop;

            // Determine visible range
            const displayWidth = canvas.width / dpr;
            const displayHeight = canvas.height / dpr;

            // Add buffer of 1 cell to prevent clipping at edges
            const startCol = Math.floor(scrollLeft / cellSize);
            const endCol = Math.min(n, Math.ceil((scrollLeft + displayWidth) / cellSize));

            const startRow = Math.floor(scrollTop / cellSize);
            const endRow = Math.min(n, Math.ceil((scrollTop + displayHeight) / cellSize));

            // Clear
            ctx.fillStyle = '#1e293b'; // var(--card-bg)
            ctx.fillRect(0, 0, displayWidth, displayHeight);

            ctx.textAlign = 'center';
            ctx.textBaseline = 'middle';
            ctx.font = `700 ${fontSize}px "Inter", sans-serif`;

            for (let r = startRow; r < endRow; r++) {
                for (let c = startCol; c < endCol; c++) {
                    const idx = r * n + c;
                    const val = flatGrid[idx];

                    // Positions relative to canvas (viewport)
                    // We must subtract scroll offset because canvas is sticky/fixed relative to flow
                    const x = (c * cellSize) - scrollLeft;
                    const y = (r * cellSize) - scrollTop;

                    // Draw Cell Background (Hover Check)
                    if (idx === hoveredCellIndex) {
                        ctx.fillStyle = '#334155'; // var(--border-color)
                        ctx.fillRect(x, y, cellSize, cellSize);

                        // Draw Hover Border
                        ctx.lineWidth = 2; // thicker border on hover
                        ctx.strokeStyle = '#38bdf8'; // var(--accent-color)
                        // Inset border to avoid clipping
                        ctx.strokeRect(x + 1.5, y + 1.5, cellSize - 3, cellSize - 3);
                    } else {
                        // Regular border
                        ctx.strokeStyle = '#334155'; // var(--border-color)
                        ctx.lineWidth = 1;
                        // Use cellSize - 1 to ensure right/bottom borders are drawn inside the pixel grid
                        ctx.strokeRect(x + 0.5, y + 0.5, cellSize - 1, cellSize - 1);
                    }

                    // Draw Text
                    ctx.fillStyle = '#f8fafc'; // var(--text-primary)
                    ctx.fillText(val.toString(), x + cellSize / 2, y + cellSize / 2);
                }
            }
        }

        // --- Interaction Handlers ---

        // Scroll & Resize
        gridContainer.onscroll = () => requestAnimationFrame(draw);
        window.addEventListener('resize', updateCanvasSize);

        // Mouse Move (Hover)
        canvas.addEventListener('mousemove', (e) => {
            const rect = canvas.getBoundingClientRect();
            const mouseX = e.clientX - rect.left;
            const mouseY = e.clientY - rect.top;

            // Add scroll offset to find "virtual" coordinates
            // Note: Since margins move the element, bounding client rect moves too.
            // But visually, the mouse is relative to the canvas.
            // So we DO add scrollLeft effectively to map canvas-relative coord to world coord.
            const scrollLeft = gridContainer.scrollLeft;
            const scrollTop = gridContainer.scrollTop;

            const virtualX = mouseX + scrollLeft;
            const virtualY = mouseY + scrollTop;

            const col = Math.floor(virtualX / cellSize);
            const row = Math.floor(virtualY / cellSize);

            if (col >= 0 && col < n && row >= 0 && row < n) {
                const newIndex = row * n + col;
                if (newIndex !== hoveredCellIndex) {
                    hoveredCellIndex = newIndex;
                    requestAnimationFrame(draw);
                }
            } else {
                if (hoveredCellIndex !== -1) {
                    hoveredCellIndex = -1;
                    requestAnimationFrame(draw);
                }
            }
        });

        canvas.addEventListener('mouseleave', () => {
            if (hoveredCellIndex !== -1) {
                hoveredCellIndex = -1;
                requestAnimationFrame(draw);
            }
        });

        // Initial Layout
        updateCanvasSize();
    }
}

// Register Service Worker for PWA
if ('serviceWorker' in navigator) {
    window.addEventListener('load', () => {
        navigator.serviceWorker.register('./sw.js')
            .then((registration) => {
                console.log('Service Worker registered successfully:', registration.scope);
            })
            .catch((error) => {
                console.log('Service Worker registration failed:', error);
            });
    });
}

run();
