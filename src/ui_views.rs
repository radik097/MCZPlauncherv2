use iced::{alignment, Element, Length, Row, Column, Text, Button, TextInput, Padding};
use crate::Message;

impl crate::MCZLauncher {
    pub fn view_login(&self) -> Element<Message> {
        let content = Column::new()
            .padding(40)
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Text::new("MCZ Launcher")
                    .size(48)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
            )
            .push(
                Text::new("NeoForge Modpack Manager")
                    .size(18)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
            )
            .push(
                Column::new()
                    .width(Length::Fixed(400.0))
                    .spacing(15)
                    .padding(30)
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
                            &self.login_username,
                            Message::UsernameChanged
                        )
                        .padding(10)
                        .size(14)
                    )
                    .push(
                        Text::new("Password")
                            .size(12)
                    )
                    .push(
                        TextInput::new(
                            "Enter password",
                            &self.login_password,
                            Message::PasswordChanged
                        )
                        .padding(10)
                        .size(14)
                        .secure(true)
                    )
                    .push(
                        Button::new(
                            Text::new("Login")
                                .horizontal_alignment(alignment::Horizontal::Center)
                        )
                        .width(Length::Fill)
                        .padding(12)
                        .on_press(Message::LoginPressed)
                    )
                    .push(
                        Row::new()
                            .spacing(10)
                            .push(
                                Text::new("Don't have an account?")
                                    .size(12)
                            )
                            .push(
                                Button::new(
                                    Text::new("Register")
                                        .size(12)
                                )
                                .padding(5)
                                .on_press(Message::SwitchToRegister)
                            )
                    )
                    .push(
                        Text::new(&self.status_message)
                            .size(12)
                    )
            );

        iced::Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    pub fn view_register(&self) -> Element<Message> {
        let content = Column::new()
            .padding(40)
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Text::new("MCZ Launcher")
                    .size(48)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
            )
            .push(
                Text::new("Create Account")
                    .size(18)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
            )
            .push(
                Column::new()
                    .width(Length::Fixed(400.0))
                    .spacing(15)
                    .padding(30)
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
                            &self.login_username,
                            Message::UsernameChanged
                        )
                        .padding(10)
                        .size(14)
                    )
                    .push(
                        Text::new("Password (8+ chars)")
                            .size(12)
                    )
                    .push(
                        TextInput::new(
                            "Enter password",
                            &self.login_password,
                            Message::PasswordChanged
                        )
                        .padding(10)
                        .size(14)
                        .secure(true)
                    )
                    .push(
                        Button::new(
                            Text::new("Create Account")
                                .horizontal_alignment(alignment::Horizontal::Center)
                        )
                        .width(Length::Fill)
                        .padding(12)
                        .on_press(Message::RegisterPressed)
                    )
                    .push(
                        Row::new()
                            .spacing(10)
                            .push(
                                Text::new("Already have an account?")
                                    .size(12)
                            )
                            .push(
                                Button::new(
                                    Text::new("Login")
                                        .size(12)
                                )
                                .padding(5)
                                .on_press(Message::SwitchToLogin)
                            )
                    )
                    .push(
                        Text::new(&self.status_message)
                            .size(12)
                    )
            );

        iced::Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    pub fn view_launcher(&self) -> Element<Message> {
        let header = Row::new()
            .width(Length::Fill)
            .push(
                Text::new("MCZ Launcher - NeoForge Modpack Manager")
                    .size(24)
            )
            .push(
                Column::new()
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Right)
                    .push(
                        if let Some(user) = &self.current_user {
                            Row::new()
                                .push(
                                    Text::new(format!("Logged in as: {}", user))
                                        .size(12)
                                )
                                .push(
                                    Button::new(Text::new("Logout"))
                                        .padding(5)
                                        .on_press(Message::Logout)
                                )
                                .into()
                        } else {
                            Row::new().into()
                        }
                    )
            );

        let content = Column::new()
            .padding(20)
            .spacing(15)
            .push(header)
            .push(
                Row::new()
                    .spacing(15)
                    .push(
                        Column::new()
                            .width(Length::FillPortion(1))
                            .spacing(10)
                            .push(Text::new("Available Modpacks").size(18))
                            .push(
                                Column::new()
                                    .spacing(10)
                                    .push(
                                        Button::new(
                                            Text::new("Vanilla Plus")
                                                .horizontal_alignment(alignment::Horizontal::Center)
                                        )
                                        .width(Length::Fill)
                                        .padding(10)
                                        .on_press(Message::SelectModpack("Vanilla Plus".to_string()))
                                    )
                                    .push(
                                        Button::new(
                                            Text::new("Tech Modpack")
                                                .horizontal_alignment(alignment::Horizontal::Center)
                                        )
                                        .width(Length::Fill)
                                        .padding(10)
                                        .on_press(Message::SelectModpack("Tech Modpack".to_string()))
                                    )
                            )
                            .push(Text::new("Available Servers").size(18))
                            .push(
                                Column::new()
                                    .spacing(10)
                                    .push(
                                        Column::new()
                                            .spacing(5)
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
                                            .spacing(5)
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
                            .width(Length::FillPortion(1))
                            .spacing(10)
                            .push(Text::new("Game Info").size(18))
                            .push(
                                Column::new()
                                    .spacing(8)
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
                                    .spacing(8)
                                    .push(Text::new("Status:").size(12))
                                    .push(
                                        Text::new(&self.status_message)
                                            .size(12)
                                    )
                            )
                            .push(
                                if self.progress > 0.0 && self.progress < 1.0 {
                                    iced::ProgressBar::new(0.0..=1.0, self.progress)
                                } else {
                                    iced::ProgressBar::new(0.0..=1.0, 0.0)
                                }
                            )
                            .push(
                                Button::new(
                                    Text::new("Launch Game")
                                        .horizontal_alignment(alignment::Horizontal::Center)
                                )
                                .width(Length::Fill)
                                .padding(15)
                                .on_press(Message::LaunchGame)
                            )
                    )
            );

        iced::Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::new(20))
            .into()
    }

    pub fn view_launching(&self) -> Element<Message> {
        let content = Column::new()
            .padding(20)
            .spacing(15)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .push(
                Text::new("Launching Game...")
                    .size(32)
            )
            .push(
                iced::ProgressBar::new(0.0..=1.0, self.progress)
                    .width(Length::Fixed(400.0))
            )
            .push(
                Text::new(&self.status_message)
                    .size(14)
            );

        iced::Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    pub fn view_error(&self) -> Element<Message> {
        let content = Column::new()
            .padding(40)
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
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
                .width(Length::Fixed(200.0))
                .padding(12)
                .on_press(Message::SwitchToLogin)
            );

        iced::Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
