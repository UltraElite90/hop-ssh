use crate::error::HopError;
use crate::host::Host;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub hosts: HashMap<String, Host>,
}

impl Config {
    pub fn path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".hop")
            .join("config.toml")
    }

    pub fn load() -> Result<Self, HopError> {
        let path = Self::path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), HopError> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    pub fn add_host(&mut self, host: Host) -> Result<(), HopError> {
        if self.hosts.contains_key(&host.name) {
            return Err(HopError::HostExists(host.name));
        }
        self.hosts.insert(host.name.clone(), host);
        Ok(())
    }

    pub fn get_host(&self, name: &str) -> Result<&Host, HopError> {
        self.hosts
            .get(name)
            .ok_or_else(|| HopError::HostNotFound(name.to_string()))
    }

    pub fn get_host_mut(&mut self, name: &str) -> Result<&mut Host, HopError> {
        self.hosts
            .get_mut(name)
            .ok_or_else(|| HopError::HostNotFound(name.to_string()))
    }

    pub fn remove_host(&mut self, name: &str) -> Result<Host, HopError> {
        self.hosts
            .remove(name)
            .ok_or_else(|| HopError::HostNotFound(name.to_string()))
    }
}