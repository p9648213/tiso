use iced::{Color, Element, Length, Subscription, Task, theme::Style, widget::container};
use iced_layershell::build_pattern::application;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::LayerShellSettings;
use iced_layershell::to_layer_message;
use so_base::{MIRA_FONT_BYTES, MIRA_FONT_NAME};
use so_core::panel::{Panel, PanelMessage};

fn main() -> iced_layershell::Result {
    application(
        PanelShell::new,
        PanelShell::namespace,
        PanelShell::update,
        PanelShell::view,
    )
    .layer_settings(LayerShellSettings {
        size: Some((u32::default(), 44)),
        anchor: Anchor::Top | Anchor::Right | Anchor::Left,
        margin: (0, 0, 0, 0),
        exclusive_zone: 44,
        ..Default::default()
    })
    .font(MIRA_FONT_BYTES)
    .default_font(MIRA_FONT_NAME)
    .style(|_, _| Style {
        background_color: Color::from_rgba(0.0, 0.0, 0.0, 0.6),
        text_color: Color::WHITE,
    })
    .subscription(PanelShell::subscription)
    .run()
}

struct PanelShell {
    panel: Panel,
}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
    Panel(PanelMessage),
}

impl PanelShell {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                panel: Panel::new(),
            },
            Task::none(),
        )
    }

    fn namespace() -> String {
        "panel".into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Panel(panel_msg) => match panel_msg {
                PanelMessage::TimeTick(local_time) => {
                    self.panel.current_time = local_time;
                    Task::none()
                }
            },
            _ => Task::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        self.panel.subscription().map(Message::Panel)
    }

    fn view(&self) -> Element<'_, Message> {
        container(self.panel.view().map(Message::Panel))
            .center_y(Length::Fill)
            .into()
    }
}
