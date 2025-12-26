use iced::widget::{button, column, container, row, text};
use iced::{Element, Task, Theme};

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application("CloudEmu", App::update, App::view)
        .theme(|_| Theme::Dark)
        .run()
}

#[derive(Default)]
struct App {
    services: Vec<Service>,
}

#[derive(Clone, Debug)]
struct Service {
    name: String,
    running: bool,
    port: Option<u16>,
}

#[derive(Debug, Clone)]
enum Message {
    StartService(usize),
    StopService(usize),
    StartAll,
    StopAll,
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::StartService(idx) => {
                if let Some(service) = self.services.get_mut(idx) {
                    service.running = true;
                }
            }
            Message::StopService(idx) => {
                if let Some(service) = self.services.get_mut(idx) {
                    service.running = false;
                }
            }
            Message::StartAll => {
                for service in &mut self.services {
                    service.running = true;
                }
            }
            Message::StopAll => {
                for service in &mut self.services {
                    service.running = false;
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let header = row![
            text("CloudEmu").size(24),
            button("Start All").on_press(Message::StartAll),
            button("Stop All").on_press(Message::StopAll),
        ]
        .spacing(20);

        let services = column(
            self.services
                .iter()
                .enumerate()
                .map(|(idx, service)| {
                    let status = if service.running { "Running" } else { "Stopped" };
                    let port = service
                        .port
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "-".into());

                    row![
                        text(&service.name).width(150),
                        text(status).width(100),
                        text(port).width(80),
                        if service.running {
                            button("Stop").on_press(Message::StopService(idx))
                        } else {
                            button("Start").on_press(Message::StartService(idx))
                        },
                    ]
                    .spacing(10)
                    .into()
                })
                .collect(),
        )
        .spacing(10);

        container(column![header, services].spacing(20))
            .padding(20)
            .into()
    }
}

impl Default for Service {
    fn default() -> Self {
        Self {
            name: String::new(),
            running: false,
            port: None,
        }
    }
}
