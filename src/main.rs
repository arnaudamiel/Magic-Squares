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
        print_square(&sq, target_n);
        
        if validator::check_magic_properties(&sq, target_n) {
            println!("\nVerified: This is a valid magic square.");
        } else {
            println!("\nError: The generated square is invalid!");
        }

    } else {
        // Parallel Verification Mode (Orders 1 to 100)
        use std::thread;
        use std::sync::mpsc;
        
        println!("Running Parallel Verification for Orders 1 to 100 (100 samples each)...");
        
        // Determine number of worker threads
        let num_threads = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        
        println!("Using {} worker threads", num_threads);
        
        // Collect all orders to process (excluding n=2)
        let orders: Vec<usize> = (1..=100).filter(|&n| n != 2).collect();
        let chunk_size = (orders.len() + num_threads - 1) / num_threads;
        
        let (tx, rx) = mpsc::channel();
        
        // Spawn worker threads
        let handles: Vec<_> = orders
            .chunks(chunk_size)
            .enumerate()
            .map(|(thread_id, chunk)| {
                let chunk = chunk.to_vec();
                let tx = tx.clone();
                
                thread::spawn(move || {
                    for &n in &chunk {
                        // Each thread gets its own RNG seeded with thread_id + n
                        let mut lcg = Lcg::new_with_seed((thread_id * 1000 + n) as u64);
                        let mut unique_squares = HashSet::new();
                        let mut all_valid = true;
                        
                        for _ in 0..100 {
                            let mut magic_gen = get_generator(n, &mut lcg);
                            let sq = magic_gen.generate(n);
                            
                            if !validator::check_magic_properties(&sq, n) {
                                all_valid = false;
                                break;
                            }
                            unique_squares.insert(sq);
                        }
                        
                        tx.send((n, all_valid, unique_squares.len())).unwrap();
                    }
                })
            })
            .collect();
        
        // Close the sender so receiver knows when to stop
        drop(tx);
        
        // Collect results from all threads
        let mut results: Vec<_> = rx.iter().collect();
        results.sort_by_key(|&(n, _, _)| n);
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Print results in order
        for (n, valid, unique_count) in results {
            if valid {
                println!("Order {}: 100/100 Valid. Unique Variations: {}", n, unique_count);
            } else {
                println!("Order {}: FAILED VALIDATION", n);
            }
        }
    }
}

fn print_square(grid: &[u32], n: usize) {
    if grid.is_empty() { return; }
    let max_val = n * n;
    let width = max_val.to_string().len() + 1; // +1 for spacing

    for r in 0..n {
        for c in 0..n {
            let val = grid[r * n + c];
            print!("{:width$}", val, width = width);
        }
        println!();
    }
}
