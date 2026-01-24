use iced::{Font, widget::image};

pub const MIRA_FONT_NAME: Font = Font::with_name("Miracode");
pub const MIRA_FONT_BYTES: &[u8] = include_bytes!("../../assets/font/Miracode.ttf").as_slice();

#[derive(Debug, Clone)]
pub enum ApplicationType {
    FilesManager,
    Terminal,
    FireFox,
    Steam,
}

#[derive(Debug, Clone)]
pub struct ApplicationItem {
    pub name: String,
    pub icon: image::Handle,
    pub app_type: ApplicationType,
}
