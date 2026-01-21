use iced::{
    Background, Color, ContentFit, Element, Font, Length, Task, application,
    widget::{container, image, stack},
};
use ui_lib::panel::{Panel, PanelMessage};

const MIRA_FONT: Font = Font::with_name("Miracode");

fn main() -> iced::Result {
    application(App::new, App::update, App::view)
        .title("Bury")
        .font(include_bytes!("../../assets/font/Miracode.ttf").as_slice())
        .default_font(MIRA_FONT)
        .centered()
        .run()
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
        let background = image("../assets/images/background.jpg")
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(ContentFit::Fill);

        let panel = container(self.panel.view().map(Message::Panel))
            .center_y(44)
            .width(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.6))),
                text_color: Some(Color::WHITE),
                ..Default::default()
            });

        stack![background, panel]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
