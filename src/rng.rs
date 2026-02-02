use std::time::{SystemTime, UNIX_EPOCH};

pub struct Lcg {
    state: u64,
}

impl Lcg {
    pub fn new() -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let seed = since_the_epoch.as_nanos() as u64;
        Self { state: seed }
    }

    pub fn next_u32(&mut self) -> u32 {
        // LCG parameters (Knuth's MMIX)
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.state >> 32) as u32
    }

    pub fn next_range(&mut self, min: usize, max: usize) -> usize {
        let range = max - min;
        if range == 0 {
            return min;
        }
        let val = self.next_u32() as usize;
        min + (val % range)
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = self.next_range(0, i + 1);
            slice.swap(i, j);
        }
    }
}
