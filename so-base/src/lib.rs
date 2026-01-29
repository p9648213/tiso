use iced::{Font, widget::image};

// General
pub const MIRA_FONT_NAME: Font = Font::with_name("Miracode");
pub const MIRA_FONT_BYTES: &[u8] = include_bytes!("../../assets/font/Miracode.ttf").as_slice();

// Files Manager
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

pub const FOLDER_ICON: &[u8] = include_bytes!("../../assets/images/folder.png").as_slice();
pub const FILE_ICON: &[u8] = include_bytes!("../../assets/images/file.png").as_slice();
pub const FILE_UNKNOWN_ICON: &[u8] = include_bytes!("../../assets/images/file-unknown.png").as_slice();

// Dock
pub const TERMINAL_ICON: &[u8] = include_bytes!("../../assets/images/terminal.png").as_slice();
pub const FIREFOX_ICON : &[u8] = include_bytes!("../../assets/images/firefox.png").as_slice();
pub const USERHOME_ICON: &[u8] = include_bytes!("../../assets/images/user-home.png").as_slice();
pub const STEAM_ICON: &[u8] = include_bytes!("../../assets/images/steam.png").as_slice();
