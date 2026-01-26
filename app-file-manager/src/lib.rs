use iced::{
    Background, Color, Element, Length, Task,
    widget::{column, container, scrollable, text},
};

#[derive(Debug, Clone)]
enum FileType {
    Directory,
    File,
    Symlink,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct File {
    file_name: String,
    file_path: String,
    file_type: FileType,
}

#[derive(Debug)]
pub struct FilesManager {
    current_dir: String,
    files: Vec<File>,
    last_error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum FilesManagerMessage {
    ReadDir(Result<Vec<File>, String>),
}

impl FilesManager {
    pub fn new() -> (Self, Task<FilesManagerMessage>) {
        (
            Self {
                current_dir: "/".to_string(),
                files: vec![],
                last_error: None,
            },
            Task::perform(read_dir("/"), FilesManagerMessage::ReadDir),
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
                        self.files.clear();
                        self.last_error = Some(error);
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, FilesManagerMessage> {
        if let Some(error) = &self.last_error {
            return container(text(format!("Error: {}", error)))
                .center(Length::Fill)
                .into();
        }

        let file_list = column(self.files.iter().map(|file| {
            let icon = match file.file_type {
                FileType::Directory => "📁",
                FileType::File => "📄",
                FileType::Symlink => "🔗",
                FileType::Unknown => "❓",
            };
            text(format!("{} {}", icon, file.file_name)).into()
        }))
        .spacing(5);

        let header = text(format!(
            "Current Dir: {}\nFiles found: {}",
            self.current_dir,
            self.files.len()
        ))
        .size(20);

        let content = column![header, file_list].spacing(20).padding(20);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::WHITE)),
                text_color: Some(Color::BLACK),
                ..Default::default()
            })
            .into()
    }
}

pub async fn read_dir(path: &str) -> Result<Vec<File>, String> {
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
            file_path,
            file_type,
        });
    }

    Ok(files)
}
