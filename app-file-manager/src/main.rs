use app_file_manager::{FileManager, FileManagerMessage};
use iced::{Task, application};

fn main() -> iced::Result {
    application(
        FileManagerApp::new,
        FileManagerApp::update,
        FileManagerApp::view,
    )
    .title("File Manager")
    .centered()
    .run()
}

#[derive(Debug)]
struct FileManagerApp {
    file_manager: FileManager,
}

#[derive(Debug)]
enum Message {
    FileManager(FileManagerMessage),
}

impl FileManagerApp {
    fn new() -> Self {
        Self {
            file_manager: FileManager::new(),
        }
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        self.file_manager.view().map(Message::FileManager)
    }
}
