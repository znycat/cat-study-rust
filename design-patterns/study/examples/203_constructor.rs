fn main() {
    unimplemented!();
}

#[derive(Default)]
pub struct Second {
    value: u64,
}

impl Second {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    /// Returns the value of the second
    pub fn value(&self) -> u64 {
        self.value
    }
}

// impl Default for Second {
//     fn default() -> Self {
//         Self { value: 0 }
//     }
// }
