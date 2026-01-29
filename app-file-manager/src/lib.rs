use iced::{
    Alignment, Background, Color, Element, Length, Padding, Task,
    widget::{MouseArea, column, container, image, scrollable, text},
};
use iced_aw::Wrap;
use so_base::{FILE_ICON, FILE_UNKNOWN_ICON, FOLDER_ICON};

#[derive(Debug, Clone)]
pub enum FileType {
    Directory,
    File,
    Symlink,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct FilePath(String);

#[derive(Debug, Clone)]
pub struct File {
    file_name: String,
    file_path: FilePath,
    file_type: FileType,
}

#[derive(Debug)]
pub struct FilesManager {
    current_dir: String,
    previous_dir: String,
    files: Vec<File>,
    last_error: Option<String>,
    folder_icon: image::Handle,
    file_icon: image::Handle,
    unknown_icon: image::Handle,
}

#[derive(Debug, Clone)]
pub enum FilesManagerMessage {
    ReadDir(Result<Vec<File>, String>),
    FileOpen(FilePath, FileType),
}

impl FilesManager {
    pub fn new() -> (Self, Task<FilesManagerMessage>) {
        (
            Self {
                current_dir: "/".to_string(),
                previous_dir: "/".to_string(),
                files: vec![],
                last_error: None,
                folder_icon: image::Handle::from_bytes(FOLDER_ICON),
                file_icon: image::Handle::from_bytes(FILE_ICON),
                unknown_icon: image::Handle::from_bytes(FILE_UNKNOWN_ICON),
            },
            Task::perform(read_dir("/".to_string()), FilesManagerMessage::ReadDir),
        )
    }

    pub fn update(&mut self, message: FilesManagerMessage) -> Task<FilesManagerMessage> {
        match message {
            FilesManagerMessage::ReadDir(result) => {
                match result {
                    Ok(files) => {
                        self.files = files;
                        self.last_error = None;
                    }
                    Err(error) => {
                        self.previous_dir = self.current_dir.clone();
                        self.files.clear();
                        self.last_error = Some(error);
                    }
                }
                Task::none()
            }
            FilesManagerMessage::FileOpen(path, file_type) => match file_type {
                FileType::Directory => {
                    self.previous_dir = self.current_dir.clone();
                    self.current_dir = path.0.clone();
                    Task::perform(read_dir(path.0), FilesManagerMessage::ReadDir)
                }
                _ => Task::none(),
            },
        }
    }

    pub fn view(&self) -> Element<'_, FilesManagerMessage> {
        if let Some(error) = &self.last_error {
            return container(
                text(format!("Error: {}", error)).color(Color::from_rgb8(209, 145, 145)),
            )
            .center(Length::Fill)
            .into();
        }

        let file_list = Wrap::with_elements(
            self.files
                .iter()
                .map(|file| {
                    let icon = match file.file_type {
                        FileType::Directory => self.folder_icon.clone(),
                        FileType::File => self.file_icon.clone(),
                        _ => self.unknown_icon.clone(),
                    };

                    MouseArea::new(
                        column![
                            image(icon).width(48).height(48),
                            text(file.file_name.clone())
                                .center()
                                .wrapping(text::Wrapping::WordOrGlyph)
                                .size(16)
                        ]
                        .width(Length::Fill)
                        .align_x(Alignment::Center)
                        .spacing(10)
                        .width(150),
                    )
                    .on_double_click(FilesManagerMessage::FileOpen(
                        file.file_path.clone(),
                        file.file_type.clone(),
                    ))
                    .into()
                })
                .collect(),
        )
        .spacing(8.0)
        .line_spacing(20.0);

        let header = text(format!("Current Dir: {}", self.current_dir)).size(20);

        let content = column![header, scrollable(file_list).width(Length::Fill)]
            .spacing(20)
            .padding(Padding {
                top: 20.0,
                bottom: 0.0,
                left: 20.0,
                right: 0.0,
            });

        container(content)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::WHITE)),
                text_color: Some(Color::BLACK),
                ..Default::default()
            })
            .into()
    }
}

pub async fn read_dir(path: String) -> Result<Vec<File>, String> {
    let mut dir = tokio::fs::read_dir(path)
        .await
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut files = vec![];

    while let Ok(Some(entry)) = dir.next_entry().await {
        let file_name = entry.file_name().to_string_lossy().into_owned();

        let file_path = entry.path().display().to_string();

        let file_type = match entry.file_type().await {
            Ok(ft) => {
                if ft.is_dir() {
                    FileType::Directory
                } else if ft.is_file() {
                    FileType::File
                } else if ft.is_symlink() {
                    FileType::Symlink
                } else {
                    FileType::Unknown
                }
            }
            Err(_) => FileType::Unknown,
        };

        files.push(File {
            file_name,
            file_path: FilePath(file_path),
            file_type,
        });
    }

    Ok(files)
}
