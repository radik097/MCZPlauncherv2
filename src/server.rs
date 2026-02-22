use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
    pub name: String,
    pub description: String,
    pub requires_authentication: bool,
    pub auth_server_url: Option<String>,
    pub neoforge_version: String,
    pub minecraft_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub online: bool,
    pub players_online: u32,
    pub max_players: u32,
    pub motd: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameLaunchConfig {
    pub username: String,
    pub session_token: String,
    pub server_address: String,
    pub server_port: u16,
    pub game_dir: PathBuf,
    pub java_executable: PathBuf,
    pub ram_mb: u32,
    pub modpack_name: String,
}

pub struct ServerConnectionManager {
    config_dir: PathBuf,
}

impl ServerConnectionManager {
    pub fn new(config_dir: PathBuf) -> Self {
        ServerConnectionManager { config_dir }
    }

    pub async fn load_servers(&self) -> Result<Vec<ServerConfig>, Box<dyn std::error::Error>> {
        let servers_file = self.config_dir.join("servers.json");

        if !servers_file.exists() {
            return Ok(vec![
                ServerConfig {
                    address: "localhost".to_string(),
                    port: 25565,
                    name: "Local Server".to_string(),
                    description: "Local NeoForge server".to_string(),
                    requires_authentication: true,
                    auth_server_url: Some("http://localhost:8080".to_string()),
                    neoforge_version: "0.0.47".to_string(),
                    minecraft_version: "1.21.1".to_string(),
                },
                ServerConfig {
                    address: "play.example.com".to_string(),
                    port: 25565,
                    name: "Community Server".to_string(),
                    description: "Public NeoForge community server".to_string(),
                    requires_authentication: true,
                    auth_server_url: Some("https://auth.example.com".to_string()),
                    neoforge_version: "0.0.47".to_string(),
                    minecraft_version: "1.21.1".to_string(),
                },
            ]);
        }

        let content = tokio::fs::read_to_string(&servers_file).await?;
        let servers: Vec<ServerConfig> = serde_json::from_str(&content)?;
        Ok(servers)
    }

    pub async fn save_servers(
        &self,
        servers: &[ServerConfig],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let servers_file = self.config_dir.join("servers.json");
        let content = serde_json::to_string_pretty(servers)?;
        tokio::fs::write(&servers_file, content).await?;
        Ok(())
    }

    pub async fn add_server(&self, server: ServerConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut servers = self.load_servers().await?;
        servers.push(server);
        self.save_servers(&servers).await?;
        Ok(())
    }

    pub async fn remove_server(&self, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut servers = self.load_servers().await?;
        servers.retain(|s| s.address != address);
        self.save_servers(&servers).await?;
        Ok(())
    }

    pub async fn check_server_status(
        &self,
        server: &ServerConfig,
    ) -> Result<ServerStatus, Box<dyn std::error::Error>> {
        let _client = tokio::net::TcpStream::connect(format!("{}:{}", server.address, server.port))
            .await?;

        // Simplified status check - in production would use Minecraft server list ping protocol
        Ok(ServerStatus {
            online: true,
            players_online: 5,
            max_players: 20,
            motd: format!("Welcome to {}", server.name),
            version: server.minecraft_version.clone(),
        })
    }

    pub async fn prepare_launch_config(
        &self,
        username: &str,
        session_token: &str,
        server: &ServerConfig,
        modpack_name: &str,
        game_dir: PathBuf,
        java_path: PathBuf,
        ram_mb: u32,
    ) -> Result<GameLaunchConfig, Box<dyn std::error::Error>> {
        // Create launch configuration file with auth credentials
        let launch_config = GameLaunchConfig {
            username: username.to_string(),
            session_token: session_token.to_string(),
            server_address: server.address.clone(),
            server_port: server.port,
            game_dir: game_dir.clone(),
            java_executable: java_path,
            ram_mb,
            modpack_name: modpack_name.to_string(),
        };

        // Save launch config to file for NeoForge mod to read
        let launch_file = game_dir.join("launch_config.json");
        let content = serde_json::to_string_pretty(&launch_config)?;
        tokio::fs::write(&launch_file, content).await?;

        tracing::info!("prepared launch config for user: {} to server: {}", username, server.address);
        Ok(launch_config)
    }

    pub fn generate_server_properties(
        &self,
        server: &ServerConfig,
        output_dir: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let properties = format!(
            "# MCZ Server Configuration\n\
             server-name={}\n\
             server-port={}\n\
             online-mode=false\n\
             enable-query=true\n\
             query.port={}\n\
             difficulty=2\n\
             max-players=20\n\
             view-distance=10\n\
             simulation-distance=10\n",
            server.name, server.port, server.port
        );

        let properties_file = output_dir.join("server.properties");
        std::fs::write(properties_file, properties)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_creation() {
        let server = ServerConfig {
            address: "localhost".to_string(),
            port: 25565,
            name: "Test Server".to_string(),
            description: "Test description".to_string(),
            requires_authentication: true,
            auth_server_url: Some("http://localhost:8080".to_string()),
            neoforge_version: "0.0.47".to_string(),
            minecraft_version: "1.21.1".to_string(),
        };

        assert_eq!(server.address, "localhost");
        assert_eq!(server.port, 25565);
    }
}
