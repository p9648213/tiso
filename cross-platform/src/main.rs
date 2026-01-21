use iced::{
    Background, Color, ContentFit, Element, Font, Length, Subscription, Task,
    application,
    widget::{column, container, image, stack, text},
};
use ui_lib::{
    dock::{Dock, DockMessage},
    panel::{Panel, PanelMessage},
};

const MIRA_FONT: Font = Font::with_name("Miracode");

fn main() -> iced::Result {
    application(App::new, App::update, App::view)
        .title("Bury")
        .font(include_bytes!("../../assets/font/Miracode.ttf").as_slice())
        .default_font(MIRA_FONT)
        .centered()
        .subscription(App::subscription)
        .run()
}

struct App {
    panel: Panel,
    dock: Dock,
    background: image::Handle,
}

#[derive(Debug, Clone)]
enum Message {
    Panel(PanelMessage),
    Dock(DockMessage),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                panel: Panel::new(),
                dock: Dock::new(),
                background: image::Handle::from_bytes(
                    include_bytes!("../../assets/images/background.jpg").as_slice(),
                ),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Panel(panel_msg) => match panel_msg {
                PanelMessage::TimeTick(local_time) => {
                    self.panel.current_time = local_time;
                    Task::none()
                }
            },
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        self.panel.subscription().map(Message::Panel)
    }

    fn view(&self) -> Element<'_, Message> {
        let background = image(&self.background)
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

        let compositor = container(text("Compositor")).center(Length::Fill);

        let dock_overlay = self.dock.view().map(Message::Dock);

        stack![background, column![panel, compositor], dock_overlay]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
