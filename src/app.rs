use cosmic::{
    Element,
    app::{Core, Task},
    iced::Alignment,
    widget::{icon, row, text},
};

const APP_ID: &str = "io.github.axolotl.CosmicDualsenseBattery";

pub struct AppState {
    core: Core,
}

impl cosmic::Application for AppState {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = ();

    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        (Self { core }, Task::none())
    }

    fn update(&mut self, _message: Self::Message) -> Task<Self::Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let icon_size = self.core.applet.suggested_size(true).0;
        row::with_capacity(2)
            .spacing(4)
            .align_y(Alignment::Center)
            .push(
                icon::from_name("battery-symbolic")
                    .symbolic(true)
                    .size(icon_size),
            )
            .push(text("DualSense"))
            .into()
    }
}
