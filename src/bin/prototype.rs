#![feature(map_many_mut)]
use std::{
    fs, io::Write, 
    time::Instant,
    collections::HashMap,
};
use iced::{
    button, Button, 
    Checkbox, Scrollable, 
    Slider, slider, 
    Element, Command, 
    Settings as IcedSettings, 
    Text, Container, 
    Length, Column, 
    Row, window, 
    Color, Application,
    Subscription, executor, 
    alignment, scrollable,
    window::Position::Specific, window::Icon
};
use std::sync::atomic::{AtomicU32, Ordering};
use global::Global;
use rand::{thread_rng, Rng};
use iced_native::{Event, keyboard};
use sqlite::State as SqlState;
use serde_derive::Deserialize;
use toml;


// Global variables for storing the letters to be typed, the English and Vietnamese words,
// and other program state such as the current screen, language, and time.

static LETTERS: Global<Vec<String>> = Global::new();
static ENGLISH: Global<Vec<String>> = Global::new();
static VIETNAMESE: Global<Vec<String>> = Global::new();
static SETTINGS_USIZE: Global<Vec<usize>> = Global::new();
static SETTINGS_BOOL: Global<Vec<bool>> = Global::new();
static TIME: Global<Vec<Instant>> = Global::new();

static N: AtomicU32 = AtomicU32::new(0);
static COLOUR:AtomicU32 = AtomicU32::new(0);
static X: AtomicU32 = AtomicU32::new(0);
static SCREEN: AtomicU32 = AtomicU32::new(0);
static TABLE: AtomicU32 = AtomicU32::new(0);
static LANG: AtomicU32 = AtomicU32::new(0);



// Struct for holding the program's settings, including text size and time limit settings.
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


struct Buttons {
    button_states: HashMap<&'static str, button::State>,
    letter_states: HashMap<&'static str, button::State>,
    settings: SettingButtons,
}

impl Default for Buttons {
    fn default() -> Self {
        Self {
            button_states: {
                let mut map = HashMap::new();
                let list = [
                    "gotomain", "gototesting","gotolang","gotosetting", 
                    "shift", "next", "submit", "space", "delete", 
                    "save", "button_state0", "button_state1", "button_state2", 
                    "button_state3", "lang_state0", "lang_state1", "basic",
                    "intermediate", "advanced"
                ];
                for i in list{
                    map.insert(i,button::State::default());
                }
                map
            },
            letter_states: {
                let mut map: HashMap<&str, button::State> = HashMap::new();
                for i in 0..26{
                    map.insert(format!("state{}",i).as_str(),button::State::default());
                }
                map
            },
            settings: SettingButtons::default(),
        }
    }
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
    ButtonPressed(String),
    LetterPressed(String),
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
    //println!("ADDED {} TO {}", letter,LETTERS.lock_mut().unwrap().concat());
    COLOUR.store(0, Ordering::SeqCst)
}

fn shiftfn(letter: String) -> String {
    if X.load(Ordering::Relaxed) == 1 {
        return letter.to_uppercase();
    } else {
        return letter.to_lowercase();
    }
}

fn shiftvaluefn() {
    if X.load(Ordering::Relaxed) == 1 {
        X.store(0, Ordering::SeqCst) ;
    } else {
        X.store(1, Ordering::SeqCst);
    }
}

fn shiftscreenfn(destination: usize) {
    TIME.lock_mut().unwrap().clear();
    TIME.lock_mut().unwrap().push(Instant::now());
    SCREEN.store(destination.try_into().unwrap(), Ordering::SeqCst);
    N.store(thread_rng().gen_range(0..ENGLISH.lock_mut().unwrap().len().try_into().unwrap()), Ordering::SeqCst);
    LETTERS.lock_mut().unwrap().clear();
    COLOUR.store(0, Ordering::SeqCst);
}

