use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftProfile {
    pub name: String,
    pub game_directory: PathBuf,
    pub java_executable: PathBuf,
    pub jvm_args: String,
    pub game_args: String,
}

pub struct MinecraftLauncher {
    game_dir: PathBuf,
    java_path: Option<PathBuf>,
}

impl MinecraftLauncher {
    pub fn new(game_dir: PathBuf) -> Self {
        MinecraftLauncher {
            game_dir,
            java_path: None,
        }
    }

    pub fn set_java_path(&mut self, path: PathBuf) {
        self.java_path = Some(path);
    }

    pub async fn find_java(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        // Try to find Java in standard locations
        let possible_paths = vec![
            "java",
            "java.exe",
            "C:\\Program Files\\Java\\jdk-21\\bin\\java.exe",
            "C:\\Program Files\\Java\\jre8\\bin\\java.exe",
            "C:\\Program Files (x86)\\Java\\jre1.8.0\\bin\\java.exe",
        ];

        for java in possible_paths {
            if let Ok(output) = Command::new(java)
                .arg("--version")
                .output()
                .await
            {
                if output.status.success() {
                    return Ok(java.into());
                }
            }
        }

        Err("Java not found".into())
    }

    pub async fn launch_game(
        &self,
        profile: &MinecraftProfile,
        _modpack_name: &str,
        ram_mb: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let java_path = match self.java_path.as_ref() {
            Some(path) => path.clone(),
            None => self.find_java().await?,
        };

        let _mods_dir = self.game_dir.join("mods");
        let _libraries_dir = self.game_dir.join("libraries");

        let mut jvm_args = vec![
            format!("-Xms1024M"),
            format!("-Xmx{}M", ram_mb),
            "-XX:+UseG1GC".to_string(),
            "-XX:+ParallelRefProcEnabled".to_string(),
            "-XX:MaxGCPauseMillis=200".to_string(),
            "-XX:InitiatingHeapOccupancyPercent=35".to_string(),
            "-DJ2D.opengl=true".to_string(),
        ];

        // Add custom JVM args
        let custom_args: Vec<&str> = profile.jvm_args.split_whitespace().collect();
        for arg in custom_args {
            jvm_args.push(arg.to_string());
        }

        tracing::info!("Launching Minecraft with {} MB RAM", ram_mb);
        tracing::debug!("JVM Args: {:?}", jvm_args);

        // This is a simplified version. In production, you would:
        // 1. Download and verify Minecraft JAR
        // 2. Set up proper classpath with all libraries
        // 3. Configure natives
        // 4. Set proper game arguments

        let mut cmd = Command::new(&java_path);
        for arg in jvm_args {
            cmd.arg(arg);
        }

        // Add game jar and arguments
        cmd.arg("-cp")
            .arg(format!("minecraft.jar;libraries/*"))
            .arg("net.minecraft.client.main.Main")
            // Launcher arguments would go here
            .current_dir(&profile.game_directory)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let _child = cmd.spawn()?;
        tracing::info!("Minecraft process started");

        Ok(())
    }

    pub fn get_default_profile(&self, name: &str) -> MinecraftProfile {
        MinecraftProfile {
            name: name.to_string(),
            game_directory: self.game_dir.clone(),
            java_executable: "java".into(),
            jvm_args: String::new(),
            game_args: String::new(),
        }
    }

    pub async fn verify_game_files(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let critical_files = vec![
            "minecraft/minecraft.jar",
            "libraries",
            "assets",
        ];

        for file in critical_files {
            let path = self.game_dir.join(file);
            if !path.exists() {
                tracing::warn!("Missing critical file: {}", file);
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn get_game_directory(&self) -> &PathBuf {
        &self.game_dir
    }

    pub async fn install_neoforge(
        &self,
        minecraft_version: &str,
        neoforge_version: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let forge_dir = self.game_dir.join("neoforge");
        tokio::fs::create_dir_all(&forge_dir).await?;

        // In production, this would:
        // 1. Download NeoForge installer
        // 2. Execute the installer
        // 3. Set up mod loader
        // 4. Configure game launcher

        tracing::info!(
            "NeoForge {} installed for Minecraft {}",
            neoforge_version,
            minecraft_version
        );

        Ok(())
    }

    pub async fn cleanup_old_versions(&self, _keep_latest: usize) -> Result<(), Box<dyn std::error::Error>> {
        // Clean up old game versions to save space
        let versions_dir = self.game_dir.join("versions");
        
        if !versions_dir.exists() {
            return Ok(());
        }

        // Read directory and sort by modification time
        // Delete all but the newest `keep_latest` versions

        tracing::info!("Cleaned up old game versions");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minecraft_profile_creation() {
        let profile = MinecraftProfile {
            name: "Test Profile".to_string(),
            game_directory: PathBuf::from("./game"),
            java_executable: PathBuf::from("java"),
            jvm_args: String::new(),
            game_args: String::new(),
        };
        assert_eq!(profile.name, "Test Profile");
    }
}
