#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

/// A simple Linear Congruential Generator (LCG) for random number generation.
/// We use this instead of the `rand` crate to minimize WASM bundle size.
/// Formula: $X_{n+1} = (aX_n + c) \pmod m$
pub struct Lcg {
    state: u64,
}

impl Lcg {

    /// Creates a new LCG seeded with the current system time.
    /// Uses `js_sys::Date::now()` on WASM and `std::time::SystemTime` natively.
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            let now = js_sys::Date::now(); // Returns milliseconds as f64
            let seed = now as u64;
            Self { state: seed }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            let seed = since_the_epoch.as_nanos() as u64;
            Self { state: seed }
        }
    }

    /// Generates the next random `u32`.
    /// Uses constants from Knuth's MMIX implementation.
    /// $a = 6364136223846793005$
    /// $c = 1442695040888963407$
    pub fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        // Return the high 32 bits for better distribution quality
        (self.state >> 32) as u32
    }

    /// Generates a random number in the range `[min, max)`.
    pub fn next_range(&mut self, min: usize, max: usize) -> usize {
        let range = max - min;
        if range == 0 {
            return min;
        }
        let val = self.next_u32() as usize;
        min + (val % range)
    }

    /// Shuffles a mutable slice using the Fisher-Yates shuffle algorithm.
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            // Pick a random index from 0 to i
            let j = self.next_range(0, i + 1);
            slice.swap(i, j);
        }
    }
}
