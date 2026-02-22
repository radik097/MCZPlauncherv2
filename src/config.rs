use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherConfig {
    pub game_directory: PathBuf,
    pub java_path: Option<PathBuf>,
    pub java_args: String,
    pub default_ram_mb: u32,
    pub max_ram_mb: u32,
    pub auto_download: bool,
    pub auto_update: bool,
    pub keep_launcher_open: bool,
    pub theme: String,
    pub language: String,
}

impl Default for LauncherConfig {
    fn default() -> Self {
        let _config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("MCZPlauncher");

        LauncherConfig {
            game_directory: dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("MCZPlauncher"),
            java_path: None,
            java_args: "-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200"
                .to_string(),
            default_ram_mb: 2048,
            max_ram_mb: 4096,
            auto_download: true,
            auto_update: true,
            keep_launcher_open: false,
            theme: "dark".to_string(),
            language: "en".to_string(),
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("MCZPlauncher");

        let config_path = config_dir.join("config.json");

        ConfigManager { config_path }
    }

    pub fn with_path(path: PathBuf) -> Self {
        ConfigManager {
            config_path: path,
        }
    }

    pub async fn load_config(&self) -> Result<LauncherConfig, Box<dyn std::error::Error>> {
        if !self.config_path.exists() {
            let config = LauncherConfig::default();
            self.save_config(&config).await?;
            return Ok(config);
        }

        let content = tokio::fs::read_to_string(&self.config_path).await?;
        let config: LauncherConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub async fn save_config(&self, config: &LauncherConfig) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = self.config_path.parent().ok_or("Invalid config path")?;
        tokio::fs::create_dir_all(config_dir).await?;
        let content = serde_json::to_string_pretty(config)?;
        tokio::fs::write(&self.config_path, content).await?;
        Ok(())
    }

    pub async fn reset_to_defaults(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = LauncherConfig::default();
        self.save_config(&config).await?;
        Ok(())
    }
}

impl LauncherConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.default_ram_mb > self.max_ram_mb {
            return Err("Default RAM cannot exceed max RAM".to_string());
        }

        if self.default_ram_mb < 512 {
            return Err("Minimum RAM should be at least 512 MB".to_string());
        }

        if self.max_ram_mb > 32768 {
            return Err("Maximum RAM should not exceed 32 GB".to_string());
        }

        Ok(())
    }

    pub fn update_ram_settings(&mut self, default: u32, max: u32) -> Result<(), String> {
        self.default_ram_mb = default;
        self.max_ram_mb = max;
        self.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LauncherConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation() {
        let mut config = LauncherConfig::default();
        config.default_ram_mb = 8192;
        config.max_ram_mb = 4096;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_update_ram_settings() {
        let mut config = LauncherConfig::default();
        assert!(config.update_ram_settings(2048, 4096).is_ok());
        assert!(config.update_ram_settings(4096, 2048).is_err());
    }
}
