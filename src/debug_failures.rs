mod rng;
mod generator;
mod validator;

use rng::Lcg;
use generator::{MagicGenerator, OddGenerator, SinglyEvenGenerator, DoublyEvenGenerator};

fn main() {
    let mut lcg = Lcg::new();
    
    // Debug N=9
    println!("DEBUG N=9");
    let mut gen9 = OddGenerator::new(&mut lcg);
    // Only looking at base structure
    // We can't access private methods easily unless we make them pub or just copy logic.
    // I'll assume the generated square is available.
    let sq9 = gen9.generate(9);
    print_analysis(9, &sq9);

    // Debug N=6
    println!("\nDEBUG N=6");
    let mut gen6 = SinglyEvenGenerator::new(&mut lcg);
    let sq6 = gen6.generate(6);
    print_analysis(6, &sq6);
}

fn print_analysis(n: usize, grid: &Vec<Vec<u32>>) {
    let magic_constant = (n as u32 * ((n * n) as u32 + 1)) / 2;
    println!("Target Constant: {}", magic_constant);

    // Rows
    print!("Rows: ");
    for r in 0..n {
        let s: u32 = grid[r].iter().sum();
        if s != magic_constant { print!("R{}:{} ", r, s); }
    }
    println!();

    // Cols
    print!("Cols: ");
    for c in 0..n {
        let s: u32 = (0..n).map(|r| grid[r][c]).sum();
        if s != magic_constant { print!("C{}:{} ", c, s); }
    }
    println!();

    // Diags
    let d1: u32 = (0..n).map(|i| grid[i][i]).sum();
    let d2: u32 = (0..n).map(|i| grid[i][n-1-i]).sum();
    println!("D1: {} (Diff {}), D2: {} (Diff {})", 
        d1, d1 as i32 - magic_constant as i32, 
        d2, d2 as i32 - magic_constant as i32);
}
