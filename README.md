# Magic Square Generator

A high-performance Magic Square generator built with Rust and WebAssembly (WASM). This project provides both a command-line interface (CLI) and a modern, responsive web application.

## 🌟 Features

- **Blazing Fast Generation**: Powered by Rust's performance and WASM for near-native speed in the browser.
- **Support for All Orders**:
    - **Odd Orders**: Implemented using the Siamese (De La Loubere) method optimizations.
    - **Singly Even Orders**: Implemented using the LUX method (Conway's method).
    - **Doubly Even Orders**: Implemented using the Truth-Grid method with random symmetries.
- **Robust Validation**: Ensures generated squares are valid and handles edge cases (like $n=2$ which is impossible).
- **Responsive Web UI**: An interface built with vanilla HTML/CSS/JS.

## 🚀 Live Demo

[View the Live Demo on GitHub Pages](https://arnaudamiel.github.io/Magic-Squares/)

## 🛠️ Build & Run

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Web Application (WASM)

To build the project for the web:

1.  **Build the WASM package**:
    ```bash
    wasm-pack build --target web
    ```
    This will generate the necessary files in the `pkg/` directory.

2.  **Run Locally**:
    Start a local web server in the project root to serve `index.html`.
    ```bash
    python -m http.server 8000
    ```
    Open `http://localhost:8000` in your browser.

    > **Note**: You cannot open `index.html` directly (via `file://`) because modern browsers restrict loading WASM modules from the local file system for security reasons.

### CLI Application

To run the generator from the command line:

1.  **Build**:
    ```bash
    cargo build --release
    ```

2.  **Run**:
    ```bash
    # Generate a magic square of order 7
    ./target/release/magic_squares.exe -n 7
    ```

## 🧩 Algorithms

The generator automatically selects the best algorithm based on the order $n$:

- **Odd ($n \pmod 2 \neq 0$)**: Uses the **Siamese Method** (De La Loubere). It places numbers diagonally, wrapping around edges.
- **Doubly Even ($n \pmod 4 = 0$)**: Uses a **Truth Grid Method**. It creates a pattern of valid/invalid positions and fills them with either $k$ or $n^2+1-k$.
- **Singly Even ($n \pmod 2 = 0, n \pmod 4 \neq 0$)**: Uses current **LUX Method**. It divides the square into $2 \times 2$ blocks and fills them according to a specific pattern (L, U, X) derived from a smaller magic square of size $n/2$.

## 📁 Project Structure

```
├── src/
│   ├── lib.rs        # WASM bindings verification logic
│   ├── main.rs       # CLI entry point
│   ├── generator.rs  # Core generation algorithms (Odd, Singly Even, Doubly Even)
│   ├── validator.rs  # Magic square property validation
│   └── rng.rs        # Custom Linear Congruential Generator (LCG)
├── pkg/              # Compiled WebAssembly artifacts
├── index.html        # Main web interface
├── style.css         # Application styling
└── ui.js             # Frontend logic & WASM bridge
```

## 📄 License

This project is open source and available under the MIT License.
