use thiserror::Error;

#[derive(Error, Debug)]
pub enum HopError {
    #[error("Host '{0}' not found")]
    HostNotFound(String),
    #[error("Host '{0}' already exists")]
    HostExists(String),
    #[error("Config error: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
