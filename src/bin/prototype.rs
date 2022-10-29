use iced::{button, Button, Element, Command, Settings, Text, Container, Length, Column, Row, window, Color, Application, Subscription, executor};
use iced::window::Position::Specific;
use iced::window::Icon;
use global::Global;
use rand::{thread_rng, Rng};
use iced_native::{Event, keyboard};
use sqlite::State;

static LETTERS: Global<Vec<String>> = Global::new();
static ENGLISH: Global<Vec<String>> = Global::new();
static VIETNAMESE: Global<Vec<String>> = Global::new();
static N: Global<Vec<usize>> = Global::new();
static COLOUR: Global<Vec<usize>> = Global::new();
static X: Global<Vec<usize>> = Global::new();
static SCREEN: Global<Vec<usize>> = Global::new();
static TABLE: Global<Vec<usize>> = Global::new();


#[derive(Default, Clone, Debug)]
struct MyButton {
    reload_state: button::State,
    gotomain_state: button::State,
    gototesting_state: button::State,
    shift_state: button::State,
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
    EventOccurred(iced_native::Event),
    ReloadButton,
    GotoMainButton,
    GotoTestingButton,
    ShiftButton,
    NextButton,
    SubmitButton,
    SpaceButton,
    DeleteButton,
    ButtonPressed0,
    ButtonPressed1,
    ButtonPressed2,
    ButtonPressed3,
}

fn pushfn(letter: String) {
    LETTERS.lock_mut().unwrap().push(shiftfn(letter.to_string()));
    println!("ADDED {} TO {}", letter,LETTERS.lock_mut().unwrap().concat());
    COLOUR.lock_mut().unwrap()[0] = 0
}

fn shiftfn(letter: String) -> String {
    if X.lock_mut().unwrap()[0] == 1 {
        return letter.to_uppercase();
    } else {
        return letter.to_lowercase();
    }
}

fn shiftvaluefn() {
    if X.lock_mut().unwrap()[0] == 1 {
        X.lock_mut().unwrap()[0] = 0;
    } else {
        X.lock_mut().unwrap()[0] = 1;
    }
}

fn shiftscreenfn(destination: usize) {
    SCREEN.lock_mut().unwrap()[0] = destination;
    N.lock_mut().unwrap()[0] = thread_rng().gen_range(0..ENGLISH.lock_mut().unwrap().len());
    LETTERS.lock_mut().unwrap().clear();
    COLOUR.lock_mut().unwrap()[0] = 0
}

fn sumbitfn() {
    println!("{:?}",LETTERS.lock_mut().unwrap().concat());
    
    if format!("{}", LETTERS.lock_mut().unwrap().concat()) == VIETNAMESE.lock_mut().unwrap()[N.lock_mut().unwrap()[0]]{
        println!("true");
        COLOUR.lock_mut().unwrap()[0] = 2;
        SCREEN.lock_mut().unwrap()[0] = 2
    } else {
        println!("false");
        COLOUR.lock_mut().unwrap()[0] = 1;
        SCREEN.lock_mut().unwrap()[0] = 2
    }
}

fn popfn() {
    if LETTERS.lock_mut().unwrap().len() != 0 {
        LETTERS.lock_mut().unwrap().pop();
        println!("{}",LETTERS.lock_mut().unwrap().concat());
        COLOUR.lock_mut().unwrap()[0] = 0
    } 
    
}

fn nextfn() {
    //println!("{}", ENGLISH.lock_mut().unwrap().len());
    N.lock_mut().unwrap()[0] = thread_rng().gen_range(0..ENGLISH.lock_mut().unwrap().len());
    LETTERS.lock_mut().unwrap().clear();
    COLOUR.lock_mut().unwrap()[0] = 0
}

