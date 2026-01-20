use iced::{Color, Element, Task};
use iced_layershell::build_pattern::application;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::LayerShellSettings;
use iced_layershell::to_layer_message;
use ui_lib::panel::{Panel, PanelMessage};

fn main() -> iced_layershell::Result {
    application(
        PanelShell::new,
        PanelShell::namespace,
        PanelShell::update,
        PanelShell::view,
    )
    .layer_settings(LayerShellSettings {
        size: Some((u32::default(), 50)),
        anchor: Anchor::Top | Anchor::Right | Anchor::Left,
        margin: (0, 0, 0, 0),
        exclusive_zone: 50,
        ..Default::default()
    })
    .style(PanelShell::style)
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
        "ShellApp".into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Panel(panel_msg) => match panel_msg {
                PanelMessage::ChangeTextPressed => {
                    self.panel.text = "I am a Layer Shell!".to_string();
                    Task::none()
                }
            },
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        self.panel.view().map(Message::Panel)
    }

    fn style(&self, theme: &iced::Theme) -> iced::theme::Style {
        use iced::theme::Style;
        Style {
            background_color: Color::WHITE,
            text_color: theme.palette().text,
        }
    }
}
