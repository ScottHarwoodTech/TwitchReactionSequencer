#[derive(Debug, Clone)]
pub enum LoadError {
    FileError,
    FormatError(String),
}

#[derive(Debug, Clone)]
pub enum SaveError {
    FormatError(String),
}
