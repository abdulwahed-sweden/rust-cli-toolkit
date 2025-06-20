use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub general: GeneralConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub editor: String,
    pub temp_dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfig {
    pub colors: bool,
    pub icons: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                editor: "nano".to_string(),
                temp_dir: "/tmp".to_string(),
            },
            ui: UiConfig {
                colors: true,
                icons: true,
            },
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // Placeholder implementation
        Ok(AppConfig::default())
    }
    
    pub fn save(&self) -> Result<()> {
        // Placeholder implementation
        Ok(())
    }
}