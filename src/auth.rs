use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub auth_server_url: String,
    pub server_address: String,
    pub server_port: u16,
    pub session_token: Option<String>,
    pub last_username: Option<String>,
    pub auto_login: bool,
}

impl Default for AuthConfig {
    fn default() -> Self {
        AuthConfig {
            auth_server_url: "http://localhost:8080".to_string(),
            server_address: "localhost".to_string(),
            server_port: 25565,
            session_token: None,
            last_username: None,
            auto_login: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub session_token: Option<String>,
    pub error: Option<String>,
    pub user_uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
    pub user_uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub username: String,
    pub user_uuid: String,
    pub session_token: String,
    pub created_at: u64,
    pub expires_at: u64,
}

pub struct AuthManager {
    config_path: PathBuf,
    auth_client: reqwest::Client,
}

impl AuthManager {
    pub fn new(config_path: PathBuf) -> Self {
        AuthManager {
            config_path,
            auth_client: reqwest::Client::new(),
        }
    }

    pub async fn load_auth_config(&self) -> Result<AuthConfig, Box<dyn std::error::Error>> {
        if !self.config_path.exists() {
            let config = AuthConfig::default();
            self.save_auth_config(&config).await?;
            return Ok(config);
        }

        let content = tokio::fs::read_to_string(&self.config_path).await?;
        let config: AuthConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub async fn save_auth_config(&self, config: &AuthConfig) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = self.config_path.parent().ok_or("Invalid config path")?;
        tokio::fs::create_dir_all(config_dir).await?;
        let content = serde_json::to_string_pretty(config)?;
        tokio::fs::write(&self.config_path, content).await?;
        Ok(())
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
        auth_url: &str,
    ) -> Result<LoginResponse, Box<dyn std::error::Error>> {
        let request = LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        };

        let response = self.auth_client
            .post(format!("{}/api/login", auth_url))
            .json(&request)
            .send()
            .await?;

        let login_response: LoginResponse = response.json().await?;
        
        if login_response.success {
            tracing::info!("Login successful for user: {}", username);
        } else {
            tracing::warn!("Login failed for user: {}", username);
        }

        Ok(login_response)
    }

    pub async fn register(
        &self,
        username: &str,
        password: &str,
        email: Option<&str>,
        auth_url: &str,
    ) -> Result<RegisterResponse, Box<dyn std::error::Error>> {
        if !Self::validate_username(username) {
            return Ok(RegisterResponse {
                success: false,
                message: "Invalid username format".to_string(),
                user_uuid: None,
            });
        }

        if !Self::validate_password(password) {
            return Ok(RegisterResponse {
                success: false,
                message: "Password must be at least 8 characters".to_string(),
                user_uuid: None,
            });
        }

        let request = RegisterRequest {
            username: username.to_string(),
            password: password.to_string(),
            email: email.map(|e| e.to_string()),
        };

        let response = self.auth_client
            .post(format!("{}/api/register", auth_url))
            .json(&request)
            .send()
            .await?;

        let register_response: RegisterResponse = response.json().await?;
        
        if register_response.success {
            tracing::info!("Registration successful for user: {}", username);
        } else {
            tracing::warn!("Registration failed for user: {}", username);
        }

        Ok(register_response)
    }

    pub async fn verify_session(
        &self,
        session_token: &str,
        auth_url: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let response = self.auth_client
            .post(format!("{}/api/verify", auth_url))
            .bearer_auth(session_token)
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    pub async fn logout(
        &self,
        session_token: &str,
        auth_url: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.auth_client
            .post(format!("{}/api/logout", auth_url))
            .bearer_auth(session_token)
            .send()
            .await?;

        tracing::info!("User logged out");
        Ok(())
    }

    fn validate_username(username: &str) -> bool {
        username.len() >= 3 && username.len() <= 16 && 
        username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    }

    fn validate_password(password: &str) -> bool {
        password.len() >= 8
    }

    pub fn is_session_valid(session: &SessionInfo) -> bool {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        current_time < session.expires_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        assert!(AuthManager::validate_username("player123"));
        assert!(AuthManager::validate_username("player_name"));
        assert!(!AuthManager::validate_username("ab")); // too short
        assert!(!AuthManager::validate_username("player@name")); // invalid char
    }

    #[test]
    fn test_validate_password() {
        assert!(AuthManager::validate_password("password123"));
        assert!(AuthManager::validate_password("Pass@Word#1"));
        assert!(!AuthManager::validate_password("short")); // too short
    }
}