fn add_button<'a>(a: &'a mut button::State,b: String,c: Message) -> Button<'a, Message> {
    return Button::new(a, Text::new(format!("{}",b))).on_press(c);
}
fn index() {
    if TABLE.lock_mut().unwrap()[0] == 1{
        TABLE.lock_mut().unwrap()[0] = 0
    } else if TABLE.lock_mut().unwrap()[0] == 0 {
        TABLE.lock_mut().unwrap()[0] = 1
    } else {
        TABLE.lock_mut().unwrap()[0] = 0
    }
    loaddata();
    nextfn();
}
fn loaddata() {
    let connection = sqlite::open("./English-Vietnamese.sqlite3").unwrap();
    
    let mut statement2 = connection
    .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
    .unwrap();
    let mut tables: Vec<String> = Vec::new();

    while let Ok(State::Row) = statement2.next() {
        //println!("{}", statement2.read::<String>(0).unwrap());
        tables.push(statement2.read::<String>(0).unwrap())
    }
    let mut statement = connection
        .prepare(format!("SELECT * FROM {}", tables[TABLE.lock_mut().unwrap()[0]]))
        .unwrap();

    let mut vietnamese: Vec<String> = Vec::new();
    let mut english: Vec<String> = Vec::new();
    let mut typename: Vec<String> = Vec::new();

    while let Ok(State::Row) = statement.next() {
        //println!("{} = {}", statement.read::<String>(0).unwrap(), statement.read::<String>(1).unwrap());
        //println!("{}",statement.read::<String>(0).unwrap());
        english.push(statement.read::<String>(0).unwrap());
        vietnamese.push(statement.read::<String>(1).unwrap());
        typename.push(statement.read::<String>(2).unwrap());
    }
    
    ENGLISH.lock_mut().unwrap().clear();
    for i in english {
        //println!("{}",i);

        ENGLISH.lock_mut().unwrap().push(i);
        
    }
    for i in vietnamese {
        VIETNAMESE.lock_mut().unwrap().push(i)
    }
}

