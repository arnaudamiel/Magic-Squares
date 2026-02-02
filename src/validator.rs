pub fn check_magic_properties(grid: &Vec<Vec<u32>>) -> bool {
    let n = grid.len();
    if n == 0 { return false; }
    
    let magic_constant = (n as u32 * ((n * n) as u32 + 1)) / 2;

    // Check Rows
    for r in 0..n {
        let sum: u32 = grid[r].iter().sum();
        if sum != magic_constant { return false; }
    }

    // Check Cols
    for c in 0..n {
        let sum: u32 = (0..n).map(|r| grid[r][c]).sum();
        if sum != magic_constant { return false; }
    }

    // Check Diagonals
    let diag1: u32 = (0..n).map(|i| grid[i][i]).sum();
    if diag1 != magic_constant { return false; }

    let diag2: u32 = (0..n).map(|i| grid[i][n - 1 - i]).sum();
    if diag2 != magic_constant { return false; }

    // Check Uniqueness (1..n^2)
    let mut flat: Vec<u32> = grid.iter().flatten().cloned().collect();
    flat.sort_unstable();
    
    for (i, &val) in flat.iter().enumerate() {
        if val != (i + 1) as u32 { return false; }
    }

    true
}
