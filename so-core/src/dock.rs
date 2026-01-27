use iced::{
    Background, Border, Color, ContentFit, Element, Length, Padding,
    border::Radius,
    widget::{MouseArea, button, container, image, row, text, tooltip},
};

use so_base::{
    ApplicationItem, ApplicationType, FIREFOX_ICON, STEAM_ICON, TERMINAL_ICON, USERHOME_ICON,
};

#[derive(Debug)]
pub struct Dock {
    items: Vec<ApplicationItem>,
    pub is_hovered: bool,
}

#[derive(Debug, Clone)]
pub enum DockMessage {
    MouseEnter,
    MouseLeave,
    AppClick(ApplicationItem),
}

impl Dock {
    pub fn new() -> Self {
        let items = vec![
            ApplicationItem {
                name: "Terminal".to_string(),
                icon: image::Handle::from_bytes(TERMINAL_ICON),
                app_type: ApplicationType::Terminal,
            },
            ApplicationItem {
                name: "Firefox".to_string(),
                icon: image::Handle::from_bytes(FIREFOX_ICON),
                app_type: ApplicationType::FireFox,
            },
            ApplicationItem {
                name: "Files Manager".to_string(),
                icon: image::Handle::from_bytes(USERHOME_ICON),
                app_type: ApplicationType::FilesManager,
            },
            ApplicationItem {
                name: "Steam".to_string(),
                icon: image::Handle::from_bytes(STEAM_ICON),
                app_type: ApplicationType::Steam,
            },
        ];

        Self {
            items,
            is_hovered: false,
        }
    }

    pub fn view(&self, window_height: f32) -> Element<'_, DockMessage> {
        let dock_items = row(self.items.iter().map(|item| {
            tooltip(
                button(
                    image(item.icon.clone())
                        .width(48)
                        .height(48)
                        .content_fit(ContentFit::Fill),
                )
                .style(button::text)
                .on_press(DockMessage::AppClick(item.clone())),
                text(&item.name).size(20).color(Color::WHITE),
                tooltip::Position::Top,
            )
            .into()
        }))
        .spacing(20);

        let dock = container(dock_items)
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
            .height(Length::Shrink);

        let dock_container = container(dock).center_x(Length::Fill).padding(Padding {
            top: if self.is_hovered { 0.0 } else { window_height },
            bottom: 12.0,
            ..Default::default()
        });

        let dock_overlay = container(dock_container).width(Length::Fill).height(68);

        MouseArea::new(dock_overlay)
            .on_enter(DockMessage::MouseEnter)
            .on_exit(DockMessage::MouseLeave)
            .into()
    }
}

impl Default for Dock {
    fn default() -> Self {
        Self::new()
    }
}
