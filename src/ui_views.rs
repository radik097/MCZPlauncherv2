use iced::{alignment, Element};
use iced::widget::{Row, Column, Text, Button, TextInput, Container, ProgressBar};
use crate::Message;

impl crate::MCZLauncher {
    pub fn view_login(&self) -> Element<Message> {
        let content = Column::new()
            .padding(40.0)
            .spacing(20.0)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .push(
                Text::new("MCZ Launcher")
                    .size(48)
                    .width(iced::Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
            )
            .push(
                Text::new("NeoForge Modpack Manager")
                    .size(18)
                    .width(iced::Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
            )
            .push(
                Column::new()
                    .width(iced::Length::Fixed(400.0))
                    .spacing(15.0)
                    .padding(30.0)
                    .push(
                        Text::new("Login")
                            .size(28)
                    )
                    .push(
                        Text::new("Username")
                            .size(12)
                    )
                    .push(
                        TextInput::new(
                            "Enter username",
                            &self.login_username
                        )
                        .on_input(Message::UsernameChanged)
                        .padding(10.0)
                        .size(14)
                    )
                    .push(
                        Text::new("Password")
                            .size(12)
                    )
                    .push(
                        TextInput::new(
                            "Enter password",
                            &self.login_password
                        )
                        .on_input(Message::PasswordChanged)
                        .padding(10.0)
                        .size(14)
                        .secure(true)
                    )
                    .push(
                        Button::new(
                            Text::new("Login")
                                .horizontal_alignment(alignment::Horizontal::Center)
                        )
                        .width(iced::Length::Fill)
                        .padding(12.0)
                        .on_press(Message::LoginPressed)
                    )
                    .push(
                        Row::new()
                            .spacing(10.0)
                            .push(
                                Text::new("Don't have an account?")
                                    .size(12)
                            )
                            .push(
                                Button::new(
                                    Text::new("Register")
                                        .size(12)
                                )
                                .padding(5.0)
                                .on_press(Message::SwitchToRegister)
                            )
                    )
                    .push(
                        Text::new(&self.status_message)
                            .size(12)
                    )
            );

        Container::new(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    pub fn view_register(&self) -> Element<Message> {
        let content = Column::new()
            .padding(40.0)
            .spacing(20.0)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .push(
                Text::new("MCZ Launcher")
                    .size(48)
                    .width(iced::Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
            )
            .push(
                Text::new("Create Account")
                    .size(18)
                    .width(iced::Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
            )
            .push(
                Column::new()
                    .width(iced::Length::Fixed(400.0))
                    .spacing(15.0)
                    .padding(30.0)
                    .push(
                        Text::new("Register")
                            .size(28)
                    )
                    .push(
                        Text::new("Username (3-16 chars, alphanumeric, _, -)")
                            .size(12)
                    )
                    .push(
                        TextInput::new(
                            "Enter username",
                            &self.login_username
                        )
                        .on_input(Message::UsernameChanged)
                        .padding(10.0)
                        .size(14)
                    )
                    .push(
                        Text::new("Password (8+ chars)")
                            .size(12)
                    )
                    .push(
                        TextInput::new(
                            "Enter password",
                            &self.login_password
                        )
                        .on_input(Message::PasswordChanged)
                        .padding(10.0)
                        .size(14)
                        .secure(true)
                    )
                    .push(
                        Text::new("Confirm Password")
                            .size(12)
                    )
                    .push(
                        TextInput::new(
                            "Confirm password",
                            &self.reg_password_confirm
                        )
                        .on_input(Message::PasswordConfirmChanged)
                        .padding(10.0)
                        .size(14)
                        .secure(true)
                    )
                    .push(
                        Button::new(
                            Text::new("Create Account")
                                .horizontal_alignment(alignment::Horizontal::Center)
                        )
                        .width(iced::Length::Fill)
                        .padding(12.0)
                        .on_press(Message::RegisterPressed)
                    )
                    .push(
                        Row::new()
                            .spacing(10.0)
                            .push(
                                Text::new("Already have an account?")
                                    .size(12)
                            )
                            .push(
                                Button::new(
                                    Text::new("Login")
                                        .size(12)
                                )
                                .padding(5.0)
                                .on_press(Message::SwitchToLogin)
                            )
                    )
                    .push(
                        Text::new(&self.status_message)
                            .size(12)
                    )
            );

        Container::new(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    pub fn view_launcher(&self) -> Element<Message> {
        let mut header_content = Column::new()
            .width(iced::Length::Fill);
        
        if let Some(user) = &self.current_user {
            header_content = header_content.push(
                Row::new()
                    .push(
                        Text::new(format!("Logged in as: {}", user))
                            .size(12)
                    )
                    .push(
                        Button::new(Text::new("Logout"))
                            .padding(5.0)
                            .on_press(Message::Logout)
                    )
            );
        }

        let header = Row::new()
            .width(iced::Length::Fill)
            .push(
                Text::new("MCZ Launcher - NeoForge Modpack Manager")
                    .size(24)
            )
            .push(header_content);

        let content = Column::new()
            .padding(20.0)
            .spacing(15.0)
            .push(header)
            .push(
                Row::new()
                    .spacing(15.0)
                    .push(
                        Column::new()
                            .width(iced::Length::FillPortion(1))
                            .spacing(10.0)
                            .push(Text::new("Available Modpacks").size(18))
                            .push(
                                Column::new()
                                    .spacing(10.0)
                                    .push(
                                        Button::new(
                                            Text::new("Vanilla Plus")
                                                .horizontal_alignment(alignment::Horizontal::Center)
                                        )
                                        .width(iced::Length::Fill)
                                        .padding(10.0)
                                        .on_press(Message::SelectModpack("Vanilla Plus".to_string()))
                                    )
                                    .push(
                                        Button::new(
                                            Text::new("Tech Modpack")
                                                .horizontal_alignment(alignment::Horizontal::Center)
                                        )
                                        .width(iced::Length::Fill)
                                        .padding(10.0)
                                        .on_press(Message::SelectModpack("Tech Modpack".to_string()))
                                    )
                            )
                            .push(Text::new("Available Servers").size(18))
                            .push(
                                Column::new()
                                    .spacing(10.0)
                                    .push(
                                        Column::new()
                                            .spacing(5.0)
                                            .push(
                                                Text::new("Local Server")
                                                    .size(14)
                                            )
                                            .push(
                                                Text::new("localhost:25565")
                                                    .size(11)
                                            )
                                    )
                                    .push(
                                        Column::new()
                                            .spacing(5.0)
                                            .push(
                                                Text::new("Community Server")
                                                    .size(14)
                                            )
                                            .push(
                                                Text::new("play.example.com:25565")
                                                    .size(11)
                                            )
                                    )
                            )
                    )
                    .push(
                        Column::new()
                            .width(iced::Length::FillPortion(1))
                            .spacing(10.0)
                            .push(Text::new("Game Info").size(18))
                            .push(
                                Column::new()
                                    .spacing(8.0)
                                    .push(Text::new("Selected Modpack:").size(12))
                                    .push(
                                        Text::new(
                                            self.selected_modpack
                                                .clone()
                                                .unwrap_or_else(|| "None".to_string())
                                        )
                                        .size(14)
                                    )
                            )
                            .push(
                                Column::new()
                                    .spacing(8.0)
                                    .push(Text::new("Status:").size(12))
                                    .push(
                                        Text::new(&self.status_message)
                                            .size(12)
                                    )
                            )
                            .push(
                                if self.progress > 0.0 && self.progress < 1.0 {
                                    ProgressBar::new(0.0..=1.0, self.progress)
                                } else {
                                    ProgressBar::new(0.0..=1.0, 0.0)
                                }
                            )
                            .push(
                                Button::new(
                                    Text::new("Launch Game")
                                        .horizontal_alignment(alignment::Horizontal::Center)
                                )
                                .width(iced::Length::Fill)
                                .padding(15.0)
                                .on_press(Message::LaunchGame)
                            )
                    )
            );

        Container::new(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(20.0)
            .into()
    }

    pub fn view_launching(&self) -> Element<Message> {
        let content = Column::new()
            .padding(20.0)
            .spacing(15.0)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .push(
                Text::new("Launching Game...")
                    .size(32)
            )
            .push(
                ProgressBar::new(0.0..=1.0, self.progress)
                    .width(iced::Length::Fixed(400.0))
            )
            .push(
                Text::new(&self.status_message)
                    .size(14)
            );

        Container::new(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    pub fn view_error(&self) -> Element<Message> {
        let content = Column::new()
            .padding(40.0)
            .spacing(20.0)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .push(
                Text::new("Error")
                    .size(32)
            )
            .push(
                Text::new(&self.status_message)
                    .size(14)
            )
            .push(
                Button::new(
                    Text::new("Return to Login")
                        .horizontal_alignment(alignment::Horizontal::Center)
                )
                .width(iced::Length::Fixed(200.0))
                .padding(12.0)
                .on_press(Message::SwitchToLogin)
            );

        Container::new(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
