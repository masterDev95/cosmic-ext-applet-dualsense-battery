use cosmic::{
    Element,
    app::{Core, Task},
    iced::{Alignment, Subscription},
    widget::{icon, row, text},
};

use crate::battery;

const APP_ID: &str = "io.github.axolotl.CosmicDualsenseBattery";

pub struct AppState {
    core: Core,
    status: BatteryStatus,
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
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            AppMsg::BatteryUpdated(status) => self.status = status,
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
        let icon_size = self.core.applet.suggested_size(true).0;
        row::with_capacity(2)
            .spacing(4)
            .align_y(Alignment::Center)
            .push(
                icon::from_name("battery-symbolic")
                    .symbolic(true)
                    .size(icon_size),
            )
            .push(text(label))
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::run(battery::subscription)
    }
}
