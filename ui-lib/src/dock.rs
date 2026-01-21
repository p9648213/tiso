use iced::{
    Background, Border, Color, ContentFit, Element, Length, Padding,
    border::Radius,
    widget::{button, container, image, row, text, tooltip},
};

#[derive(Debug, Clone)]
pub struct Dock {
    items: Vec<DockItem>,
}

#[derive(Debug, Clone)]
struct DockItem {
    name: String,
    icon: image::Handle,
}

#[derive(Debug, Clone)]
pub enum DockMessage {}

impl Default for Dock {
    fn default() -> Self {
        Self::new()
    }
}

impl Dock {
    pub fn new() -> Self {
        let items = vec![
            DockItem {
                name: "Terminal".to_string(),
                icon: image::Handle::from_bytes(
                    include_bytes!("../../assets/images/terminal.png").as_slice(),
                ),
            },
            DockItem {
                name: "Firefox".to_string(),
                icon: image::Handle::from_bytes(
                    include_bytes!("../../assets/images/firefox.png").as_slice(),
                ),
            },
            DockItem {
                name: "Files".to_string(),
                icon: image::Handle::from_bytes(
                    include_bytes!("../../assets/images/files.png").as_slice(),
                ),
            },
            DockItem {
                name: "Steam".to_string(),
                icon: image::Handle::from_bytes(
                    include_bytes!("../../assets/images/steam.png").as_slice(),
                ),
            },
        ];

        Self { items }
    }

    pub fn view(&self) -> Element<'_, DockMessage> {
        let dock_items = row(self.items.iter().map(|item| {
            tooltip(
                button(
                    image(item.icon.clone())
                        .width(48)
                        .height(48)
                        .content_fit(ContentFit::Fill),
                )
                .style(button::text),
                text(&item.name).size(20).color(Color::WHITE),
                tooltip::Position::Top,
            )
            .into()
        }))
        .spacing(20);

        container(dock_items)
            .padding(Padding {
                right: 12.0,
                left: 12.0,
                bottom: 4.0,
                top: 4.0,
            })
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
                border: Border {
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
                    width: 1.0,
                    radius: Radius::from(12.0),
                },
                ..Default::default()
            })
            .width(Length::Shrink)
            .height(Length::Shrink)
            .into()
    }
}
