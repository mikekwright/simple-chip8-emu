// This is the main application that will hold the emulator view and publish interactions events.
use iced::{Application, executor, Command, Element, Theme};
use iced::widget::{button, column, text};
use crate::hardware::display::Chip8Display;

#[derive(Default)]
pub struct SimpleChip8Emulator {
    display: Chip8Display,
    counter: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
}

impl Application for SimpleChip8Emulator {
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
