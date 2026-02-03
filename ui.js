import init, { generate_magic_square } from "./pkg/magic_squares.js";

/**
 * Main application entry point.
 * Initializes the WASM module and sets up event listeners.
 */
async function run() {
    // Initialize the WASM module (fetches and compiles magic_squares_bg.wasm)
    await init();

    const orderInput = document.getElementById('order-n');
    const generateBtn = document.getElementById('generate-btn');
    const gridContainer = document.getElementById('grid-container');
    const statsContainer = document.getElementById('stats');
    const magicConstantValue = document.getElementById('magic-constant');

    // Limit the maximum order to prevent browser hanging.
    const MAX_ORDER = 100;

    // --- Keyboard Accessibility ---
    orderInput.addEventListener('keydown', (event) => {
        if (event.key === 'Enter') {
            event.preventDefault(); // Prevent accidental form submission if wrapped in one
            generateBtn.click();
        }
    });

    /**
     * Event listener for the "Generate Square" button.
     * Handles input validation, UI state calls, and invokes the WASM generator.
     */
    generateBtn.addEventListener('click', async () => {
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

                // If we get here, generation was successful!
                renderGrid(result.grid, result.n);

                // Calculate Magic Constant: M = n * (n^2 + 1) / 2
                // Use BigInt to avoid overflow for large n.
                const constant = (BigInt(result.n) * (BigInt(result.n) * BigInt(result.n) + 1n)) / 2n;
                magicConstantValue.innerText = constant.toString();
                statsContainer.classList.remove('hidden');

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
        const grid = document.createElement('div');
        grid.className = 'magic-grid';

        // The largest number in an n×n magic square is n²
        const maxNumber = n * n;
        const digitCount = maxNumber.toString().length;

        // Calculate cell size based on digit count to fit the largest number
        // Base sizes adjusted for digit count
        let cellSize, fontSize;

        if (digitCount <= 2) {
            // Numbers up to 99 (n ≤ 9)
            cellSize = '50px';
            fontSize = '1.1rem';
        } else if (digitCount === 3) {
            // Numbers up to 999 (n ≤ 31)
            cellSize = n > 20 ? '45px' : '50px';
            fontSize = n > 20 ? '1rem' : '1.1rem';
        } else if (digitCount === 4) {
            // Numbers up to 9999 (n ≤ 99)
            cellSize = n > 70 ? '38px' : (n > 50 ? '40px' : '45px');
            fontSize = n > 70 ? '0.75rem' : (n > 50 ? '0.85rem' : '0.95rem');
        } else if (digitCount === 5) {
            // Numbers up to 99999 (n ≤ 316)
            cellSize = '40px';
            fontSize = '0.65rem';
        } else {
            // 6+ digits
            cellSize = '45px';
            fontSize = '0.5rem';
        }

        // Use CSS Grid for layout - all cells same size
        grid.style.gridTemplateColumns = `repeat(${n}, ${cellSize})`;
        grid.style.gridTemplateRows = `repeat(${n}, ${cellSize})`;

        flatGrid.forEach(val => {
            const cell = document.createElement('div');
            cell.className = 'grid-cell';
            cell.innerText = val;
            // All cells get the same size
            cell.style.width = cellSize;
            cell.style.height = cellSize;
            cell.style.fontSize = fontSize;
            grid.appendChild(cell);
        });

        gridContainer.appendChild(grid);
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
