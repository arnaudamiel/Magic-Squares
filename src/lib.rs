mod rng;
mod generator;
mod validator;

use wasm_bindgen::prelude::*;
use rng::Lcg;
use generator::{MagicGenerator, OddGenerator, SinglyEvenGenerator, DoublyEvenGenerator};

#[wasm_bindgen]
pub struct MagicSquareResult {
    grid: Vec<u32>,
    n: usize,
}

#[wasm_bindgen]
impl MagicSquareResult {
    #[wasm_bindgen(getter)]
    pub fn grid(&self) -> Vec<u32> {
        self.grid.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn n(&self) -> usize {
        self.n
    }
}

#[wasm_bindgen]
pub fn generate_magic_square(n: usize) -> Option<MagicSquareResult> {
    if n == 2 || n == 0 {
        return None;
    }

    let mut lcg = Lcg::new();
    let mut magic_gen: Box<dyn MagicGenerator> = if n % 2 != 0 {
        Box::new(OddGenerator::new(&mut lcg))
    } else if n % 4 != 0 {
        Box::new(SinglyEvenGenerator::new(&mut lcg))
    } else {
        Box::new(DoublyEvenGenerator::new(&mut lcg))
    };

    let square = magic_gen.generate(n);
    let flat_grid = square.into_iter().flatten().collect();

    Some(MagicSquareResult {
        grid: flat_grid,
        n,
    })
}

#[wasm_bindgen]
pub fn verify_magic_square(n: usize, flat_grid: Vec<u32>) -> bool {
    if flat_grid.len() != n * n {
        return false;
    }

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
