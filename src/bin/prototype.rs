use iced::{button, Button, Element, Sandbox, Settings, Text, Container, Length, Column, Row, window};
use iced::window::Position::Specific;
use iced::window::Icon;
use global::Global;
use rand::{thread_rng, Rng};

static LETTERS: Global<Vec<String>> = Global::new();
static ENGLISH: Global<Vec<&str>> = Global::new();
static VIETNAMESE: Global<Vec<&str>> = Global::new();
static N: Global<Vec<usize>> = Global::new();

#[derive(Default, Clone, Debug)]
struct MyButton {
    next_state: button::State,
    submit_state: button::State,
    space_state: button::State,
    delete_state: button::State,
    button_state0: button::State, 
    button_state1: button::State,
    button_state2: button::State,
    button_state3: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    NextButton,
    SubmitButton,
    SpaceButton,
    DeleteButton,
    ButtonPressed0,
    ButtonPressed1,
    ButtonPressed2,
    ButtonPressed3,
}


fn pushfn(letter: &str) {
    LETTERS.lock_mut().unwrap().push(letter.to_string());
    println!("ADDED {} TO {}", letter,LETTERS.lock_mut().unwrap().concat());
}

fn sumbitfn() {
    println!("{:?}",LETTERS.lock_mut().unwrap().concat());
    let vietnamese = ["của chị ấy","vâng","có thể","không thể",];
    for i in vietnamese {
        VIETNAMESE.lock_mut().unwrap().push(i)
    }
    
    if format!("{}", LETTERS.lock_mut().unwrap().concat()) == VIETNAMESE.lock_mut().unwrap()[N.lock_mut().unwrap()[0]]{
        println!("true")
    } else {
        println!("false")
    }
}

fn popfn() {
    if LETTERS.lock_mut().unwrap().len() != 0 {
        LETTERS.lock_mut().unwrap().pop();
        println!("{}",LETTERS.lock_mut().unwrap().concat());
    } 
    
}

fn nextfn() {
    N.lock_mut().unwrap()[0] = thread_rng().gen_range(0..4);
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
            Message::SubmitButton => sumbitfn(),
            Message::SpaceButton => pushfn(" "),
            Message::DeleteButton => popfn(),
            Message::NextButton => nextfn(),
            Message::ButtonPressed0 => pushfn("a"),
            Message::ButtonPressed1 => pushfn("b"),
            Message::ButtonPressed2 => pushfn("c"),
            Message::ButtonPressed3 => pushfn("d"),

        };

    }

    fn view(&mut self) -> Element<Message> {

        fn add_button<'a>(a: &'a mut button::State,b: &'a str,c: Message) -> Button<'a, Message> {
            return Button::new(a, Text::new(format!("{}",b))).on_press(c);
        }
       

        let english = ["hers","yes","can","can not"];
        for i in english {
            ENGLISH.lock_mut().unwrap().push(i)
        }

        let english = Text::new(format!("{}",ENGLISH.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Units(150)).size(80);

        let text1 = Text::new(format!("{}", LETTERS.lock_mut().unwrap().concat())).height(Length::Units(150)).size(80);

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
        let submit = add_button(&mut self.submit_state, "submit", Message::SubmitButton);
        let space = add_button(&mut self.space_state, "space", Message::SpaceButton);
        let delete = add_button(&mut self.delete_state, "delete", Message::DeleteButton);
        let next = add_button(&mut self.next_state, "next", Message::NextButton);

        let mut userrow = Row::new();
        userrow = userrow.push(submit).push(space).push(delete).push(next);


        let mut row1 = Row::new();
        for button in buttons {
            row1 = row1.push(button);
        };

        let column1 = Column::new().push(english).push(text1).push(userrow).push(row1).width(Length::Fill).align_items(iced::Alignment::Center);
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
    N.lock_mut().unwrap().push(thread_rng().gen_range(0..4));
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
        text_multithreading: true,
        exit_on_close_request: true,
        try_opengles_first: false,
    };
    MyButton::run(setting)
}