fn makemain(selfx: &mut MyButton) -> Element<Message>{
    let main: Element<Message> = Container::new(add_button(&mut selfx.gototesting_state, String::from("Enter Testing"), Message::GotoTestingButton))
        .padding(100)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into();
    return main;
}
fn makereview(selfx: &mut MyButton) -> Element<Message>{
    let exit = add_button(&mut selfx.gotomain_state, String::from("Exit"), Message::GotoMainButton);
    let colours = vec![Color::BLACK,Color::from_rgb(1.0, 0.0, 0.0),Color::from_rgb(0.0, 1.0, 0.0)];

    let subtitle1 = Text::new("Your answer").color(colours[COLOUR.lock_mut().unwrap()[0]]);
    let subtitle2 = Text::new("Vietnamese");
    let subtitle3 = Text::new("English");
    
    let youranswer = Text::new(format!("{}", LETTERS.lock_mut().unwrap().concat())).height(Length::Units(80)).size(40).color(colours[COLOUR.lock_mut().unwrap()[0]]);
    let english = Text::new(format!("{}",ENGLISH.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Units(80)).size(40);
    let vietnamese = Text::new(format!("{}",VIETNAMESE.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Units(80)).size(40);

    let resume = add_button(&mut selfx.gototesting_state, String::from("Resume"), Message::GotoTestingButton);
    let column = Column::new().push(exit).push(subtitle1).push(youranswer).push(subtitle2).push(vietnamese).push(subtitle3).push(english).push(resume);
    let main: Element<Message> = Container::new(column)
        .padding(100)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .into();
    return main;
}

fn makelevel(selfx: &mut MyButton) -> Element<Message>{
    let english = Text::new(format!("{}",ENGLISH.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Units(150)).size(80);

    let colours = vec![Color::BLACK,Color::from_rgb(1.0, 0.0, 0.0),Color::from_rgb(0.0, 1.0, 0.0)];
    let text1 = Text::new(format!("{}", LETTERS.lock_mut().unwrap().concat())).height(Length::Units(150)).size(80).color(colours[COLOUR.lock_mut().unwrap()[0]]);

    let buttons = [
        add_button(&mut selfx.button_state0, shiftfn(String::from("a")), Message::ButtonPressed0),
        add_button(&mut selfx.button_state1, shiftfn(String::from("b")), Message::ButtonPressed1),
        add_button(&mut selfx.button_state2, shiftfn(String::from("c")), Message::ButtonPressed2),
        add_button(&mut selfx.button_state3, shiftfn(String::from("d")), Message::ButtonPressed3),
    ];

    let reload = add_button(&mut selfx.reload_state, String::from("Reload"), Message::ReloadButton);

    let exit = add_button(&mut selfx.gotomain_state, String::from("Exit Testing"), Message::GotoMainButton);
    let shift = add_button(&mut selfx.shift_state, String::from("Shift"), Message::ShiftButton);
    let submit = add_button(&mut selfx.submit_state, String::from("submit"), Message::SubmitButton);
    let space = add_button(&mut selfx.space_state, String::from("space"), Message::SpaceButton);
    let delete = add_button(&mut selfx.delete_state, String::from("delete"), Message::DeleteButton);
    let next = add_button(&mut selfx.next_state, String::from("next"), Message::NextButton);

    let mut userrow = Row::new();
    userrow = userrow.push(submit).push(space).push(delete).push(next).push(shift).push(reload);


    let mut row1 = Row::new();
    for button in buttons {
        row1 = row1.push(button);
    };
    let utilrow = Row::new().push(exit);
    let column1 = Column::new().push(utilrow.width(Length::Fill)).push(english).push(text1).push(userrow).push(row1).width(Length::Fill).align_items(iced::Alignment::Center);
    let testing: Element<Message> = Container::new(column1)
        .padding(100)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into();
    return testing;
}
impl Application for MyButton {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    
    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn new(_flags: ()) -> (MyButton, Command<Message>) {
        (MyButton::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Button")
    }
    
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ReloadButton => index(),
            Message::GotoMainButton => shiftscreenfn(0),
            Message::GotoTestingButton => shiftscreenfn(1),
            Message::SubmitButton => sumbitfn(),
            Message::SpaceButton => pushfn(String::from(" ")),
            Message::DeleteButton => popfn(),
            Message::NextButton => nextfn(),
            Message::ShiftButton => shiftvaluefn(),
            Message::ButtonPressed0 => pushfn(String::from("a")),
            Message::ButtonPressed1 => pushfn(String::from("b")),
            Message::ButtonPressed2 => pushfn(String::from("c")),
            Message::ButtonPressed3 => pushfn(String::from("d")),
            Message::EventOccurred(event) => {
                if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Space, modifiers: _ }) = event {
                    pushfn(String::from(" "))
                } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::LShift, modifiers: _ }) = event {
                    shiftvaluefn()
                } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Right, modifiers: _}) = event {
                    nextfn()
                } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Enter, modifiers: _ }) = event {
                    sumbitfn()
                } 
            },
        };

        Command::none()

    }

    fn view(&mut self) -> Element<Message> {

        if SCREEN.lock_mut().unwrap()[0] == 0 {
            return makemain(self);
        } else if SCREEN.lock_mut().unwrap()[0] == 1 {
            return makelevel(self);
        } else if SCREEN.lock_mut().unwrap()[0] == 2 {
            return makereview(self);
        } else {
            return makemain(self);
        }
        
    }
    
}


fn main() -> iced::Result {
    let rgba = vec![0, 0, 0, 255];
    TABLE.lock_mut().unwrap().push(0);
    loaddata();
    N.lock_mut().unwrap().push(thread_rng().gen_range(0..ENGLISH.lock_mut().unwrap().len()));
    COLOUR.lock_mut().unwrap().push(0);
    X.lock_mut().unwrap().push(0);
    SCREEN.lock_mut().unwrap().push(0);

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
        id: Some("October".to_string()),
        flags: (),
        default_text_size: 20,
        text_multithreading: true,
        exit_on_close_request: true,
        try_opengles_first: false,
    };
    MyButton::run(setting)

}

