use std::fs;
use std::io::Write;
use std::time::{Duration, Instant};
use iced::{button, Button, Checkbox, Scrollable, Slider, slider, Element, Command, Settings as IcedSettings, Text, Container, Length, Column, Row, window, Color, Application, Subscription, executor, alignment, scrollable};
use iced::window::Position::Specific;
use iced::window::Icon;
use global::Global;
use rand::{thread_rng, Rng};
use iced_native::{Event, keyboard};
use sqlite::State;
use serde_derive::Deserialize;
use toml;

static LETTERS: Global<Vec<String>> = Global::new();
static ENGLISH: Global<Vec<String>> = Global::new();
static VIETNAMESE: Global<Vec<String>> = Global::new();
static N: Global<Vec<usize>> = Global::new();
static COLOUR: Global<Vec<usize>> = Global::new();
static X: Global<Vec<usize>> = Global::new();
static SCREEN: Global<Vec<usize>> = Global::new();
static TABLE: Global<Vec<usize>> = Global::new();
static LANG: Global<Vec<usize>> = Global::new();
static SETTINGS_USIZE: Global<Vec<usize>> = Global::new();
static SETTINGS_BOOL: Global<Vec<bool>> = Global::new();


#[derive(Deserialize, Debug)]
struct Data {
    settings: Settings
}

#[derive(Deserialize, Debug)]
struct TextsizeData {
    h1: usize,
    h2: usize,
    h3: usize,
    h4: usize,
    body: usize,
}


#[derive(Deserialize, Debug)]
struct SoundData {
    sound: bool,
    volume: usize,

}

#[derive(Deserialize, Debug)]
struct TimeData {
    timed: bool,
    length: usize,
}

#[derive(Deserialize, Debug)]
struct Settings {
    seperate_check_synonyms: bool,
    sound: SoundData,
    textsize: TextsizeData,
    time: TimeData,
}


#[derive(Default, Clone, Debug)]
struct Buttons {
    gotomain_state: button::State,
    gototesting_state: button::State,
    gotolang_state: button::State,
    gotosetting_state: button::State,
    shift_state: button::State,
    next_state: button::State,
    submit_state: button::State,
    space_state: button::State,
    delete_state: button::State,
    save_state: button::State,
    button_state0: button::State, 
    button_state1: button::State,
    button_state2: button::State,
    button_state3: button::State,
    lang_state0: button::State,
    lang_state1: button::State,
    basic_state: button::State,
    intermediate_state: button::State,
    advanced_state: button::State,
    settings: SettingButtons,

}
#[derive(Clone, Debug, Default)]
struct SettingButtons {
    volume: slider::State,
    length: slider::State,
    h1: slider::State,
    h2: slider::State,
    h3: slider::State,
    h4: slider::State,
    body: slider::State,
    scrollable_state: scrollable::State,
    load_state: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(iced_native::Event),
    GotoMainButton,
    GotoTestingButton,
    GotoLangButton,
    GotoSettingButton,
    ShiftButton,
    NextButton,
    SubmitButton,
    SpaceButton,
    DeleteButton,
    SaveButton,
    ButtonPressed0,
    ButtonPressed1,
    ButtonPressed2,
    ButtonPressed3,
    LangButton0,
    LangButton1,
    BasicButton,
    IntermediateButton,
    AdvancedButton,
    SeperateCheckBox(bool),
    SoundBox(bool),
    VolumeSlider(i32),
    TimedBox(bool),
    LengthSlider(i32),
    H1Slider(i32),
    H2Slider(i32),
    H3Slider(i32),
    H4Slider(i32),
    BodySlider(i32),
    LoadButton,
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
    return Button::new(a, body(b)).on_press(c);
}

