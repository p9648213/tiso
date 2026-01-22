use iced::{
    Element,
    widget::{container, row, text},
};

use crate::ApplicationItem;

#[derive(Debug, Clone)]
pub struct Compositor {
    pub open_apps: Vec<ApplicationItem>,
}

#[derive(Debug, Clone)]
pub enum CompositorMessage {}

impl Compositor {
    pub fn new() -> Self {
        Self { open_apps: vec![] }
    }

    pub fn view(&self) -> Element<'_, CompositorMessage> {
        let content = row(self
            .open_apps
            .iter()
            .map(|app| text(app.name.clone()).into()))
        .spacing(12);

        container(content).into()
    }
}
