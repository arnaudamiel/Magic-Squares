use crate::rng::Lcg;

pub trait MagicGenerator {
    fn generate(&mut self, n: usize) -> Vec<Vec<u32>>;
}

pub struct OddGenerator<'a> {
    rng: &'a mut Lcg,
}

impl<'a> OddGenerator<'a> {
    pub fn new(rng: &'a mut Lcg) -> Self {
        Self { rng }
    }

    fn generate_base_arrays(&mut self, n: usize) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
        let mut base_a = vec![vec![0; n]; n];
        let mut base_b = vec![vec![0; n]; n];

        // Standard De La Loubere (Siamese method)
        // Start middle of top row
        let mut r = 0;
        let mut c = n / 2;

        for k in 0..(n * n) {
            // k goes 0 to n^2-1
            // A gets runs: k / n
            // B gets cycles: k % n
            // But wait, standard Siamese places 1..n^2. 
            // We want to decompose the values.
            // Let's place (k / n) into A and (k % n) into B at position (r, c)
            base_a[r][c] = (k / n) as u32;
            base_b[r][c] = (k % n) as u32;

            // Move Up-Right
            let next_r = if r == 0 { n - 1 } else { r - 1 };
            let next_c = if c == n - 1 { 0 } else { c + 1 };

            if base_a[next_r][next_c] != 0 || (next_r == 0 && next_c == n/2 && k != 0) { // Check occupancy via B is safer? No, 0 is valid.
                                                                                       // Check if we looped? De La Loubere collision rule:
                                                                                       // If blocked, move DOWN from original relative to current? 
                                                                                       // Standard rule: if next is filled, go down one from current.
                                                                                       // But we are filling A and B. How to check 'filled'?
                                                                                       // We validly visit every cell exactly once.
                                                                                       // We can track filled via a separate boolean grid or just implied logic.
                 // Actually, the collision happens every n steps.
                 if (k + 1) % n == 0 {
                     // Move down
                     r = if r == n - 1 { 0 } else { r + 1 };
                     // c stays same
                 } else {
                     r = next_r;
                     c = next_c;
                 }
            } else {
                 // Logic for "is filled" is tricky if 0 is a valid value.
                 // But simply using the modulo rule is the correct deterministic way.
                 r = next_r;
                 c = next_c;
            }
        }
        (base_a, base_b)
    }

    fn is_safe_diag(&self, grid: &Vec<Vec<u32>>, n: usize) -> bool {
        // Check Main Diag
        let diag1: Vec<u32> = (0..n).map(|i| grid[i][i]).collect();
        // Check Anti Diag
        let diag2: Vec<u32> = (0..n).map(|i| grid[i][n-1-i]).collect();
        
        self.check_diag_vec(&diag1, n) && self.check_diag_vec(&diag2, n)
    }

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

    fn get_shuffled_mapping(&mut self, n: usize, can_shuffle: bool) -> Vec<u32> {
        let mut vals: Vec<u32> = (0..n as u32).collect();
        
        if !can_shuffle {
            return vals;
        }

        // Constraint: Mid value maps to Mid value.
        let mid = (n - 1) / 2;
        // Remove mid
        vals.remove(mid);
        // Shuffle
        self.rng.shuffle(&mut vals);
        // Insert mid back at mid
        vals.insert(mid, mid as u32);
        
        vals
    }
}

impl<'a> MagicGenerator for OddGenerator<'a> {
    fn generate(&mut self, n: usize) -> Vec<Vec<u32>> {
        let (raw_a, raw_b) = self.generate_base_arrays(n);
        
        // Check safety
        let safe_a = self.is_safe_diag(&raw_a, n);
        let safe_b = self.is_safe_diag(&raw_b, n);
        
        // We only shuffle if diagonal structure permits.
        // If N=3, safe. N=9, likely unsafe.
        let map_a = self.get_shuffled_mapping(n, safe_a);
        let map_b = self.get_shuffled_mapping(n, safe_b);

        let mut grid = vec![vec![0; n]; n];
        for r in 0..n {
            for c in 0..n {
                let val_a = map_a[raw_a[r][c] as usize];
                let val_b = map_b[raw_b[r][c] as usize];
                grid[r][c] = (n as u32 * val_a) + val_b + 1;
            }
        }
        grid
    }
}

