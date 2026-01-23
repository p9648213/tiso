use iced::{
    Background, Color, Element, Length,
    widget::{container, text},
};

#[derive(Debug, Clone)]
pub struct FileManager {}

#[derive(Debug, Clone)]
pub enum FileManagerMessage {}

impl FileManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<'_, FileManagerMessage> {
        container(text("File Manager!"))
            .center(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::WHITE)),
                text_color: Some(Color::BLACK),
                ..Default::default()
            })
            .into()
    }
}
