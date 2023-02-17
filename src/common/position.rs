#[derive(Debug, Clone)]
pub struct Position {
    pub source_path: String,
    pub row: usize,
}

impl Position {
    pub fn new(source_path: String, row: usize) -> Self {
        Self { source_path, row }
    }
}
