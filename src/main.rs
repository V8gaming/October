use iced::{button, Button, Element, Sandbox, Settings, Text, Container, Length, Column, Row};

static CHARLIST: [&str; 4] = ["a","b","c", "d"];
#[derive(Default)]
struct MyButton {
    button_state1: button::State, 
    button_state2: button::State,
    button_state3: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed1,
    ButtonPressed2,
    ButtonPressed3,
}

impl Sandbox for MyButton {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Button")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed1 => println!("Button pressed"),
            Message::ButtonPressed2 => println!("Button pressed"),
            Message::ButtonPressed3 => println!("Button pressed"),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let text1 = Text::new(format!("text")).height(Length::Units(250)).size(100);
        
        let button1 = Button::new(&mut self.button_state1, Text::new("A"))
            .on_press(Message::ButtonPressed1);

        let button2 = Button::new(&mut self.button_state2, Text::new("B"))
            .on_press(Message::ButtonPressed2);

        let button3 = Button::new(&mut self.button_state3, Text::new("C"))
            .on_press(Message::ButtonPressed3);

        let row1 = Row::new().push(button1).push(button2).push(button3);
        let column1 = Column::new().push(text1).push(row1);
        Container::new(column1)
            .padding(100)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    MyButton::run(Settings::default())
}