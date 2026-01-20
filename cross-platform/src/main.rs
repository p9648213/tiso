use iced::{Background, Color, Element, Length, Task, application, widget::container};
use ui_lib::panel::{Panel, PanelMessage};

fn main() -> iced::Result {
    application(App::new, App::update, App::view).run()
}

struct App {
    panel: Panel,
}

#[derive(Debug, Clone)]
enum Message {
    Panel(PanelMessage),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                panel: Panel::new(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Panel(panel_msg) => match panel_msg {
                PanelMessage::ChangeTextPressed => {
                    self.panel.text = "I am a Normal Window!".to_string();
                    Task::none()
                }
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(self.panel.view().map(Message::Panel))
            .height(50)
            .width(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::BLACK)),
                text_color: Some(Color::WHITE),
                ..Default::default()
            })
            .into()
    }
}
