import init, { generate_magic_square } from "./pkg/magic_squares.js";

async function run() {
    // Initialize WASM
    await init();

    const orderInput = document.getElementById('order-n');
    const generateBtn = document.getElementById('generate-btn');
    const gridContainer = document.getElementById('grid-container');
    const statsContainer = document.getElementById('stats');
    const magicConstantValue = document.getElementById('magic-constant');

    const MAX_ORDER = 100;

    generateBtn.addEventListener('click', async () => {
        const n = parseInt(orderInput.value);

        // Validation
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

        // UI State: Loading
        generateBtn.disabled = true;
        generateBtn.innerText = "Generating...";
        gridContainer.innerHTML = '<p class="placeholder-text">Generating magic...</p>';
        statsContainer.classList.add('hidden');

        // Wrap in setTimeout to allow UI to render the "Generating..." state
        setTimeout(() => {
            try {
                const result = generate_magic_square(n);
                
                if (result) {
                    renderGrid(result.grid, result.n);
                    
                    const constant = (BigInt(result.n) * (BigInt(result.n) * BigInt(result.n) + 1n)) / 2n;
                    magicConstantValue.innerText = constant.toString();
                    statsContainer.classList.remove('hidden');
                } else {
                    gridContainer.innerHTML = '<p class="error-text">Failed to generate magic square.</p>';
                }
            } catch (error) {
                console.error("Error generating square:", error);
                if (error instanceof RangeError || error.message.includes("out of memory")) {
                    alert("Out of memory: The requested order is too large for the browser to handle.");
                    gridContainer.innerHTML = '<p class="error-text">Out of memory error.</p>';
                } else {
                    alert("An unexpected error occurred during generation.");
                    gridContainer.innerHTML = '<p class="error-text">An error occurred.</p>';
                }
            } finally {
                generateBtn.disabled = false;
                generateBtn.innerText = "Generate Square";
            }
        }, 50);
    });

    function renderGrid(flatGrid, n) {
        gridContainer.innerHTML = '';
        const grid = document.createElement('div');
        grid.className = 'magic-grid';
        
        // Calculate cell sizes
        const cellSize = n > 15 ? (n > 30 ? (n > 60 ? '20px' : '30px') : '40px') : '50px';
        const fontSize = n > 15 ? (n > 30 ? (n > 60 ? '0.5rem' : '0.7rem') : '0.9rem') : '1.1rem';

        // Use the calculated cellSize for the grid layout to ensure consistency
        grid.style.gridTemplateColumns = `repeat(${n}, ${cellSize})`;
        grid.style.gridTemplateRows = `repeat(${n}, ${cellSize})`;

        flatGrid.forEach(val => {
            const cell = document.createElement('div');
            cell.className = 'grid-cell';
            cell.innerText = val;
            cell.style.width = cellSize;
            cell.style.height = cellSize;
            cell.style.fontSize = fontSize;
            grid.appendChild(cell);
        });

        gridContainer.appendChild(grid);
    }
}

run();
