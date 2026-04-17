use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Host {
    pub name: String,
    pub hostname: String,
    pub user: String,
    pub port: u16,
    pub identity: Option<String>,
    pub tags: Vec<String>,
    pub group: Option<String>,
}

impl Host {
    pub fn new(name: &str, hostname: &str, user: &str, port: u16) -> Self {
        Self {
            name: name.to_string(),
            hostname: hostname.to_string(),
            user: user.to_string(),
            port,
            identity: None,
            tags: vec![],
            group: None,
        }
    }

    pub fn connection_string(&self) -> String {
        format!("{}@{}", self.user, self.hostname)
    }
}
