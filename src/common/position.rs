#[derive(Debug, Clone)]
pub struct Position {
    pub source_path: String,
    pub column: usize,
    pub row: usize,
}

impl Position {
    pub fn new(source_path: String, column: usize, row: usize) -> Self {
        Self {
            source_path,
            column,
            row,
        }
    }
}
