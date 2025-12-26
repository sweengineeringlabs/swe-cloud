use dioxus::prelude::*;
use cloudemu_ui::App;

fn main() {
    tracing_subscriber::fmt::init();

    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_window(
                    dioxus::desktop::WindowBuilder::new()
                        .with_title("CloudEmu")
                        .with_inner_size(dioxus::desktop::LogicalSize::new(1200, 800))
                )
        )
        .launch(App);
}
