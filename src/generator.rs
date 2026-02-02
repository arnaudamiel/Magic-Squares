use crate::rng::Lcg;

/// Trait defining the interface for a Magic Square Generator.
/// Implementations of this trait handle specific cases based on the order $n$.
pub trait MagicGenerator {
    /// Generates a magic square of order $n$.
    /// Use nested vectors for easier mapping to WASM outputs, though flat vectors are more performant.
    fn generate(&mut self, n: usize) -> Vec<Vec<u32>>;
}

/// Generator for Odd order magic squares ($n % 2 != 0$).
/// Uses the Siamese (De La Loubere) method.
pub struct OddGenerator<'a> {
    rng: &'a mut Lcg,
}

impl<'a> OddGenerator<'a> {
    pub fn new(rng: &'a mut Lcg) -> Self {
        Self { rng }
    }

    /// Generates two base arrays (A and B) used for constructing the final square.
    /// This variation allows for additional shuffling/transformations if needed.
    fn generate_base_arrays(&mut self, n: usize) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
        let mut base_a = vec![vec![0; n]; n];
        let mut base_b = vec![vec![0; n]; n];

        // Standard De La Loubere (Siamese method) initialization.
        // Start in the middle of the top row.
        let mut r = 0;
        let mut c = n / 2;

        for k in 0..(n * n) {
            // k goes 0 to n^2-1
            // We decompose the value k into two components:
            // - A holds the "runs" (k / n)
            // - B holds the "cycles" (k % n)
            base_a[r][c] = (k / n) as u32;
            base_b[r][c] = (k % n) as u32;

            // Move Up-Right
            let next_r = if r == 0 { n - 1 } else { r - 1 };
            let next_c = if c == n - 1 { 0 } else { c + 1 };

            // Check for collision or if we've completed a cycle of n.
            // In the standard method, a collision happens exactly every n steps.
            // The condition `base_a[next_r][next_c] != 0` is one way to check,
            // but tracking the count `k` is deterministic and cleaner here because 0 is a valid value.
            if (k + 1) % n == 0 {
                // Move DOWN one step from the CURRENT position (not next).
                r = if r == n - 1 { 0 } else { r + 1 };
                // c stays same
            } else {
                // Move to the calculated Up-Right position.
                r = next_r;
                c = next_c;
            }
        }
        (base_a, base_b)
    }

    /// Checks if the diagonal properties of the grid allow for shuffling.
    /// This is an advanced check: if diagonals are constant or fully unique, transformations are safer.
    fn is_safe_diag(&self, grid: &Vec<Vec<u32>>, n: usize) -> bool {
        // Check Main Diagonal
        let diag1: Vec<u32> = (0..n).map(|i| grid[i][i]).collect();
        // Check Anti-Diagonal
        let diag2: Vec<u32> = (0..n).map(|i| grid[i][n-1-i]).collect();
        
        self.check_diag_vec(&diag1, n) && self.check_diag_vec(&diag2, n)
    }

    /// Helper verify a vector's properties for diagonal safety.
    fn check_diag_vec(&self, d: &Vec<u32>, n: usize) -> bool {
        // Safe if:
        // 1. All elements unique (Set size == n)
        // 2. All elements same (Set size == 1)
        // Unsafe if mixed repeats.
        let mut sorted = d.clone();
        sorted.sort_unstable();
        
        let mut unique_count = 1;
        for i in 1..n {
            if sorted[i] != sorted[i-1] {
                unique_count += 1;
            }
        }
        
        if unique_count == n { return true; } // Unique
        if unique_count == 1 { return true; } // Constant
        false // Mixed
    }

    /// Generates a shuffled mapping for the values 0..n-1.
    /// Constraints: The middle value must map to itself to preserve symmetry.
    fn get_shuffled_mapping(&mut self, n: usize, can_shuffle: bool) -> Vec<u32> {
        let mut vals: Vec<u32> = (0..n as u32).collect();
        
        if !can_shuffle {
            return vals;
        }

        // Constraint: Mid value maps to Mid value.
        let mid = (n - 1) / 2;
        // Remove mid
        vals.remove(mid);
        // Shuffle the remaining values
        self.rng.shuffle(&mut vals);
        // Insert mid back at its original position
        vals.insert(mid, mid as u32);
        
        vals
    }
}

impl<'a> MagicGenerator for OddGenerator<'a> {
    fn generate(&mut self, n: usize) -> Vec<Vec<u32>> {
        let (raw_a, raw_b) = self.generate_base_arrays(n);
        
        // Check safety of the generated base arrays.
        let safe_a = self.is_safe_diag(&raw_a, n);
        let safe_b = self.is_safe_diag(&raw_b, n);
        
        // We only shuffle if diagonal structure permits to maintain magic properties.
        // For example, N=3 is safe, but N=9 might be unsafe for arbitrary shuffling.
        let map_a = self.get_shuffled_mapping(n, safe_a);
        let map_b = self.get_shuffled_mapping(n, safe_b);

        let mut grid = vec![vec![0; n]; n];
        for r in 0..n {
            for c in 0..n {
                let val_a = map_a[raw_a[r][c] as usize];
                let val_b = map_b[raw_b[r][c] as usize];
                // Combine the two Greaco-Latin squares: Final = n * A + B + 1
                grid[r][c] = (n as u32 * val_a) + val_b + 1;
            }
        }
        grid
    }
}

/// Generator for Singly Even order magic squares ($n % 2 == 0$ but $n % 4 != 0$, e.g., 6, 10, 14).
/// Uses the LUX Method (Conway's method).
pub struct SinglyEvenGenerator<'a> {
    rng: &'a mut Lcg,
}

