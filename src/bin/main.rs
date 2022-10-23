use iced::{button, Button, Element, Sandbox, Settings, Text, Container, Length, Column, Row, window};
use iced::window::Position::Specific;
use iced::window::Icon;

#[derive(Default, Clone)]
struct MyButton {
    button_state0: button::State, 
    button_state1: button::State,
    button_state2: button::State,
    button_state3: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed0,
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
        let mut letters: Vec<&str> = Vec::new();
        match message {
            Message::ButtonPressed0 => letters.push("a"),
            Message::ButtonPressed1 => letters.push("b"),
            Message::ButtonPressed2 => letters.push("c"),
            Message::ButtonPressed3 => letters.push("d"),
        }
    }

    fn view(&mut self) -> Element<Message> {

        fn add_button<'a>(a: &'a mut button::State,b: &'a str,c: Message) -> Button<'a, Message> {
            return Button::new(a, Text::new(format!("{}",b))).on_press(c);
        }
        
        let text1 = Text::new(format!("text")).height(Length::Units(250)).size(100);

        /* 
        let mut buttons: Vec<Button<Message>> = Vec::new();
        
        let button = add_button(&mut self.button_state0, charlist[i], Message::ButtonPressed0);
        buttons.push(button)
        */
        let buttons = [
            add_button(&mut self.button_state0, "a", Message::ButtonPressed0),
            add_button(&mut self.button_state1, "b", Message::ButtonPressed1),
            add_button(&mut self.button_state2, "c", Message::ButtonPressed2),
            add_button(&mut self.button_state3, "d", Message::ButtonPressed3),
        ];

         /* 
        let button1 = Button::new(&mut self.button_state1, Text::new("A"))
            .on_press(Message::ButtonPressed1);

        let button2 = Button::new(&mut self.button_state2, Text::new("B"))
            .on_press(Message::ButtonPressed2);

        let button3 = Button::new(&mut self.button_state3, Text::new("C"))
            .on_press(Message::ButtonPressed3);
        */

        let mut row1 = Row::new();
        for button in buttons {
            row1 = row1.push(button);
        };
        let column1 = Column::new().push(text1).push(row1).width(Length::Fill).align_items(iced::Alignment::Center);
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
    let rgba = vec![0, 0, 0, 255];
    let setting: iced::Settings<()> = Settings {
        window: window::Settings {
            size: (800, 600),
            resizable: true,
            decorations: true,
            min_size: Some((100 as u32,100 as u32)),
            max_size: Some((2000 as u32,2000 as u32)),
            transparent: false,
            always_on_top: true,
            icon: Some(Icon::from_rgba(rgba, 1, 1).unwrap()),
            position: Specific(0, 0),
        },
        default_font: Some(include_bytes!("../../resources/Arial Unicode MS Font.ttf")),
        antialiasing: true,
        id: Some("buttons".to_string()),
        flags: (),
        default_text_size: 20,
        text_multithreading: false,
        exit_on_close_request: true,
        try_opengles_first: false,
    };
    MyButton::run(setting)
}