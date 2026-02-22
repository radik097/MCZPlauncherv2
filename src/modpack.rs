use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modpack {
    pub name: String,
    pub version: String,
    pub minecraft_version: String,
    pub neoforge_version: String,
    pub description: String,
    pub author: String,
    pub mods: Vec<ModEntry>,
    pub settings: ModpackSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModEntry {
    pub name: String,
    pub download_url: String,
    pub version: String,
    pub required: bool,
    pub filename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModpackSettings {
    pub ram_min: u32,      // MB
    pub ram_max: u32,      // MB
    pub java_args: String,
    pub custom_args: Option<Vec<String>>,
}

impl Default for ModpackSettings {
    fn default() -> Self {
        ModpackSettings {
            ram_min: 2048,
            ram_max: 4096,
            java_args: String::new(),
            custom_args: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModpackList {
    pub modpacks: Vec<Modpack>,
}

pub struct ModpackManager {
    config_dir: PathBuf,
}

impl ModpackManager {
    pub fn new(config_dir: PathBuf) -> Self {
        ModpackManager { config_dir }
    }

    pub async fn load_modpack_list(&self) -> Result<ModpackList, Box<dyn std::error::Error>> {
        let list_file = self.config_dir.join("modpacks.json");

        if !list_file.exists() {
            return Ok(ModpackList {
                modpacks: vec![
                    self.create_default_modpack("Vanilla Plus", "1.20.1", "0.0.47"),
                    self.create_default_modpack("Tech Modpack", "1.20.1", "0.0.47"),
                ],
            });
        }

        let content = tokio::fs::read_to_string(&list_file).await?;
        let list: ModpackList = serde_json::from_str(&content)?;
        Ok(list)
    }

    pub async fn save_modpack_list(&self, list: &ModpackList) -> Result<(), Box<dyn std::error::Error>> {
        tokio::fs::create_dir_all(&self.config_dir).await?;
        let list_file = self.config_dir.join("modpacks.json");
        let content = serde_json::to_string_pretty(list)?;
        tokio::fs::write(&list_file, content).await?;
        Ok(())
    }

    pub async fn load_modpack(&self, name: &str) -> Result<Modpack, Box<dyn std::error::Error>> {
        let list = self.load_modpack_list().await?;
        list.modpacks
            .into_iter()
            .find(|mp| mp.name == name)
            .ok_or_else(|| format!("Modpack {} not found", name).into())
    }

    pub async fn add_modpack(&self, modpack: Modpack) -> Result<(), Box<dyn std::error::Error>> {
        let mut list = self.load_modpack_list().await?;
        list.modpacks.push(modpack);
        self.save_modpack_list(&list).await?;
        Ok(())
    }

    pub async fn remove_modpack(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut list = self.load_modpack_list().await?;
        list.modpacks.retain(|mp| mp.name != name);
        self.save_modpack_list(&list).await?;
        Ok(())
    }

    fn create_default_modpack(&self, name: &str, mc_version: &str, forge_version: &str) -> Modpack {
        Modpack {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            minecraft_version: mc_version.to_string(),
            neoforge_version: forge_version.to_string(),
            description: format!("{} for Minecraft {}", name, mc_version),
            author: "MCZ Team".to_string(),
            mods: vec![],
            settings: ModpackSettings::default(),
        }
    }

    pub fn get_modpack_dir(&self, modpack_name: &str) -> PathBuf {
        self.config_dir
            .parent()
            .unwrap_or(&self.config_dir)
            .join("modpacks")
            .join(modpack_name)
    }

    /// Load a Modrinth modpack from a .mrpack file
    pub async fn load_modrinth_modpack(
        &self,
        mrpack_path: &Path,
    ) -> Result<(crate::modrinth::ModrinthModpack, Modpack), Box<dyn std::error::Error>> {
        // Parse the .mrpack file
        let modrinth_pack = crate::modrinth::parse_mrpack(mrpack_path)?;
        
        // Convert to our Modpack format
        let mod_entries = crate::modrinth::get_client_mods(&modrinth_pack)
            .into_iter()
            .map(|(path, url, hash)| ModEntry {
                name: path.clone(),
                download_url: url,
                version: modrinth_pack.index.version_id.clone(),
                required: true,
                filename: std::path::Path::new(&path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
            })
            .collect();

        let modpack = Modpack {
            name: modrinth_pack.index.name.clone(),
            version: modrinth_pack.index.version_id.clone(),
            minecraft_version: modrinth_pack.index.game.clone(),
            neoforge_version: "auto".to_string(),
            description: modrinth_pack.index.summary.clone().unwrap_or_default(),
            author: "Modrinth".to_string(),
            mods: mod_entries,
            settings: ModpackSettings::default(),
        };

        tracing::info!(
            "Loaded Modrinth modpack: {} v{}",
            modpack.name,
            modpack.version
        );

        Ok((modrinth_pack, modpack))
    }

    /// Extract and install a Modrinth modpack to the game directory
    pub async fn install_modrinth_modpack(
        &self,
        modrinth_pack: &crate::modrinth::ModrinthModpack,
        target_dir: &Path,
        client_side: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Extract override files
        crate::modrinth::extract_overrides(modrinth_pack, target_dir, client_side)?;

        // Download mods with verification
        let mods = if client_side {
            crate::modrinth::get_client_mods(modrinth_pack)
        } else {
            crate::modrinth::get_server_mods(modrinth_pack)
        };

        for (path, url, hash) in mods {
            let full_path = target_dir.join(&path);
            std::fs::create_dir_all(full_path.parent().unwrap_or(&target_dir))?;

            // Download and verify
            crate::mod_downloader::download_and_verify_mod(&url, &full_path, &hash, 3)
                .await?;
        }

        tracing::info!(
            "Installed Modrinth modpack to {:?}",
            target_dir
        );

        Ok(())
    }

    /// List all available Modrinth modpacks in the modpacks directory
    pub async fn list_modrinth_modpacks(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let modpacks_dir = self.config_dir
            .parent()
            .unwrap_or(&self.config_dir)
            .join("modrinth_modpacks");

        if !modpacks_dir.exists() {
            tokio::fs::create_dir_all(&modpacks_dir).await?;
            return Ok(Vec::new());
        }

        let mut modpacks = Vec::new();
        let mut entries = tokio::fs::read_dir(&modpacks_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("mrpack") {
                if let Some(name) = path.file_stem() {
                    modpacks.push(name.to_string_lossy().to_string());
                }
            }
        }

        Ok(modpacks)
    }
}