fn index(num: usize) {
    let mut languages: Vec<String> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        languages.push(file.unwrap().path().display().to_string())
    }
    let connection = sqlite::open(format!("{}", languages[LANG.lock_mut().unwrap()[0]])).unwrap();
    let mut statement2 = connection
    .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
    .unwrap();
    let mut tables: Vec<String> = Vec::new();
    while let Ok(State::Row) = statement2.next() {
        tables.push(statement2.read::<String>(0).unwrap())
    }
    if num < tables.len() {
        SCREEN.lock_mut().unwrap()[0] = 1;
        TABLE.lock_mut().unwrap()[0] = num;
        loaddata();
        nextfn();
    }
}
fn loadtables<'a>(self1: &'a mut button::State, self2: &'a mut button::State, self3: &'a mut button::State) -> Vec<Button<'a, Message>> {
    if LANG.lock_mut().unwrap()[0] == 0 {
        return vec![
            add_button(self1, String::from("Enter basic"), Message::BasicButton), 
            add_button(self2, String::from("Enter intermediate"), Message::IntermediateButton),
            add_button(self3, String::from("Enter advanced"), Message::AdvancedButton)
            ];
    } else if LANG.lock_mut().unwrap()[0] == 1 {
        return vec![
            add_button(self1, String::from("Enter basic"), Message::BasicButton), 
            add_button(self2, String::from("Enter intermediate"), Message::IntermediateButton),            
            ];
    } else {
        return vec![
            add_button(self1, String::from("Enter basic"), Message::BasicButton), 
            add_button(self2, String::from("Enter intermediate"), Message::IntermediateButton),            
            ];
    }
}
fn loaddata() {
    let mut languages: Vec<String> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        //println!("{}", file.unwrap().path().display());
        languages.push(file.unwrap().path().display().to_string())
    }
    let connection = sqlite::open(format!("{}", languages[LANG.lock_mut().unwrap()[0]])).unwrap();
    
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
fn changelang(num: usize) {
    LANG.lock_mut().unwrap()[0] = num;
    shiftscreenfn(0);
    loaddata();
}
fn makelang(selfx: &mut Buttons) -> Element<Message>{
    let lang0 = add_button(&mut selfx.lang_state0, String::from("Lang0"), Message::LangButton0);
    let lang1 = add_button(&mut selfx.lang_state1, String::from("Lang1"), Message::LangButton1);
    let langcolumn = Column::new().push(lang0).push(lang1);
    let main: Element<Message> = Container::new(langcolumn)
        .padding(100)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into();
    return main;
}

fn savefn() {
    let mut file = std::fs::File::create("./settings.toml").expect("create failed");
    file.write_all(format!("[settings]\n").as_bytes()).expect("write failed");
    file.write_all(format!("seperate_check_synonyms = {} # 0\n", SETTINGS_BOOL.lock_mut().unwrap()[0]).as_bytes()).expect("write failed");
    file.write_all(format!("\n").as_bytes()).expect("write failed");
    file.write_all(format!("[settings.sound]\n").as_bytes()).expect("write failed");
    file.write_all(format!("sound =  {} # 1\n", SETTINGS_BOOL.lock_mut().unwrap()[1]).as_bytes()).expect("write failed");
    file.write_all(format!("volume = {} # 0\n", SETTINGS_USIZE.lock_mut().unwrap()[0]).as_bytes()).expect("write failed");
    file.write_all(format!("\n").as_bytes()).expect("write failed");

    file.write_all(format!("[settings.time]\n").as_bytes()).expect("write failed");
    file.write_all(format!("timed =  {} # 2\n", SETTINGS_BOOL.lock_mut().unwrap()[2]).as_bytes()).expect("write failed");
    file.write_all(format!("length = {} # 1\n", SETTINGS_USIZE.lock_mut().unwrap()[1]).as_bytes()).expect("write failed");
    file.write_all(format!("\n").as_bytes()).expect("write failed");

    file.write_all(format!("[settings.textsize]\n").as_bytes()).expect("write failed");
    file.write_all(format!("h1 = {} # 2\n", SETTINGS_USIZE.lock_mut().unwrap()[2]).as_bytes()).expect("write failed");
    file.write_all(format!("h2 = {} # 3\n", SETTINGS_USIZE.lock_mut().unwrap()[3]).as_bytes()).expect("write failed");
    file.write_all(format!("h3 = {} # 4\n", SETTINGS_USIZE.lock_mut().unwrap()[4]).as_bytes()).expect("write failed");
    file.write_all(format!("h4 = {} # 5\n", SETTINGS_USIZE.lock_mut().unwrap()[5]).as_bytes()).expect("write failed");
    file.write_all(format!("body = {} # 6\n", SETTINGS_USIZE.lock_mut().unwrap()[6]).as_bytes()).expect("write failed");
    file.write_all(format!("\n").as_bytes()).expect("write failed");

}
#[macro_export]
macro_rules! makeslider {
    ($a:expr,$b:expr,$c:expr,$d:expr)=>{
        {
            Slider::new($a,0..=$b,$c as i32,$d)
        }
    };
    ($a:expr,$b:expr,$c:expr,$d:expr, $e:expr)=>{
        {
            Slider::new($a,$b..=$c,$d as i32,$e)
        }
    }
}

