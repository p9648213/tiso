use iced::{
    Background, Color, ContentFit, Element, Length, Size, Subscription, Task, application,
    widget::{Space, column, container, image, stack},
    window::{self, Settings},
};
use so_base::{MIRA_FONT_BYTES, MIRA_FONT_NAME};
use so_core::{
    compositor::{Compositor, CompositorMessage},
    dock::{Dock, DockMessage},
    panel::{Panel, PanelMessage},
};

fn main() -> iced::Result {
    application(App::new, App::update, App::view)
        .title("Bury")
        .font(MIRA_FONT_BYTES)
        .default_font(MIRA_FONT_NAME)
        .centered()
        .window(Settings {
            fullscreen: true,
            ..Default::default()
        })
        .subscription(App::subscription)
        .run()
}

#[derive(Debug)]
struct App {
    panel: Panel,
    dock: Dock,
    compositor: Compositor,
    background: image::Handle,
    window_size: Size,
}

#[derive(Debug)]
enum Message {
    Panel(PanelMessage),
    Dock(DockMessage),
    Compositor(CompositorMessage),
    WindowSize(Size),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                panel: Panel::new(),
                dock: Dock::new(),
                compositor: Compositor::new(),
                background: image::Handle::from_bytes(
                    include_bytes!("../../assets/images/background.jpg").as_slice(),
                ),
                window_size: Size::default(),
            },
            window::latest()
                .and_then(window::size)
                .map(Message::WindowSize),
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
            Message::Dock(dock_msg) => match dock_msg {
                DockMessage::MouseEnter => {
                    self.dock.is_hovered = true;
                    Task::none()
                }
                DockMessage::MouseLeave => {
                    self.dock.is_hovered = false;
                    Task::none()
                }
                DockMessage::AppClick(app) => self
                    .compositor
                    .update(CompositorMessage::OpenApplication(app))
                    .map(Message::Compositor),
            },
            Message::Compositor(compositor_msg) => self
                .compositor
                .update(compositor_msg)
                .map(Message::Compositor),
            Message::WindowSize(size) => {
                self.window_size = size;
                Task::none()
            }
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

        let compositor = container(
            self.compositor
                .view(self.window_size)
                .map(Message::Compositor),
        )
        .center(Length::Fill);

        let dock = self.dock.view(self.window_size.height).map(Message::Dock);

        let dock_overlay = column![Space::new().height(Length::Fill), dock];

        stack![background, column![panel, compositor], dock_overlay]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
