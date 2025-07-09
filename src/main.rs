mod hardware;

use iced::{Application, executor, Command, Element, Settings, Theme};
use iced::widget::{button, column, text};
use crate::hardware::display::Chip8Display;

fn main() -> iced::Result {
    Chip8GuiApp::run(Settings::default())
}

#[derive(Default)]
struct Chip8GuiApp {
    display: Chip8Display,
    counter: u32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
}

impl Application for Chip8GuiApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("CHIP-8 Emulator Display")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Increment => {
                self.counter += 1;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        use crate::hardware::display::Display;
        column![
            text("CHIP-8 Emulator Display").size(32),
            button("Increment Counter").on_press(Message::Increment),
            text(format!("Counter: {}", self.counter)),
            text(format!("Display size: {}x{}", Display::width(&self.display), Display::height(&self.display))),
        ]
        .padding(20)
        .spacing(10)
        .into()
    }
}
