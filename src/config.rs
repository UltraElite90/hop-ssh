use crate::host::Host;
use anyhow::{Context, Result};
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

    pub fn load() -> Result<Self> {
        let path = Self::path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config at {:?}", path))?;
        toml::from_str(&content).with_context(|| "Failed to parse config")
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    pub fn add_host(&mut self, host: Host) -> Result<()> {
        if self.hosts.contains_key(&host.name) {
            anyhow::bail!("Host '{}' already exists. Use `hop edit` to modify.", host.name);
        }
        self.hosts.insert(host.name.clone(), host);
        Ok(())
    }

    pub fn get_host(&self, name: &str) -> Result<&Host> {
        self.hosts.get(name)
            .with_context(|| format!("Host '{}' not found. Use `hop list` to see all hosts.", name))
    }

    pub fn get_host_mut(&mut self, name: &str) -> Result<&mut Host> {
        self.hosts.get_mut(name)
            .with_context(|| format!("Host '{}' not found.", name))
    }

    pub fn remove_host(&mut self, name: &str) -> Result<Host> {
        self.hosts.remove(name)
            .with_context(|| format!("Host '{}' not found.", name))
    }
}
