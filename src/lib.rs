mod rng;
mod generator;
mod validator;

use wasm_bindgen::prelude::*;
use rng::Lcg;
use generator::{MagicGenerator, OddGenerator, SinglyEvenGenerator, DoublyEvenGenerator};

/// Represents the result of a magic square generation.
/// This struct is exported to WASM, allowing Javascript to access the grid and its order.
#[wasm_bindgen]
pub struct MagicSquareResult {
    /// The flattened magic square grid (1D vector).
    grid: Vec<u32>,
    /// The order of the magic square (n).
    n: usize,
}

#[wasm_bindgen]
impl MagicSquareResult {
    /// Returns a copy of the flattened grid to Javascript.
    /// Note: Cloning is necessary because we are passing ownership across the WASM boundary.
    #[wasm_bindgen(getter)]
    pub fn grid(&self) -> Vec<u32> {
        self.grid.clone()
    }

    /// Returns the order (n) of the square.
    #[wasm_bindgen(getter)]
    pub fn n(&self) -> usize {
        self.n
    }
}

/// Main entry point for generating a magic square from Javascript.
///
/// # Arguments
///
/// * `n` - The order of the magic square to generate.
///
/// # Returns
///
/// * `Option<MagicSquareResult>` - The generated result, or `None` if `n` is invalid (e.g., 0 or 2).
#[wasm_bindgen]
pub fn generate_magic_square(n: usize) -> Option<MagicSquareResult> {
    // Validate input: 0 and 2 are impossible orders for standard magic squares.
    if n == 2 || n == 0 {
        return None;
    }

    // Initialize our custom Linear Congruential Generator (LCG).
    // In a real-world scenario, we might seed this with time, but for deterministic/demo purposes, we use default.
    let mut lcg = Lcg::new();
    
    // Select the appropriate generator based on the order n.
    let mut magic_gen: Box<dyn MagicGenerator> = if n % 2 != 0 {
        Box::new(OddGenerator::new(&mut lcg))
    } else if n % 4 != 0 {
        Box::new(SinglyEvenGenerator::new(&mut lcg))
    } else {
        Box::new(DoublyEvenGenerator::new(&mut lcg))
    };

    // Generate the square logic.
    let square = magic_gen.generate(n);
    
    // Flatten the 2D vector into a 1D vector for easier passing to JS.
    let flat_grid = square.into_iter().flatten().collect();

    Some(MagicSquareResult {
        grid: flat_grid,
        n,
    })
}

/// Verifies if a given grid is a valid magic square.
/// This function is exported to allow client-side verification if needed.
#[wasm_bindgen]
pub fn verify_magic_square(n: usize, flat_grid: Vec<u32>) -> bool {
    if flat_grid.len() != n * n {
        return false;
    }

    // Reconstruct 2D grid from flat vector
    let mut grid = Vec::with_capacity(n);
    for chunk in flat_grid.chunks(n) {
        grid.push(chunk.to_vec());
    }

    validator::check_magic_properties(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_order_3() {
        let result = generate_magic_square(3).expect("Should generate order 3");
        assert_eq!(result.n(), 3);
        assert_eq!(result.grid().len(), 9);
        assert!(verify_magic_square(3, result.grid()));
    }

    #[test]
    fn test_generate_order_4() {
        let result = generate_magic_square(4).expect("Should generate order 4");
        assert_eq!(result.n(), 4);
        assert_eq!(result.grid().len(), 16);
        assert!(verify_magic_square(4, result.grid()));
    }

    #[test]
    fn test_generate_order_6() {
        let result = generate_magic_square(6).expect("Should generate order 6");
        assert_eq!(result.n(), 6);
        assert_eq!(result.grid().len(), 36);
        assert!(verify_magic_square(6, result.grid()));
    }

    #[test]
    fn test_invalid_orders() {
        assert!(generate_magic_square(0).is_none());
        assert!(generate_magic_square(2).is_none());
    }
}
