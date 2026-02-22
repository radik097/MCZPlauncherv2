use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

/// The main Modrinth index file structure within a .mrpack archive
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthIndex {
    pub format_version: u32,
    pub game: String,
    pub version_id: String,
    pub name: String,
    pub summary: Option<String>,
    pub files: Vec<ModrinthFile>,
    pub dependencies: HashMap<String, String>,
}

/// Individual file entry in a Modrinth modpack
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthFile {
    pub path: String,
    pub hashes: Hashes,
    pub env: Option<EnvSupport>,
    pub downloads: Vec<String>,
    pub file_size: u64,
}

/// Hash verification info for modpack files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hashes {
    pub sha1: String,
    pub sha512: String,
}

/// Environment support specification (client/server)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvSupport {
    pub client: Option<String>, // "required", "optional", or "unsupported"
    pub server: Option<String>,
}

/// Parsed Modrinth modpack with extracted metadata and files
#[derive(Debug, Clone)]
pub struct ModrinthModpack {
    pub index: ModrinthIndex,
    pub overrides: HashMap<String, Vec<u8>>,
    pub client_overrides: HashMap<String, Vec<u8>>,
    pub server_overrides: HashMap<String, Vec<u8>>,
}

/// Parse a .mrpack file (which is a ZIP archive)
pub fn parse_mrpack(file_path: &Path) -> Result<ModrinthModpack, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut archive = ZipArchive::new(file)?;

    // Parse the main index file
    let index = {
        let mut index_file = archive.by_name("modrinth.index.json")?;
        let mut contents = String::new();
        index_file.read_to_string(&mut contents)?;
        serde_json::from_str::<ModrinthIndex>(&contents)?
    };

    tracing::info!(
        "Loading Modrinth modpack: {} v{}",
        index.name,
        index.version_id
    );

    // Extract override files
    let mut overrides = HashMap::new();
    let mut client_overrides = HashMap::new();
    let mut server_overrides = HashMap::new();

    let archive_len = archive.len();
    for i in 0..archive_len {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();
        let is_dir = file.is_dir();

        if !is_dir {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;

            if file_name.starts_with("overrides/") {
                let relative_path = file_name.strip_prefix("overrides/").unwrap();
                if !relative_path.is_empty() {
                    overrides.insert(relative_path.to_string(), contents);
                    tracing::debug!("Found override: {}", relative_path);
                }
            } else if file_name.starts_with("client-overrides/") {
                let relative_path = file_name.strip_prefix("client-overrides/").unwrap();
                if !relative_path.is_empty() {
                    client_overrides.insert(relative_path.to_string(), contents);
                    tracing::debug!("Found client override: {}", relative_path);
                }
            } else if file_name.starts_with("server-overrides/") {
                let relative_path = file_name.strip_prefix("server-overrides/").unwrap();
                if !relative_path.is_empty() {
                    server_overrides.insert(relative_path.to_string(), contents);
                    tracing::debug!("Found server override: {}", relative_path);
                }
            }
        }
    }

    Ok(ModrinthModpack {
        index,
        overrides,
        client_overrides,
        server_overrides,
    })
}

/// Get the download URLs for all client-side mods in the modpack
pub fn get_client_mods(modpack: &ModrinthModpack) -> Vec<(String, String, String)> {
    // Returns tuples of (path, download_url, sha512_hash)
    modpack
        .index
        .files
        .iter()
        .filter(|file| {
            // Include if not explicitly unsupported on client
            match &file.env {
                Some(env) => {
                    env.client.as_deref() != Some("unsupported")
                }
                None => true,
            }
        })
        .filter_map(|file| {
            file.downloads.first().map(|url| {
                (
                    file.path.clone(),
                    url.clone(),
                    file.hashes.sha512.clone(),
                )
            })
        })
        .collect()
}

/// Get the download URLs for server-side mods
pub fn get_server_mods(modpack: &ModrinthModpack) -> Vec<(String, String, String)> {
    // Returns tuples of (path, download_url, sha512_hash)
    modpack
        .index
        .files
        .iter()
        .filter(|file| {
            // Include if not explicitly unsupported on server
            match &file.env {
                Some(env) => {
                    env.server.as_deref() != Some("unsupported")
                }
                None => true,
            }
        })
        .filter_map(|file| {
            file.downloads.first().map(|url| {
                (
                    file.path.clone(),
                    url.clone(),
                    file.hashes.sha512.clone(),
                )
            })
        })
        .collect()
}

/// Extract override files to a target directory
pub fn extract_overrides(
    modpack: &ModrinthModpack,
    target_dir: &Path,
    client_side: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // First extract universal overrides
    for (relative_path, contents) in &modpack.overrides {
        let full_path = target_dir.join(relative_path);
        std::fs::create_dir_all(full_path.parent().unwrap())?;
        std::fs::write(&full_path, contents)?;
        tracing::info!("Extracted override: {}", relative_path);
    }

    // Then extract side-specific overrides
    let overrides = if client_side {
        &modpack.client_overrides
    } else {
        &modpack.server_overrides
    };

    for (relative_path, contents) in overrides {
        let full_path = target_dir.join(relative_path);
        std::fs::create_dir_all(full_path.parent().unwrap())?;
        std::fs::write(&full_path, contents)?;
        tracing::info!(
            "Extracted {} override: {}",
            if client_side { "client" } else { "server" },
            relative_path
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_filter() {
        let modpack = ModrinthModpack {
            index: ModrinthIndex {
                format_version: 1,
                game: "minecraft".to_string(),
                version_id: "1.20.1".to_string(),
                name: "Test Pack".to_string(),
                summary: None,
                files: vec![
                    ModrinthFile {
                        path: "mods/client.jar".to_string(),
                        hashes: Hashes {
                            sha1: "abc123".to_string(),
                            sha512: "def456".to_string(),
                        },
                        env: Some(EnvSupport {
                            client: Some("required".to_string()),
                            server: Some("unsupported".to_string()),
                        }),
                        downloads: vec!["https://example.com/client.jar".to_string()],
                        file_size: 1024,
                    },
                ],
                dependencies: HashMap::new(),
            },
            overrides: HashMap::new(),
            client_overrides: HashMap::new(),
            server_overrides: HashMap::new(),
        };

        let client_mods = get_client_mods(&modpack);
        assert_eq!(client_mods.len(), 1);

        let server_mods = get_server_mods(&modpack);
        assert_eq!(server_mods.len(), 0);
    }
}