fn sumbitfn() {
    //println!("{:?}",LETTERS.lock_mut().unwrap().concat());
    
    if format!("{}", LETTERS.lock_mut().unwrap().concat()) == VIETNAMESE.lock_mut().unwrap()[N.load(Ordering::Relaxed) as usize]{
        //println!("true");
        COLOUR.store(2, Ordering::SeqCst);
        SCREEN.store(2, Ordering::SeqCst);
    } else {
        //println!("false");
        COLOUR.store(1, Ordering::SeqCst);
        SCREEN.store(2, Ordering::SeqCst);
    }
}

fn popfn() {
    if LETTERS.lock_mut().unwrap().len() != 0 {
        LETTERS.lock_mut().unwrap().pop();
        //println!("{}",LETTERS.lock_mut().unwrap().concat());
        COLOUR.store(0, Ordering::SeqCst);
    } 
    
}

fn nextfn() {
    TIME.lock_mut().unwrap().clear();
    TIME.lock_mut().unwrap().push(Instant::now());
    //println!("{}", ENGLISH.lock_mut().unwrap().len());
    N.store(thread_rng().gen_range(0..ENGLISH.lock_mut().unwrap().len().try_into().unwrap()), Ordering::SeqCst);
    LETTERS.lock_mut().unwrap().clear();
    COLOUR.store(0, Ordering::SeqCst);
}

fn add_button<'a>(state: &'a mut button::State,content: String,message: Message) -> Button<'a, Message> {
    return Button::new(state, body(content)).on_press(message);
}


fn index(num: usize) {
    let mut languages: Vec<String> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        languages.push(file.unwrap().path().display().to_string())
    }
    let connection = sqlite::open(
        format!(
            "{}", languages[LANG.load(Ordering::Relaxed) as usize]
        )
    ).unwrap();
    let mut statement2 = connection
    .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
    .unwrap();
    let mut tables: Vec<String> = Vec::new();
    while let Ok(State::Row) = statement2.next() {
        tables.push(statement2.read::<String>(0).unwrap())
    }
    if num < tables.len() {
        SCREEN.store(1, Ordering::SeqCst);
        TABLE.store(num.try_into().unwrap(), Ordering::SeqCst);
        loaddata();
        nextfn();
    }
}
fn loadtables<'a>(self1: &'a mut button::State, self2: &'a mut button::State, self3: &'a mut button::State) -> Vec<Button<'a, Message>> {
    if LANG.load(Ordering::Relaxed) == 0 {
        return vec![
            add_button(self1, String::from("Enter basic"), Message::ButtonPressed("Basic".to_string())), 
            add_button(self2, String::from("Enter intermediate"), Message::ButtonPressed("Intermediate".to_string())),
            add_button(self3, String::from("Enter advanced"), Message::ButtonPressed("Advanced".to_string()))
            ];
    } else if LANG.load(Ordering::Relaxed) == 1 {
        return vec![
            add_button(self1, String::from("Enter basic"), Message::ButtonPressed("Basic".to_string())), 
            add_button(self2, String::from("Enter intermediate"), Message::ButtonPressed("Intermediate".to_string())),           
            ];
    } else {
        return vec![
            add_button(self1, String::from("Enter basic"), Message::ButtonPressed("Basic".to_string())), 
            add_button(self2, String::from("Enter intermediate"), Message::ButtonPressed("Intermediate".to_string())),          
            ];
    }
}
fn loaddata() {
    let mut languages: Vec<String> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        //println!("{}", file.unwrap().path().display());
        languages.push(file.unwrap().path().display().to_string())
    }
    let connection = sqlite::open(format!("{}", languages[LANG.load(Ordering::Relaxed) as usize])).unwrap();
    
    let mut statement2 = connection
    .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
    .unwrap();
    let mut tables: Vec<String> = Vec::new();

    while let Ok(State::Row) = statement2.next() {
        //println!("{}", statement2.read::<String>(0).unwrap());
        tables.push(statement2.read::<String>(0).unwrap())
    }
    let mut statement = connection
        .prepare(format!("SELECT * FROM {}", tables[TABLE.load(Ordering::Relaxed) as usize]))
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
    LANG.store(num.try_into().unwrap(), Ordering::SeqCst);
    shiftscreenfn(0);
    loaddata();
}

