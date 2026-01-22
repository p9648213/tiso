use iced::{
    Element, Length,
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
        container(text("File Manager!")).center(Length::Fill).into()
    }
}