fn makesettings(selfx: &mut Buttons) -> Element<Message>{
    let h2_general= h2(String::from("General Settings"));
    let seperatecheckbox = Checkbox::new(SETTINGS_BOOL.lock_mut().unwrap()[0], "Seperately check synonyms", Message::SeperateCheckBox);
    let h2_sound = h2(String::from("Sound Settings"));
    let soundbox = Checkbox::new(SETTINGS_BOOL.lock_mut().unwrap()[1], "Sound Enabled", Message::SoundBox);
    let volumeslider = makeslider!(&mut selfx.settings.volume, 100, SETTINGS_USIZE.lock_mut().unwrap()[0], Message::VolumeSlider);
    let h2_time = h2(String::from("Time Settings"));
    let lengthbox = Checkbox::new(SETTINGS_BOOL.lock_mut().unwrap()[2], "Timed", Message::TimedBox);
    let lengthslider = makeslider!(&mut selfx.settings.length, 1, 30, SETTINGS_USIZE.lock_mut().unwrap()[1], Message::LengthSlider);
    let h2_text= h2(String::from("Text Settings"));
    let h1slider = makeslider!(&mut selfx.settings.h1, 200, SETTINGS_USIZE.lock_mut().unwrap()[2], Message::H1Slider);
    let h2slider = makeslider!(&mut selfx.settings.h2, 200, SETTINGS_USIZE.lock_mut().unwrap()[3], Message::H2Slider);
    let h3slider = makeslider!(&mut selfx.settings.h3, 200, SETTINGS_USIZE.lock_mut().unwrap()[4], Message::H3Slider);
    let h4slider = makeslider!(&mut selfx.settings.h4, 200, SETTINGS_USIZE.lock_mut().unwrap()[5], Message::H4Slider);
    let bodyslider = makeslider!(&mut selfx.settings.body, 200, SETTINGS_USIZE.lock_mut().unwrap()[6], Message::BodySlider);
    let volume = h4(String::from(format!("Volume: {}", SETTINGS_USIZE.lock_mut().unwrap()[0])));
    let length = h4(String::from(format!("Time: {}", SETTINGS_USIZE.lock_mut().unwrap()[1])));
    let h1 = h1(String::from(format!("H1: {}", SETTINGS_USIZE.lock_mut().unwrap()[2])));
    let h2 = h2(String::from(format!("H2: {}", SETTINGS_USIZE.lock_mut().unwrap()[3])));
    let h3 = h3(String::from(format!("H3: {}", SETTINGS_USIZE.lock_mut().unwrap()[4])));
    let h4 = h4(String::from(format!("H4: {}", SETTINGS_USIZE.lock_mut().unwrap()[5])));
    let body = body(String::from(format!("Body: {}", SETTINGS_USIZE.lock_mut().unwrap()[6])));
    let reset = add_button(&mut selfx.settings.load_state, String::from("Reload Default"), Message::LoadButton);
    let save = add_button(&mut selfx.save_state, String::from("Save"), Message::SaveButton); 
    let exit = add_button(&mut selfx.gotomain_state, String::from("Exit"), Message::GotoMainButton);
    let row = Row::new().push(save).push(exit).push(reset);

    let settingcolumn = Column::new()
    .push(h2_general)
    .push(seperatecheckbox)
    .push(h2_sound)
    .push(soundbox)
    .push(volume).push(volumeslider)
    .push(h2_time)
    .push(lengthbox)
    .push(length).push(lengthslider)
    .push(h2_text)
    .push(h1).push(h1slider)
    .push(h2).push(h2slider)
    .push(h3).push(h3slider)
    .push(h4).push(h4slider)
    .push(body).push(bodyslider)
    .push(row);

    let padding = Container::new(settingcolumn).padding(50).width(Length::Fill);
    let scroll = Scrollable::new(&mut selfx.settings.scrollable_state).push(padding);
    let main: Element<Message> = Container::new(scroll)
        .padding(50)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into();
    return main;
}

