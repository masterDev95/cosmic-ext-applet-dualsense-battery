use cosmic::{
    Element,
    app::{Core, Task},
    iced::{Alignment, Length, Size, Subscription, window},
    widget::{container, icon, row, text},
};

use crate::battery;

const APP_ID: &str = "io.github.axolotl.CosmicDualsenseBattery";

pub struct AppState {
    core: Core,
    status: BatteryStatus,
    content_size_requested: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum BatteryStatus {
    Checking,
    Connected(u8),
    Disconnected,
    Unavailable,
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    BatteryUpdated(BatteryStatus),
}

impl cosmic::Application for AppState {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = AppMsg;

    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        (
            Self {
                core,
                status: BatteryStatus::Checking,
                content_size_requested: false,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            AppMsg::BatteryUpdated(status) => {
                self.status = status;
                if !self.content_size_requested {
                    self.content_size_requested = true;
                    let (width, height) = self.core.applet.suggested_window_size();
                    let size = if self.core.applet.is_horizontal() {
                        Size::new(54.0, height.get() as f32)
                    } else {
                        Size::new(width.get() as f32, height.get() as f32)
                    };
                    if let Some(id) = self.core.main_window_id() {
                        return window::resize(id, size);
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let label = match self.status {
            BatteryStatus::Checking => "...".to_owned(),
            BatteryStatus::Connected(percentage) => format!("{percentage}%"),
            BatteryStatus::Disconnected => "Déconnectée".to_owned(),
            BatteryStatus::Unavailable => "Indisponible".to_owned(),
        };
        let icon_name = match self.status {
            BatteryStatus::Checking => "battery-symbolic",
            BatteryStatus::Connected(percentage) => battery_icon(percentage),
            BatteryStatus::Disconnected => "battery-missing-symbolic",
            BatteryStatus::Unavailable => "battery-caution-symbolic",
        };
        let icon_size = self.core.applet.suggested_size(true).0;
        container(
            row::with_capacity(2)
                .spacing(4)
                .align_y(Alignment::Center)
                .push(icon::from_name(icon_name).symbolic(true).size(icon_size))
                .push(text(label)),
        )
        .center_y(Length::Fill)
        .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::run(battery::subscription)
    }
}

fn battery_icon(percentage: u8) -> &'static str {
    match percentage {
        0..=5 => "cosmic-applet-battery-level-0-symbolic",
        6..=10 => "cosmic-applet-battery-level-5-symbolic",
        11..=20 => "cosmic-applet-battery-level-10-symbolic",
        21..=35 => "cosmic-applet-battery-level-20-symbolic",
        36..=50 => "cosmic-applet-battery-level-35-symbolic",
        51..=65 => "cosmic-applet-battery-level-50-symbolic",
        66..=80 => "cosmic-applet-battery-level-65-symbolic",
        81..=90 => "cosmic-applet-battery-level-80-symbolic",
        91..=99 => "cosmic-applet-battery-level-90-symbolic",
        _ => "cosmic-applet-battery-level-100-symbolic",
    }
}

#[cfg(test)]
mod tests {
    use super::battery_icon;

    #[test]
    fn selects_icon_for_battery_percentage() {
        assert_eq!(battery_icon(0), "cosmic-applet-battery-level-0-symbolic");
        assert_eq!(battery_icon(50), "cosmic-applet-battery-level-35-symbolic");
        assert_eq!(
            battery_icon(100),
            "cosmic-applet-battery-level-100-symbolic"
        );
    }
}
