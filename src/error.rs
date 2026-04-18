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
    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),
    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("Invalid port number: {0}")]
    InvalidPort(String),
    #[error("SSH command failed")]
    SshFailed,
}