fn makemain(selfx: &mut Buttons) -> Element<Message>{
    let title = h1(String::from("October"));
    let settings = add_button(&mut selfx.gotosetting_state, String::from("Settings"), Message::GotoSettingButton);
    let langs = add_button(&mut selfx.gotolang_state, String::from("Languages"), Message::GotoLangButton);
    let buttons = loadtables(&mut selfx.basic_state, &mut selfx.intermediate_state, &mut selfx.advanced_state);

    let mut maincolumn = Column::new().push(settings).push(title).push(langs);

    for i in buttons  {
        maincolumn = maincolumn.push(i);
    }

    let main: Element<Message> = Container::new(maincolumn)
        .padding(100)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into();
    return main;
}
fn makereview(selfx: &mut Buttons) -> Element<Message>{
    let exit = add_button(&mut selfx.gotomain_state, String::from("Exit"), Message::GotoMainButton);
    let colours = vec![Color::BLACK,Color::from_rgb(1.0, 0.0, 0.0),Color::from_rgb(0.0, 1.0, 0.0)];

    let subtitle1 = h3(String::from("Your Response")).color(colours[COLOUR.lock_mut().unwrap()[0]]).horizontal_alignment(alignment::Horizontal::Center).width(Length::Fill);
    let subtitle2 = h3(String::from("Vietnamese")).horizontal_alignment(alignment::Horizontal::Center).width(Length::Fill);
    let subtitle3 = h3(String::from("English")).horizontal_alignment(alignment::Horizontal::Center).width(Length::Fill);
    
    let response = h4(format!("{}", LETTERS.lock_mut().unwrap().concat())).height(Length::Units(60)).color(colours[COLOUR.lock_mut().unwrap()[0]]);
    let english = h4(format!("{}",ENGLISH.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Units(60));
    let vietnamese = h4(format!("{}",VIETNAMESE.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Units(60));

    let resume = add_button(&mut selfx.gototesting_state, String::from("Resume"), Message::GotoTestingButton);
    let column = Column::new().push(exit).push(subtitle1).push(response).push(subtitle2).push(vietnamese).push(subtitle3).push(english).push(resume);
    let review: Element<Message> = Container::new(column)
        .padding(100)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .into();
    return review;
}

fn makelevel(selfx: &mut Buttons) -> Element<Message>{
    let english = h2(format!("{}",ENGLISH.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Units(150));
    
    let colours = vec![Color::BLACK,Color::from_rgb(1.0, 0.0, 0.0),Color::from_rgb(0.0, 1.0, 0.0)];
    let text1 = h2(format!("{}", LETTERS.lock_mut().unwrap().concat())).height(Length::Units(150)).color(colours[COLOUR.lock_mut().unwrap()[0]]);

    let buttons = [
        add_button(&mut selfx.button_state0, shiftfn(String::from("a")), Message::ButtonPressed0),
        add_button(&mut selfx.button_state1, shiftfn(String::from("b")), Message::ButtonPressed1),
        add_button(&mut selfx.button_state2, shiftfn(String::from("c")), Message::ButtonPressed2),
        add_button(&mut selfx.button_state3, shiftfn(String::from("d")), Message::ButtonPressed3),
    ];

    let exit = add_button(&mut selfx.gotomain_state, String::from("Exit Testing"), Message::GotoMainButton);
    let shift = add_button(&mut selfx.shift_state, String::from("Shift"), Message::ShiftButton);
    let submit = add_button(&mut selfx.submit_state, String::from("submit"), Message::SubmitButton);
    let space = add_button(&mut selfx.space_state, String::from("space"), Message::SpaceButton);
    let delete = add_button(&mut selfx.delete_state, String::from("delete"), Message::DeleteButton);
    let next = add_button(&mut selfx.next_state, String::from("next"), Message::NextButton);

    let mut userrow = Row::new();
    userrow = userrow.push(submit).push(space).push(delete).push(next).push(shift);


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
impl Application for Buttons {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    
    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn new(_flags: ()) -> (Buttons, Command<Message>) {
        (Buttons::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("October")
    }
    
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LoadButton => loadsettings(),
            Message::GotoMainButton => shiftscreenfn(0),
            Message::GotoLangButton => shiftscreenfn(3),
            Message::GotoTestingButton => shiftscreenfn(1),
            Message::GotoSettingButton => shiftscreenfn(4),
            Message::BasicButton => index(0),
            Message::IntermediateButton => index(1),
            Message::AdvancedButton => index(2),
            Message::SubmitButton => sumbitfn(),
            Message::SpaceButton => pushfn(String::from(" ")),
            Message::DeleteButton => popfn(),
            Message::NextButton => nextfn(),
            Message::ShiftButton => shiftvaluefn(),
            Message::LangButton0 => changelang(0),
            Message::LangButton1 => changelang(1),
            Message::ButtonPressed0 => pushfn(String::from("a")),
            Message::ButtonPressed1 => pushfn(String::from("b")),
            Message::ButtonPressed2 => pushfn(String::from("c")),
            Message::ButtonPressed3 => pushfn(String::from("d")),
            Message::SaveButton => savefn(),
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
            Message::SeperateCheckBox(state) => SETTINGS_BOOL.lock_mut().unwrap()[0] = state,
            Message::SoundBox(state) => SETTINGS_BOOL.lock_mut().unwrap()[1] = state,
            Message::TimedBox(state) => SETTINGS_BOOL.lock_mut().unwrap()[2] = state,
            Message::VolumeSlider(num) => SETTINGS_USIZE.lock_mut().unwrap()[0] = num as usize,
            Message::LengthSlider(num) => SETTINGS_USIZE.lock_mut().unwrap()[1] = num as usize,
            Message::H1Slider(num) => SETTINGS_USIZE.lock_mut().unwrap()[2] = num as usize,
            Message::H2Slider(num) => SETTINGS_USIZE.lock_mut().unwrap()[3] = num as usize,
            Message::H3Slider(num) => SETTINGS_USIZE.lock_mut().unwrap()[4] = num as usize,
            Message::H4Slider(num) => SETTINGS_USIZE.lock_mut().unwrap()[5] = num as usize,
            Message::BodySlider(num) => SETTINGS_USIZE.lock_mut().unwrap()[6] = num as usize,
 
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
        } else if SCREEN.lock_mut().unwrap()[0] == 3 {
            return makelang(self);
        } else if SCREEN.lock_mut().unwrap()[0] == 4 {
            return makesettings(self);
        } else {
            return makemain(self);
        }
        
    }
    
}
fn loadsettings() {
    let filename = "./settings.toml";
    let contents = fs::read_to_string(filename).unwrap();

    SETTINGS_BOOL.lock_mut().unwrap().clear();
    SETTINGS_USIZE.lock_mut().unwrap().clear();
    let data: Data = toml::from_str(&contents).unwrap();
    let boollist = 
    [
        data.settings.seperate_check_synonyms,
        data.settings.sound.sound,
        data.settings.time.timed
    ];
    for i in boollist {
        SETTINGS_BOOL.lock_mut().unwrap().push(i);
    }
    let usizelist = 
    [
        data.settings.sound.volume,
        data.settings.time.length,
        data.settings.textsize.h1,
        data.settings.textsize.h2,
        data.settings.textsize.h3,
        data.settings.textsize.h4,
        data.settings.textsize.body,

    ];
    for i in usizelist {
        SETTINGS_USIZE.lock_mut().unwrap().push(i);
    }

}
fn h1(text: String) -> Text {
    return Text::new(text).size(SETTINGS_USIZE.lock_mut().unwrap()[2] as u16);
}
fn h2(text: String) -> Text {
    return Text::new(text).size(SETTINGS_USIZE.lock_mut().unwrap()[3] as u16);
}
fn h3(text: String) -> Text {
    return Text::new(text).size(SETTINGS_USIZE.lock_mut().unwrap()[4] as u16);
}
fn h4(text: String) -> Text {
    return Text::new(text).size(SETTINGS_USIZE.lock_mut().unwrap()[5] as u16);
}
fn body(text: String) -> Text {
    return Text::new(text).size(SETTINGS_USIZE.lock_mut().unwrap()[6] as u16);
}

fn main() -> iced::Result {
    loadsettings();
    let rgba = vec![0, 0, 0, 255];
    TABLE.lock_mut().unwrap().push(0);
    LANG.lock_mut().unwrap().push(0);
    loaddata();
    N.lock_mut().unwrap().push(thread_rng().gen_range(0..ENGLISH.lock_mut().unwrap().len()));
    COLOUR.lock_mut().unwrap().push(0);
    X.lock_mut().unwrap().push(0);
    SCREEN.lock_mut().unwrap().push(0);
    

    let setting: IcedSettings<()> = IcedSettings {
        window: window::Settings {
            size: (800, 600),
            resizable: true,
            decorations: true,
            min_size: Some((100 as u32,100 as u32)),
            max_size: Some((2000 as u32,2000 as u32)),
            transparent: false,
            always_on_top: false,
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
    Buttons::run(setting)

}

