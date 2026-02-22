mod ui;
mod ui_views;
mod download;
mod modpack;
mod minecraft;
mod config;
mod auth;
mod server;

use iced::{
    executor, window, Application, Command, Element, Settings, Theme,
};
use iced::widget::text_input;
use std::path::PathBuf;

pub fn main() -> iced::Result {
    MCZLauncher::run(Settings {
        window: window::Settings {
            size: iced::Size {
                width: 1000.0,
                height: 700.0,
            },
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug)]
pub struct MCZLauncher {
    app_state: AppState,
    
    // Auth state
    login_username: String,
    login_password: String,
    reg_password_confirm: String,
    username_input_id: text_input::Id,
    password_input_id: text_input::Id,
    password_confirm_id: text_input::Id,
    
    // Main launcher state
    selected_modpack: Option<String>,
    progress: f32,
    status_message: String,
    game_dir: PathBuf,
    current_user: Option<String>,
    session_token: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppState {
    Login,
    Register,
    MainLauncher,
    Launching,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LauncherState {
    Idle,
    DownloadingMinecraft,
    DownloadingNeoForge,
    DownloadingMods,
    Installing,
    Ready,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Login messages
    UsernameChanged(String),
    PasswordChanged(String),
    PasswordConfirmChanged(String),
    LoginPressed,
    RegisterPressed,
    SwitchToRegister,
    SwitchToLogin,
    
    // Auth messages
    LoginSuccess(String, String), // username, session_token
    LoginFailed(String),
    RegisterSuccess,
    RegisterFailed(String),
    
    // Launcher messages
    SelectModpack(String),
    LaunchGame,
    UpdateProgress(f32),
    UpdateStatus(String),
    RefreshModpacks,
    Logout,
    Error(String),
}

impl Application for MCZLauncher {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let game_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("MCZPlauncher");

        let launcher = MCZLauncher {
            app_state: AppState::Login,
            login_username: String::new(),
            login_password: String::new(),
            reg_password_confirm: String::new(),
            username_input_id: text_input::Id::unique(),
            password_input_id: text_input::Id::unique(),
            password_confirm_id: text_input::Id::unique(),
            selected_modpack: None,
            progress: 0.0,
            status_message: "Ready to launch".to_string(),
            game_dir,
            current_user: None,
            session_token: None,
        };

        (launcher, Command::none())
    }

    fn title(&self) -> String {
        "MCZ Launcher - NeoForge Modpack Manager".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UsernameChanged(username) => {
                self.login_username = username;
                Command::none()
            }
            Message::PasswordConfirmChanged(password) => {
                self.reg_password_confirm = password;
                Command::none()
            }
                if !self.login_username.is_empty() && !self.login_password.is_empty() {
                    self.status_message = "Logging in...".to_string();
                    // In production, this would call async auth
                    self.app_state = AppState::MainLauncher;
                    self.current_user = Some(self.login_username.clone());
                    self.session_token = Some("sample_token_123".to_string());
                    Command::none()
                } else {
                    self.status_message = "Please enter username and password".to_string();
                    Command::none()
                }
            }
            Message::RegisterPressed => {
                if self.login_username.is_empty() {
                    self.status_message = "Please enter a username".to_string();
                    Command::none()
                } else if self.login_password.is_empty() {
                    self.status_message = "Please enter a password".to_string();
                    Command::none()
                } else if self.reg_password_confirm.is_empty() {
                    self.status_message = "Please confirm your password".to_string();
                    Command::none()
                } else if self.login_password != self.reg_password_confirm {
                    self.status_message = "Passwords do not match".to_string();
                    self.login_password.clear();
                    self.reg_password_confirm.clear();
                    Command::none()
                } else {
                    self.status_message = "Creating account...".to_string();
                    self.app_state = AppState::MainLauncher;
                    self.current_user = Some(self.login_username.clone());
                    self.session_token = Some("sample_token_123".to_string());
                    Command::none()
                }
            }
            Message::SwitchToRegister => {
                self.app_state = AppState::Register;
                self.login_username.clear();
                self.login_password.clear();
                self.reg_password_confirm.clear();
                self.status_message = String::new();
                Command::none()
            }
            Message::SwitchToLogin => {
                self.app_state = AppState::Login;
                self.login_username.clear();
                self.login_password.clear();
                self.reg_password_confirm.clear();
                self.status_message = String::new();
                Command::none()
            }
            Message::LoginSuccess(username, token) => {
                self.current_user = Some(username);
                self.session_token = Some(token);
                self.app_state = AppState::MainLauncher;
                self.status_message = "Login successful!".to_string();
                Command::none()
            }
            Message::LoginFailed(error) => {
                self.status_message = format!("Login failed: {}", error);
                Command::none()
            }
            Message::SelectModpack(name) => {
                self.selected_modpack = Some(name);
                self.status_message = format!("Selected: {}", self.selected_modpack.as_ref().unwrap());
                Command::none()
            }
            Message::LaunchGame => {
                if let Some(_modpack) = &self.selected_modpack {
                    self.app_state = AppState::Launching;
                    self.status_message = "Downloading Minecraft...".to_string();
                    Command::none()
                } else {
                    self.status_message = "Please select a modpack first".to_string();
                    Command::none()
                }
            }
            Message::UpdateProgress(progress) => {
                self.progress = progress;
                Command::none()
            }
            Message::UpdateStatus(status) => {
                self.status_message = status;
                Command::none()
            }
            Message::Logout => {
                self.current_user = None;
                self.session_token = None;
                self.app_state = AppState::Login;
                self.login_username.clear();
                self.login_password.clear();
                self.status_message = "Logged out successfully".to_string();
                Command::none()
            }
            Message::Error(error) => {
                self.app_state = AppState::Error;
                self.status_message = format!("Error: {}", error);
                Command::none()
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        match self.app_state {
            AppState::Login => self.view_login(),
            AppState::Register => self.view_register(),
            AppState::MainLauncher => self.view_launcher(),
            AppState::Launching => self.view_launching(),
            AppState::Error => self.view_error(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
