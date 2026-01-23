use app_file_manager::{FileManager, FileManagerMessage};
use iced::{
    Alignment, Element, Length, Size,
    widget::{column, container, row, scrollable, text},
};

use crate::{ApplicationItem, ApplicationType};

#[derive(Debug, Clone)]
pub struct Compositor {
    pub open_apps: Vec<ApplicationItem>,
    file_manager: FileManager,
}

#[derive(Debug, Clone)]
pub enum CompositorMessage {
    FileManager(FileManagerMessage),
}

impl Compositor {
    pub fn new() -> Self {
        Self {
            open_apps: vec![],
            file_manager: FileManager::new(),
        }
    }

    pub fn view(&self, widown_size: Size) -> Element<'_, CompositorMessage> {
        let windows = scrollable(
            row(self.open_apps.iter().map(|app| match app.app_type {
                ApplicationType::FilesManager => container(column![
                    container(text("FileManager"))
                        .padding(8)
                        .width(Length::Fill)
                        .style(|_| container::Style {
                            background: Some(iced::Background::Color(iced::Color::from_rgb8(
                                167, 186, 186
                            ))),
                            ..Default::default()
                        }),
                    self.file_manager.view().map(CompositorMessage::FileManager)
                ])
                .width((widown_size.width - 12.0) / 2.0)
                .height(widown_size.height / 1.3)
                .into(),
                _ => container(text("Not support")).into(),
            }))
            .spacing(12)
            .height(Length::Fill)
            .align_y(Alignment::Center),
        )
        .horizontal();

        container(windows).into()
    }
}
