use crate::models::AppConfig;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct ConfigStore {
    file_path: PathBuf,
}

impl ConfigStore {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    pub fn load(&self) -> Result<AppConfig> {
        if !self.file_path.exists() {
            return Ok(AppConfig::default());
        }

        let content = fs::read_to_string(&self.file_path)
            .with_context(|| format!("Failed to read config file: {}", self.file_path.display()))?;

        if content.trim().is_empty() {
            return Ok(AppConfig::default());
        }

        let config: AppConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", self.file_path.display()))?;

        Ok(config)
    }

    pub fn save(&self, config: &AppConfig) -> Result<()> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        let content = toml::to_string_pretty(config)
            .context("Failed to serialize config")?;

        fs::write(&self.file_path, content)
            .with_context(|| format!("Failed to write config file: {}", self.file_path.display()))?;

        Ok(())
    }

    pub fn get_default_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .context("Could not determine config directory")?;
        path.push("termtask");
        path.push("config.toml");
        Ok(path)
    }
}