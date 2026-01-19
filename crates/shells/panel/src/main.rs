use iced::widget::container;
use iced::{Color, Element, Task};
use iced_layershell::build_pattern::application;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::LayerShellSettings;
use iced_layershell::to_layer_message;

fn main() -> iced_layershell::Result {
    application(Panel::new, Panel::namespace, Panel::update, Panel::view)
        .layer_settings(LayerShellSettings {
            size: Some((u32::default(), 50)),
            anchor: Anchor::Top | Anchor::Right | Anchor::Left,
            margin: (0, 0, 0, 0),
            exclusive_zone: 50,
            ..Default::default()
        })
        .style(Panel::style)
        .run()
}

struct Panel {}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {}

impl Panel {
    fn new() -> (Self, Task<Message>) {
        (Self {}, Task::none())
    }
    fn namespace() -> String {
        String::from("Panel")
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        container("Hello World").into()
    }

    fn style(&self, theme: &iced::Theme) -> iced::theme::Style {
        use iced::theme::Style;
        Style {
            background_color: Color::WHITE,
            text_color: theme.palette().text,
        }
    }
}
