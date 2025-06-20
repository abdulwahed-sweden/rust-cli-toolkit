use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Permission denied: {action}")]
    PermissionDenied { action: String },
    
    #[error("Network error: {message}")]
    NetworkError { message: String },
    
    #[error("Configuration error: {message}")]
    ConfigError { message: String },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
