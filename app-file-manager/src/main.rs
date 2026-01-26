use app_file_manager::{FilesManager, FilesManagerMessage};
use iced::{Task, application};
use so_base::{MIRA_FONT_BYTES, MIRA_FONT_NAME};

fn main() -> iced::Result {
    application(
        FilesManagerApp::new,
        FilesManagerApp::update,
        FilesManagerApp::view,
    )
    .font(MIRA_FONT_BYTES)
    .default_font(MIRA_FONT_NAME)
    .title("File Manager")
    .centered()
    .run()
}

#[derive(Debug)]
struct FilesManagerApp {
    files_manager: FilesManager,
}

#[derive(Debug, Clone)]
enum Message {
    FilesManager(FilesManagerMessage),
}

impl FilesManagerApp {
    fn new() -> (Self, Task<Message>) {
        let (files_manager, fm_task) = FilesManager::new();
        (Self { files_manager }, fm_task.map(Message::FilesManager))
    }
 
    fn update(&mut self, message: Message) -> Task<Message> { 
        match message {
            Message::FilesManager(fm_msg) => {
                self.files_manager.update(fm_msg).map(Message::FilesManager)
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        self.files_manager.view().map(Message::FilesManager)
    }
}
