use iced::{
    Background, Color, Element, Length, Task,
    widget::{container, text},
};

#[derive(Debug)]
pub struct FilesManager {
    current_dir: String,
}

#[derive(Debug, Clone)]
pub enum FilesManagerMessage {
    FolderRead(String),
}

impl FilesManager {
    pub fn new() -> (Self, Task<FilesManagerMessage>) {
        (
            Self {
                current_dir: "/".to_string(),
            },
            Task::none(),
        )
    }

    pub fn view(&self) -> Element<'_, FilesManagerMessage> {
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

pub async fn read_folder() {
    ()
}