impl<'a> SinglyEvenGenerator<'a> {
    pub fn new(rng: &'a mut Lcg) -> Self {
        Self { rng }
    }
}

impl<'a> MagicGenerator for SinglyEvenGenerator<'a> {
    /// Implements the LUX Method.
    /// 1. Create a magic square of order $m = n/2$ (which is odd).
    /// 2. Each cell in the $m \times m$ square represents a $2 \times 2$ block in the target $n \times n$ square.
    /// 3. Fill blocks with specific patterns (L, U, X) based on the cell's position.
    fn generate(&mut self, n: usize) -> Vec<Vec<u32>> {
        let m = n / 2;
        // Use OddGenerator for the base pattern of size m
        let mut odd_gen = OddGenerator::new(self.rng);
        
        // The base square determines the order in which we fill blocks.
        // Subtract 1 from values to get 0-based indices.
        let base_square = odd_gen.generate(m); 
        
        let mut grid = vec![vec![0; n]; n];

        // LUX Pattern Preparation
        // Top k rows: L
        // Next 1 row: U
        // Bottom k-1 rows: X
        // Middle U must swap with L above it.
        
        let mut pattern_grid = vec![vec![' '; m]; m];
        let k_lux = m / 2;

        for r in 0..m {
            for c in 0..m {
                if r <= k_lux { pattern_grid[r][c] = 'L'; }
                else if r == k_lux + 1 { pattern_grid[r][c] = 'U'; }
                else { pattern_grid[r][c] = 'X'; }
            }
        }
        
        // Swap center U with L above it to satisfy magic properties.
        // Center of m square is at (k_lux, k_lux).
        pattern_grid[k_lux][k_lux] = 'U';
        pattern_grid[k_lux + 1][k_lux] = 'L';

        for r in 0..m {
            for c in 0..m {
                let val = base_square[r][c] - 1; // 0-based sequence value for this block
                let start = val * 4 + 1; // The starting number for this 2x2 block (e.g., 1, 5, 9...)
                
                // Top-left coordinates of the 2x2 block in the final grid
                let br = r * 2;
                let bc = c * 2;

                // Fill the 2x2 block according to the pattern
                match pattern_grid[r][c] {
                    'L' => {
                        // L pattern:
                        // . 1
                        // 2 3
                        // 4 .
                        // (Visual representation of order within block)
                        // Correct filling:
                        grid[br][bc+1] = start;       // 1 (Top Right)
                        grid[br+1][bc] = start + 1;   // 2 (Bot Left)
                        grid[br+1][bc+1] = start + 2; // 3 (Bot Right)
                        grid[br][bc] = start + 3;     // 4 (Top Left)
                    },
                    'U' => {
                        // U pattern:
                        // 1 4
                        // 2 3
                        grid[br][bc] = start;         // 1 (Top Left)
                        grid[br+1][bc] = start + 1;   // 2 (Bot Left)
                        grid[br+1][bc+1] = start + 2; // 3 (Bot Right)
                        grid[br][bc+1] = start + 3;   // 4 (Top Right)
                    },
                    'X' => {
                        // X pattern:
                        // 1 4
                        // 3 2
                        grid[br][bc] = start;         // 1 (Top Left)
                        grid[br+1][bc+1] = start + 1; // 2 (Bot Right)
                        grid[br+1][bc] = start + 2;   // 3 (Bot Left)
                        grid[br][bc+1] = start + 3;   // 4 (Top Right)
                    },
                    _ => {}
                }
            }
        }
        grid
    }
}

/// Generator for Doubly Even order magic squares ($n % 4 == 0$).
/// Uses the Truth-Grid method (or Generalized Method of 4).
pub struct DoublyEvenGenerator<'a> {
    rng: &'a mut Lcg,
}

impl<'a> DoublyEvenGenerator<'a> {
    pub fn new(rng: &'a mut Lcg) -> Self {
        Self { rng }
    }
}

impl<'a> MagicGenerator for DoublyEvenGenerator<'a> {
    fn generate(&mut self, n: usize) -> Vec<Vec<u32>> {
        // Concept:
        // 1. Fill grid sequentially 1..n^2.
        // 2. Identify "Diagonal" positions using a 4x4 truth grid pattern.
        // 3. For diagonal positions, invert the value: val = (n^2 + 1) - val.
        // 4. For non-diagonal positions, keep the sequential value.
        // Condition for Diagonal: (i % 4 == j % 4) || (i % 4 + j % 4 == 3)
        
        let mut grid = vec![vec![0; n]; n];
        
        // Random Variations:
        // To produce different squares, we can apply symmetries to the indices
        // (Transpose, Reflect) before filling.
        let do_transpose = self.rng.next_range(0, 2) == 1;
        let do_flip_r = self.rng.next_range(0, 2) == 1;
        let do_flip_c = self.rng.next_range(0, 2) == 1;

        for r in 0..n {
            for c in 0..n {
                let val_seq = (r * n + c + 1) as u32;
                let val_inv = ((n * n) as u32 + 1) - val_seq;

                // Check 4x4 block diagonal condition
                let r4 = r % 4;
                let c4 = c % 4;
                // Primary diagonal within block || Secondary diagonal within block
                let is_diag = (r4 == c4) || (r4 + c4 == 3);

                let val = if is_diag { val_inv } else { val_seq };

                // Apply random variations to target indices
                let mut tr = r;
                let mut tc = c;
                
                if do_flip_r { tr = n - 1 - tr; }
                if do_flip_c { tc = n - 1 - tc; }
                if do_transpose { 
                   let temp = tr; tr = tc; tc = temp; 
                }
                
                grid[tr][tc] = val;
            }
        }
        grid
    }
}
