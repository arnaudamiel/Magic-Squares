mod rng;
mod generator;
mod validator;

use rng::Lcg;
use generator::{MagicGenerator, OddGenerator, SinglyEvenGenerator, DoublyEvenGenerator};
use std::env;
use std::collections::HashSet;

fn get_generator<'a>(n: usize, rng: &'a mut Lcg) -> Box<dyn MagicGenerator + 'a> {
    if n % 2 != 0 {
        Box::new(OddGenerator::new(rng))
    } else if n % 4 != 0 {
        Box::new(SinglyEvenGenerator::new(rng))
    } else {
        Box::new(DoublyEvenGenerator::new(rng))
    }
}

/// Main entry point for the Command Line Interface (CLI) version of the Magic Square Generator.
/// 
/// Usage:
///     magic_squares.exe -n <ORDER>
///
/// Example:
///     magic_squares.exe -n 7
///
/// If no arguments are provided, it runs a verification suite for orders 1-100.
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut target_n = 0;

    // Parse arguments
    for i in 0..args.len() {
        if args[i] == "-n" && i + 1 < args.len() {
            if let Ok(n) = args[i+1].parse::<usize>() {
                target_n = n;
            }
        }
    }

    let mut lcg = Lcg::new();

    if target_n > 0 {
        // Single Generation Mode
        if target_n == 2 {
             println!("Order 2 Magic Square is impossible.");
             return;
        }
        let mut magic_gen = get_generator(target_n, &mut lcg);
        let sq = magic_gen.generate(target_n);
        print_square(&sq);
        
        if validator::check_magic_properties(&sq) {
            println!("\nVerified: This is a valid magic square.");
        } else {
            println!("\nError: The generated square is invalid!");
        }

    } else {
        // Verification Mode (Orders 1 to 100)
        println!("Running Verification for Orders 1 to 100 (100 samples each)...");
        
        for n in 1..=100 {
            if n == 2 {
                println!("Order 2: Impossible (Skipping)");
                continue;
            }

            let mut unique_squares = HashSet::new();
            let mut all_valid = true;

            for _ in 0..100 {
                let mut magic_gen = get_generator(n, &mut lcg);
                let sq = magic_gen.generate(n);
                
                if !validator::check_magic_properties(&sq) {
                    all_valid = false;
                    println!("Order {}: INVALID SQUARE GENERATED!", n);
                    break;
                }
                
                unique_squares.insert(sq);
            }

            if all_valid {
                println!("Order {}: 100/100 Valid. Unique Variations: {}", n, unique_squares.len());
            } else {
                println!("Order {}: FAILED VALIDATION", n);
            }
        }
    }
}

fn print_square(grid: &Vec<Vec<u32>>) {
    if grid.is_empty() { return; }
    let n = grid.len();
    let max_val = n * n;
    let width = max_val.to_string().len() + 1; // +1 for spacing

    for row in grid {
        for val in row {
            print!("{:width$}", val, width = width);
        }
        println!();
    }
}
