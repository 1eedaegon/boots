pub struct Store {
    // Database connection will be added here
}

impl Store {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
