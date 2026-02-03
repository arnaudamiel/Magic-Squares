# Magic Square Generator

A high-performance Magic Square generator built with Rust and WebAssembly (WASM). This project provides both a command-line interface (CLI) and a modern, responsive web application.

## 🌟 Features

- **Blazing Fast Generation**: Powered by Rust's performance and WASM for near-native speed in the browser.
- **Progressive Web App**: Installable on desktop and mobile devices with full offline support.
- **Support for All Orders**:
    - **Odd Orders**: Implemented using the Siamese (De La Loubere) method optimizations.
    - **Singly Even Orders**: Implemented using the LUX method (Conway's method).
    - **Doubly Even Orders**: Implemented using the Truth-Grid method with random symmetries.
- **Robust Validation**: Ensures generated squares are valid and handles edge cases (like $n=2$ which is impossible).
- **Responsive Web UI**: An interface built with vanilla HTML/CSS/JS that works perfectly on mobile and desktop.

## 📱 Progressive Web App

This application is a **Progressive Web App (PWA)**, which means you can:

- **Install it on your device**: Add it to your home screen on mobile or desktop for a native app-like experience
- **Use it offline**: Once loaded, the app works completely offline - generate magic squares anywhere, anytime
- **Fast loading**: Assets are cached for instant loading on subsequent visits
- **Automatic updates**: The app updates automatically in the background when new versions are available

### How to Install

**On Desktop (Chrome/Edge):**
1. Visit the [Live Demo](https://arnaudamiel.github.io/Magic-Squares/)
2. Look for the install icon (⊕) in the address bar
3. Click "Install" to add the app to your applications

**On Mobile (Android/iOS):**
1. Visit the [Live Demo](https://arnaudamiel.github.io/Magic-Squares/)
2. Tap the browser menu (⋮ or share icon)
3. Select "Add to Home Screen" or "Install App"
4. The app will appear on your home screen like a native app

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

### Running Locally with Python

Since modern browsers restrict loading WebAssembly from `file://` URIs, you **must** serve the files through a local web server.

1.  **Ensure you have Python installed** (Python 3 is recommended).
2.  **Open a terminal** in the project root directory.
3.  **Start the server**:
    ```bash
    # For Python 3
    python -m http.server 3000
    ```
4.  **Access the application**:
    Open [http://localhost:3000](http://localhost:3000) in your web browser.

> [!TIP]
> You can use any port (e.g., 8080, 5000) by changing the number at the end of the command.

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
├── icons/            # PWA app icons
├── index.html        # Main web interface
├── style.css         # Application styling
├── ui.js             # Frontend logic & WASM bridge
├── manifest.json     # PWA manifest
└── sw.js             # Service Worker for offline support
```

## 📄 License

This project is open source and available under the MIT License.
