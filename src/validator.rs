/// Verifies that a given sequence of vectors forms a valid magic square.
///
/// A magic square of order $n$ must satisfy:
/// 1. The sum of every row is the magic constant $M = n(n^2+1)/2$.
/// 2. The sum of every column is $M$.
/// 3. The sum of both main diagonals is $M$.
/// 4. All numbers from $1$ to $n^2$ appear exactly once.
pub fn check_magic_properties(grid: &Vec<Vec<u32>>) -> bool {
    let n = grid.len();
    if n == 0 { return false; }
    
    // Calculate the Magic Constant: M = n * (n^2 + 1) / 2
    let magic_constant = (n as u32 * ((n * n) as u32 + 1)) / 2;

    // Check Rows
    for r in 0..n {
        let sum: u32 = grid[r].iter().sum();
        if sum != magic_constant { return false; }
    }

    // Check Columns
    for c in 0..n {
        let sum: u32 = (0..n).map(|r| grid[r][c]).sum();
        if sum != magic_constant { return false; }
    }

    // Check Main Diagonal (Top-Left to Bottom-Right)
    let diag1: u32 = (0..n).map(|i| grid[i][i]).sum();
    if diag1 != magic_constant { return false; }

    // Check Anti-Diagonal (Top-Right to Bottom-Left)
    let diag2: u32 = (0..n).map(|i| grid[i][n - 1 - i]).sum();
    if diag2 != magic_constant { return false; }

    // Check Uniqueness (1..n^2)
    // We flatten the grid, sort it, and verify sequentiality.
    let mut flat: Vec<u32> = grid.iter().flatten().cloned().collect();
    flat.sort_unstable();
    
    for (i, &val) in flat.iter().enumerate() {
        if val != (i + 1) as u32 { return false; }
    }

    true
}
