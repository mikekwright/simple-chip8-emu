use iced::widget::{button, column, text};


impl IcedDisplay {
    fn view(&self) {
        // The buttons
        let increment = button("+").on_press(Message::Increment);
        let decrement = button("-").on_press(Message::Decrement);

        // The number
        let counter = text(self.value);

        // The layout
        let interface = column![increment, counter, decrement];
    }
}