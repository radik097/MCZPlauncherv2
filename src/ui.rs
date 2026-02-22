use iced::{Element, Container, Column, Row, Text, Button, Length, Padding, alignment};
use crate::Message;

pub struct ModpackCard {
    pub name: String,
    pub description: String,
    pub version: String,
}

impl ModpackCard {
    pub fn new(name: String, description: String, version: String) -> Self {
        ModpackCard {
            name,
            description,
            version,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let content = Column::new()
            .padding(10)
            .spacing(5)
            .push(
                Text::new(&self.name)
                    .size(16)
            )
            .push(
                Text::new(&self.description)
                    .size(12)
            )
            .push(
                Text::new(format!("v{}", self.version))
                    .size(10)
            );

        Container::new(content)
            .padding(10)
            .into()
    }
}

pub fn create_header() -> Element<'static, Message> {
    Text::new("MCZ Launcher - NeoForge Modpack Manager")
        .size(24)
        .into()
}

pub fn create_status_bar(status: &str, progress: f32) -> Element<Message> {
    let content = Column::new()
        .spacing(5)
        .push(
            Text::new(status)
                .size(12)
        )
        .push(
            iced::ProgressBar::new(0.0..=1.0, progress)
                .height(Length::Units(4))
        );

    Container::new(content)
        .padding(10)
        .width(Length::Fill)
        .into()
}
