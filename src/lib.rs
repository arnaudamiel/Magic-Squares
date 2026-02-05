pub mod rng;
pub mod generator;
pub mod validator;

use wasm_bindgen::prelude::*;
use rng::Lcg;


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
    /// Returns a raw pointer to the grid buffer.
    #[wasm_bindgen]
    pub fn get_grid_ptr(&self) -> *const u32 {
        self.grid.as_ptr()
    }

    /// Returns the number of elements in the grid.
    #[wasm_bindgen]
    pub fn get_grid_len(&self) -> usize {
        self.grid.len()
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
/// * `Result<MagicSquareResult, JsError>` - The generated result, or an error if generation fails.
#[wasm_bindgen]
pub fn generate_magic_square(n: usize) -> Result<MagicSquareResult, JsError> {
    // 1. Validate basic magic square constraints
    if n == 2 {
        return Err(JsError::new("Order 2 magic squares are mathematically impossible."));
    }
    if n == 0 {
        return Err(JsError::new("Order cannot be 0."));
    }

    // 2. Validate integer overflow safety
    // The maximum value in a magic square of order n is n^2.
    // We use u32 to store values, so n^2 must fit in u32::MAX.
    // sqrt(u32::MAX) = sqrt(4,294,967,295) ≈ 65535.
    if n > 65535 {
        return Err(JsError::new(&format!(
            "Order {} is too large. Max allowed order is 65535 to prevent integer overflow in u32.",
            n
        )));
    }

    // 3. Validate memory safety (Soft Limit)
    // A grid of size n*n*4 bytes (u32).
    // e.g., n=20,000 -> 400,000,000 elements * 4 bytes = 1.6 GB.
    // This is risky for a browser tab. Let's set a conservative limit around 200MB (~50M elements).
    // sqrt(50,000,000) ≈ 7071.
    const MAX_SAFE_ORDER: usize = 7000;
    if n > MAX_SAFE_ORDER {
        return Err(JsError::new(&format!(
            "Order {} is too large for browser memory safety. Capped at {}.",
            n, MAX_SAFE_ORDER
        )));
    }

    // Initialize our custom Linear Congruential Generator (LCG).
    // In a real-world scenario, we might seed this with time, but for deterministic/demo purposes, we use default.
    let mut lcg = Lcg::new();
    
    // Select the appropriate generator based on the order n.
    let mut magic_gen = generator::create(n, &mut lcg);

    // Generate the square logic. (This could still panic on OOM, but our checks above minimize it)
    let square_vec = magic_gen.generate(n);
    
    // The result is already a flat Vec<u32>, so no flattening needed!
    Ok(MagicSquareResult {
        grid: square_vec,
        n,
    })
}

/// Verifies if a given grid is a valid magic square.
/// This function is exported to allow client-side verification if needed.
#[wasm_bindgen]
pub fn verify_magic_square(n: usize, flat_grid: Vec<u32>) -> bool {
    validator::check_magic_properties(&flat_grid, n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_generate_order_3() {
        let result = generate_magic_square(3).expect("Should generate order 3");
        assert_eq!(result.n(), 3);
        assert_eq!(result.grid().len(), 9);
        assert!(verify_magic_square(3, result.grid()));
    }

    #[wasm_bindgen_test]
    fn test_generate_order_4() {
        let result = generate_magic_square(4).expect("Should generate order 4");
        assert_eq!(result.n(), 4);
        assert_eq!(result.grid().len(), 16);
        assert!(verify_magic_square(4, result.grid()));
    }

    #[wasm_bindgen_test]
    fn test_generate_order_6() {
        let result = generate_magic_square(6).expect("Should generate order 6");
        assert_eq!(result.n(), 6);
        assert_eq!(result.grid().len(), 36);
        assert!(verify_magic_square(6, result.grid()));
    }

    #[wasm_bindgen_test]
    fn test_invalid_orders() {
        assert!(generate_magic_square(0).is_err());
        assert!(generate_magic_square(2).is_err());
    }

    #[wasm_bindgen_test]
    fn test_too_large_order() {
        // Test soft limit
        assert!(generate_magic_square(7001).is_err());
        assert!(generate_magic_square(66000).is_err());
    }
}
