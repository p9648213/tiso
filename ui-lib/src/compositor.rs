use app_file_manager::{FileManager, FileManagerMessage};
use iced::{
    Element, Size,
    widget::{container, row, scrollable, text},
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
                ApplicationType::FilesManager => {
                    container(self.file_manager.view().map(CompositorMessage::FileManager))
                        .width(widown_size.width / 2.0)
                        .height(widown_size.height / 1.3)
                        .into()
                }
                _ => container(text("Not support")).into(),
            }))
            .spacing(12),
        )
        .horizontal();

        container(windows).into()
    }
}