pub struct SinglyEvenGenerator<'a> {
    rng: &'a mut Lcg,
}

impl<'a> SinglyEvenGenerator<'a> {
    pub fn new(rng: &'a mut Lcg) -> Self {
        Self { rng }
    }
}

impl<'a> MagicGenerator for SinglyEvenGenerator<'a> {
    // LUX Method
    fn generate(&mut self, n: usize) -> Vec<Vec<u32>> {
        let m = n / 2;
        // Use OddGenerator for the base pattern of size m
        let mut odd_gen = OddGenerator::new(self.rng);
        // We need the raw 1..m^2 square from it, explicitly subtract 1 to get 0..m^2-1 indices for LUX? 
        // LUX works by picking a pattern L, U, or X for each block.
        // The ORDER of blocks is determined by the magic square of size m.
        let base_square = odd_gen.generate(m); 
        
        let mut grid = vec![vec![0; n]; n];

        // LUX Patterns
        // L: [ [4,1], [2,3] ] 
        // U: [ [1,4], [2,3] ]
        // X: [ [1,4], [3,2] ]
        // These are relative offsets? No.
        // LUX fills the block with values $4k+1, 4k+2, 4k+3, 4k+4$ where $k$ is the value in base_square - 1.
        // The positions of 1, 2, 3, 4 depend on the letter.
        
        // Define rows for LUX patterns

        
        // Rows 0 to split: L
        // Row split + 1: U
        // Rows split + 2 to m-1: X
        // Middle Row (split) is mostly L, but one U needs to swap with L below it?
        // Standard LUX:
        // Top k rows: L
        // Next 1 row: U
        // Bottom k-1 rows: X
        // Swap center U with L above it.

        let mut pattern_grid = vec![vec![' '; m]; m];
        let k_lux = m / 2;

        for r in 0..m {
            for c in 0..m {
                if r <= k_lux { pattern_grid[r][c] = 'L'; }
                else if r == k_lux + 1 { pattern_grid[r][c] = 'U'; }
                else { pattern_grid[r][c] = 'X'; }
            }
        }
        // Swap center U with L above
        // Center of m is (k_lux, k_lux).
        // Since we adjusted indices:
        // Row k_lux is L. Row k_lux+1 is U.
        // We swap [k_lux+1][k_lux] (U) with [k_lux][k_lux] (L).
        pattern_grid[k_lux][k_lux] = 'U';
        pattern_grid[k_lux + 1][k_lux] = 'L';

        for r in 0..m {
            for c in 0..m {
                let val = base_square[r][c] - 1; // 0-based index
                let start = val * 4 + 1;
                
                // Block coords
                let br = r * 2;
                let bc = c * 2;

                match pattern_grid[r][c] {
                    'L' => {
                        grid[br][bc+1] = start;     // 1 -> Top Right
                        grid[br+1][bc] = start + 1; // 2 -> Bot Left
                        grid[br+1][bc+1] = start + 2; // 3 -> Bot Right
                        grid[br][bc] = start + 3;     // 4 -> Top Left
                    },
                    'U' => {
                        grid[br][bc] = start;       // 1 -> Top Left
                        grid[br+1][bc] = start + 1; // 2 -> Bot Left
                        grid[br+1][bc+1] = start + 2; // 3 -> Bot Right
                        grid[br][bc+1] = start + 3;   // 4 -> Top Right
                    },
                    'X' => {
                        grid[br][bc] = start;       // 1 -> Top Left
                        grid[br+1][bc+1] = start + 1; // 2 -> Bot Right
                        grid[br+1][bc] = start + 2;   // 3 -> Bot Left
                        grid[br][bc+1] = start + 3;   // 4 -> Top Right
                    },
                    _ => {}
                }
            }
        }
        grid
    }
}

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
        // Simple Doubly Even
        // Create 1..n^2 grid.
        // Truth grid: 4x4 blocks. Diagonals are True.
        // Or simpler: (i % 4 == j % 4) || (i % 4 + j % 4 == 3)
        
        let mut grid = vec![vec![0; n]; n];
        
        // Variation: Transpose output? Reflect?
        // We determine variation flags first.
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
                let is_diag = (r4 == c4) || (r4 + c4 == 3);

                let val = if is_diag { val_inv } else { val_seq };

                // Apply variation to indices
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
