use iced::{
    Alignment, Element, Length, Padding,
    widget::{container, row, text},
};

#[derive(Debug, Clone)]
pub struct Panel {
    pub text: String,
}

#[derive(Debug, Clone)]
pub enum PanelMessage {
    ChangeTextPressed,
}

impl Default for Panel {
    fn default() -> Self {
        Self::new()
    }
}

impl Panel {
    pub fn new() -> Self {
        Self {
            text: "Initial State".to_string(),
        }
    }

    pub fn view(&self) -> Element<'_, PanelMessage> {
        container(row![workspace(), datetime(), widgets()].align_y(Alignment::Center))
            .padding(Padding::from([0, 12]))
            .into()
    }
}

fn datetime<'a>() -> Element<'a, PanelMessage> {
    container(text("Mon 05 Jan 01:14:22"))
        .center_x(Length::Fill)
        .into()
}

fn workspace<'a>() -> Element<'a, PanelMessage> {
    row![text("Workspace"), text("Application")]
        .spacing(10)
        .into()
}

fn widgets<'a>() -> Element<'a, PanelMessage> {
    text("Widgets").into()
}
