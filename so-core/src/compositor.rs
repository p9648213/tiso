use std::collections::HashMap;

use app_file_manager::{FilesManager, FilesManagerMessage};
use iced::{
    Alignment, Element, Length, Size, Task,
    widget::{column, container, row, scrollable, text},
};

use so_base::{ApplicationItem, ApplicationType};

#[derive(Debug)]
pub struct Compositor {
    app_id: u32,
    pub opening_apps: Vec<(u32, ApplicationItem)>,
    files_manager: HashMap<u32, FilesManager>,
}

#[derive(Debug, Clone)]
pub enum CompositorMessage {
    FilesManager(u32, FilesManagerMessage),
    OpenApplication(ApplicationItem),
}

impl Compositor {
    pub fn new() -> Self {
        Self {
            app_id: 0,
            opening_apps: vec![],
            files_manager: HashMap::new(),
        }
    }

    pub fn update(&mut self, message: CompositorMessage) -> Task<CompositorMessage> {
        match message {
            CompositorMessage::OpenApplication(app_item) => {
                let current_id = self.app_id;

                self.opening_apps.push((self.app_id, app_item.clone()));

                let mut command = Task::none();

                match app_item.app_type {
                    ApplicationType::FilesManager => {
                        let (files_manager, fm_task) = FilesManager::new();
                        self.files_manager.insert(self.app_id, files_manager);
                        command = fm_task
                            .map(move |msg| CompositorMessage::FilesManager(current_id, msg));
                    }
                    _ => {}
                }

                self.app_id += 1;

                command
            }
            CompositorMessage::FilesManager(id, fm_msg) => {
                if let Some(fm) = self.files_manager.get_mut(&id) {
                    fm.update(fm_msg)
                        .map(move |msg| CompositorMessage::FilesManager(id, msg))
                } else {
                    Task::none()
                }
            }
        }
    }

    pub fn view(&self, widown_size: Size) -> Element<'_, CompositorMessage> {
        let windows = scrollable(
            row(self
                .opening_apps
                .iter()
                .map(|(app_id, app)| match app.app_type {
                    ApplicationType::FilesManager => {
                        if let Some(files_manager) = self.files_manager.get(app_id) {
                            container(column![
                                container(text(format!("FileManager {}", app_id)))
                                    .padding(8)
                                    .width(Length::Fill)
                                    .style(|_| container::Style {
                                        background: Some(iced::Background::Color(
                                            iced::Color::from_rgb8(167, 186, 186)
                                        )),
                                        ..Default::default()
                                    }),
                                files_manager
                                    .view()
                                    .map(move |msg| CompositorMessage::FilesManager(*app_id, msg))
                            ])
                            .width((widown_size.width - 12.0) / 2.0)
                            .height(widown_size.height / 1.3)
                            .into()
                        } else {
                            container("Error")
                                .width((widown_size.width - 12.0) / 2.0)
                                .height(widown_size.height / 1.3)
                                .into()
                        }
                    }
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