fn makelang<'a>(selfx: &'a mut Buttons) -> Element<'a, Message>{
    let [state0, state1] = selfx.button_states.get_many_mut(["lang_state0", "lang_state1"]).unwrap();
    let lang0 = add_button(state0, String::from("Lang0"), Message::ButtonPressed("Lang0".to_string()));
    let lang1 = add_button(state1, String::from("Lang1"), Message::ButtonPressed("Lang1".to_string()));

/*     let lang0 = add_button(selfx.button_states.get_mut("lang_state0").unwrap(), String::from("Lang0"), Message::LangButton0);
    let lang1 = add_button(selfx.button_states.get_mut("lang_state1").unwrap(), String::from("Lang1"), Message::LangButton1);
 */    let langcolumn = Column::new().push(lang0).push(lang1);
    let main: Element<'a, Message> = Container::new(langcolumn)
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
    let [state0, state1] = selfx.button_states.get_many_mut(["save", "gotomain"]).unwrap();

    let save = add_button(state0, String::from("Save"), Message::ButtonPressed("Save".to_string())); 
    let exit = add_button(state1, String::from("Exit"), Message::ButtonPressed("Exit".to_string()));
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
    let [state0, state1, state2, state3, state4] = selfx.button_states.get_many_mut(["gotosetting", "gotolang", "basic", "intermediate", "advanced"]).unwrap();

    let settings = add_button(state0, String::from("Settings"), Message::ButtonPressed("Settings".to_string()));
    let langs = add_button(state1, String::from("Languages"), Message::ButtonPressed("Languages".to_string()));
    let buttons = loadtables(state2, state3, state4);

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
    let [state0, state1] = selfx.button_states.get_many_mut(["gotomain", "gototesting"]).unwrap();

    let exit = add_button(state0, String::from("Exit"), Message::ButtonPressed("Exit".to_string()));
    let colours = vec![Color::BLACK,Color::from_rgb(1.0, 0.0, 0.0),Color::from_rgb(0.0, 1.0, 0.0)];

    let subtitle1 = h3(String::from("Your Response"))
        .color(colours[COLOUR.load(Ordering::Relaxed) as usize])
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(Length::Fill);

    let subtitle2 = h3(String::from("Vietnamese"))
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(Length::Fill);

    let subtitle3 = h3(String::from("English"))
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(Length::Fill);
    
    let response = h4(format!("{}", LETTERS.lock_mut().unwrap().concat()))
        .height(Length::Units(60))
        .color(colours[COLOUR.load(Ordering::Relaxed) as usize]);
    let english = h4(format!("{}",ENGLISH.lock_mut().unwrap()[N.load(Ordering::Relaxed) as usize] ))
        .height(Length::Units(60));
    let vietnamese = h4(format!("{}",VIETNAMESE.lock_mut().unwrap()[N.load(Ordering::Relaxed) as usize] ))
        .height(Length::Units(60));

    let resume = add_button(state1, String::from("Resume"), Message::ButtonPressed("Resume".to_string()));
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
    let timer = h4(
        format!("{:.2}",  
        (SETTINGS_USIZE.lock_mut().unwrap()[1] as f64 - TIME.lock_mut().unwrap()[0].elapsed().as_secs_f64()))
    );
    
    if TIME.lock_mut().unwrap()[0].elapsed().as_secs() >= SETTINGS_USIZE.lock_mut().unwrap()[1] as u64 {
        sumbitfn()
    }
    let english = h2(format!("{}",ENGLISH.lock_mut().unwrap()[N.load(Ordering::Relaxed) as usize] ))
    .height(Length::Units(150));
    
    let colours = vec![Color::BLACK,Color::from_rgb(1.0, 0.0, 0.0),Color::from_rgb(0.0, 1.0, 0.0)];
    let text1 = h2(format!("{}", LETTERS.lock_mut().unwrap().concat()))
    .height(Length::Units(150))
    .color(colours[COLOUR.load(Ordering::Relaxed) as usize]);
    let [state0, state1, state2, state3, state4, state5, state6, state7, state8, state9] = selfx.button_states.get_many_mut(["button_state0", "button_state1", "button_state2", "button_state3", "gotomain","shift","submit", "space", "delete","next"]).unwrap();
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

    
    let letterstates:[&mut ; 26] = selfx.letter_states.get_many_mut(letters).unwrap();
   

    let buttons = [
        add_button(state0, shiftfn(String::from("a")), Message::LetterPressed("a".to_string())),
        add_button(state1, shiftfn(String::from("b")), Message::LetterPressed("b".to_string())),
        add_button(state2, shiftfn(String::from("c")), Message::LetterPressed("c".to_string())),
        add_button(state3, shiftfn(String::from("d")), Message::LetterPressed("d".to_string())),
    ];

    let exit = add_button(state4, String::from("Exit Testing"), Message::ButtonPressed("Exit".to_string()));
    let shift = add_button(state5, String::from("Shift"), Message::ButtonPressed("Shift".to_string()));
    let submit = add_button(state6, String::from("submit"), Message::ButtonPressed("Submit".to_string()));
    let space = add_button(state7, String::from("space"), Message::ButtonPressed("Space".to_string()));
    let delete = add_button(state8, String::from("delete"), Message::ButtonPressed("Delete".to_string()));
    let next = add_button(state9, String::from("next"), Message::ButtonPressed("Next".to_string()));

    let mut userrow = Row::new();
    userrow = userrow.push(submit).push(space).push(delete).push(next).push(shift);


    let mut row1 = Row::new();
    for button in buttons {
        row1 = row1.push(button);
    };
    let utilrow = Row::new().push(timer).push(exit);
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
            Message::ButtonPressed(string) => {
                match string.as_str() {
                    "Basic" => index(0),
                    "Intermediate" => index(1),
                    "Advanced" => index(2),
                    "Lang0" => changelang(0),
                    "Lang1" => changelang(1),
                    "Save" => savefn(),
                    "Exit" => shiftscreenfn(0),
                    "Settings" => shiftscreenfn(4),
                    "Languages" => shiftscreenfn(3),
                    "Resume" => shiftscreenfn(1),
                    "Shift" => shiftvaluefn(),
                    "Submit" => sumbitfn(),
                    "Space" => pushfn(String::from(" ")),
                    "Delete" => popfn(),
                    "Next" => nextfn(),
                    _ => (),
                }
            }
            Message::LetterPressed(string) => pushfn(string),
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
        if SCREEN.load(Ordering::Relaxed) == 0 {
            return makemain(self);
        } else if SCREEN.load(Ordering::Relaxed) == 1 {
            return makelevel(self);
        } else if SCREEN.load(Ordering::Relaxed) == 2 {
            return makereview(self);
        } else if SCREEN.load(Ordering::Relaxed) == 3 {
            return makelang(self);
        } else if SCREEN.load(Ordering::Relaxed) == 4 {
            return makesettings(self);
        } else {
            return makemain(self);
        }
        
    }
    
}
pub fn loadsettings() {
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
    loaddata();
    N.store(thread_rng().gen_range(0..ENGLISH.lock_mut().unwrap().len()).try_into().unwrap(), Ordering::SeqCst);
    TIME.lock_mut().unwrap().push(Instant::now());

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
        antialiasing: false,
        id: Some("October".to_string()),
        flags: (),
        default_text_size: 20,
        text_multithreading: true,
        exit_on_close_request: true,
        try_opengles_first: false,
    };
    Buttons::run(setting)

}

