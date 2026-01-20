use iced::{
    Element,
    widget::{button, container, row, text},
};

// 1. Shared State
#[derive(Debug, Clone)]
pub struct Panel {
    pub text: String,
}

// 2. Shared Events
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
        container(row![
            text(&self.text),
            button("Click Me").on_press(PanelMessage::ChangeTextPressed)
        ])
        .into()
    }
}
