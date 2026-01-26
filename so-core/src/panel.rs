use std::time::Duration;

use chrono::{DateTime, Local};
use iced::{
    Alignment, Element, Length, Padding, Subscription,
    widget::{container, row, text},
};

#[derive(Debug)]
pub struct Panel {
    pub current_time: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub enum PanelMessage {
    TimeTick(DateTime<Local>),
}

impl Panel {
    pub fn new() -> Self {
        Self {
            current_time: Local::now(),
        }
    }

    pub fn view(&self) -> Element<'_, PanelMessage> {
        container(
            row![workspace(), datetime(&self.current_time), widgets()].align_y(Alignment::Center),
        )
        .padding(Padding::from([0, 12]))
        .into()
    }

    pub fn subscription(&self) -> Subscription<PanelMessage> {
        iced::time::every(Duration::from_secs(1)).map(|_| PanelMessage::TimeTick(Local::now()))
    }
}

impl Default for Panel {
    fn default() -> Self {
        Self::new()
    }
}

fn datetime<'a>(time: &DateTime<Local>) -> Element<'a, PanelMessage> {
    let formatted_time = time.format("%a %d %b %T").to_string();

    container(text(formatted_time))
        .center_x(Length::Fill)
        .into()
}

fn workspace<'a>() -> Element<'a, PanelMessage> {
    row![text("Application")].spacing(10).into()
}

fn widgets<'a>() -> Element<'a, PanelMessage> {
    text("Widgets").into()
}
