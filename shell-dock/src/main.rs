use iced::alignment::Vertical;
use iced::theme::Style;
use iced::widget::container;
use iced::{Color, Element, Length, Task};
use iced_layershell::settings::LayerShellSettings;
use iced_layershell::to_layer_message;
use iced_layershell::{build_pattern::application, reexport::Layer};
use ui_lib::dock::{Dock, DockMessage};

fn main() -> iced_layershell::Result {
    application(
        DockShell::new,
        DockShell::namespace,
        DockShell::update,
        DockShell::view,
    )
    .layer_settings(LayerShellSettings {
        size: Some((u32::default(), 120)),
        layer: Layer::Overlay,
        margin: (0, 0, 12, 0),
        ..Default::default()
    })
    .style(|_, _| Style {
        background_color: Color::TRANSPARENT,
        text_color: Color::default(),
    })
    .run()
}

struct DockShell {
    dock: Dock,
}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
    Dock(DockMessage),
}

impl DockShell {
    fn new() -> (Self, Task<Message>) {
        (Self { dock: Dock::new() }, Task::none())
    }

    fn namespace() -> String {
        "dock".into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Dock(dock_msg) => match dock_msg {
                DockMessage::MouseEnter => {
                    self.dock.is_hovered = true;
                    Task::none()
                }
                DockMessage::MouseLeave => {
                    self.dock.is_hovered = false;
                    Task::none()
                }
            },
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let dock = self.dock.view(0.0).map(Message::Dock);
        container(dock)
            .height(Length::Fill)
            .align_y(Vertical::Bottom)
            .center_x(Length::Fill)
            .into()
    }
}
