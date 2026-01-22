use iced::widget::image;

pub mod compositor;
pub mod dock;
pub mod panel;

#[derive(Debug, Clone)]
pub enum ApplicationType {
    FilesManager,
    Terminal,
    FireFox,
    Steam,
}

#[derive(Debug, Clone)]
pub struct ApplicationItem {
    name: String,
    icon: image::Handle,
    app_type: ApplicationType,
}
