#![feature(map_many_mut)]
use std::{
    fs, io::Write, 
    time::Instant,
    collections::HashMap,
    convert::TryInto,
    sync::atomic::{AtomicU32, Ordering},
};
use iced::{
    button::{self}, Button, 
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
use futures::executor::block_on;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use iced_native::{Event, keyboard};
use sqlite::State as SqlState;
use toml::{self};
use tokio::sync::RwLock;


// Global variables for storing the letters to be typed, the English and Vietnamese words,
// and other program state such as the current screen, language, and time.

//static LETTERS: Global<Vec<String>> = Global::new();
//static ENGLISH: Global<Vec<String>> = Global::new();
//static VIETNAMESE: Global<Vec<String>> = Global::new();
//static SETTINGS_USIZE: Global<Vec<usize>> = Global::new();
//static SETTINGS_BOOL: Global<Vec<bool>> = Global::new();
//static TIME: Global<Vec<Instant>> = Global::new();

lazy_static! {
    static ref LETTERS: RwLock<Vec<String>> = RwLock::new(Vec::new());
    static ref ENGLISH: RwLock<Vec<String>> = RwLock::new(Vec::new());
    static ref VIETNAMESE: RwLock<Vec<String>> = RwLock::new(Vec::new());
    static ref SETTINGS_USIZE: RwLock<Vec<usize>> = RwLock::new(Vec::new());
    static ref SETTINGS_BOOL: RwLock<Vec<bool>> = RwLock::new(Vec::new());
    static ref TIME: RwLock<Instant> = RwLock::new(Instant::now());
}

static N: AtomicU32 = AtomicU32::new(0);
static COLOUR:AtomicU32 = AtomicU32::new(0);
static X: AtomicU32 = AtomicU32::new(0);
static SCREEN: AtomicU32 = AtomicU32::new(0);
static TABLE: AtomicU32 = AtomicU32::new(0);
static LANG: AtomicU32 = AtomicU32::new(0);
static MAXSTATES: AtomicU32 = AtomicU32::new(0);
const LANGONE: [&str;26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
const LANGTWO: [&str;33] = [
    "ẳ","á","â","à","ạ","ầ","ậ", "ấ","ả","ặ",
    "đ",
    "ỏ","ơ","ờ","ồ","ó","ô","ọ","ộ","ớ","ở",
    "ư","ụ","ữ","ú", "ủ",
    "í","ì","ị",
    "ế","ẹ", "ể", "ề"
];
const PUNCTUATION: [&str;8] = ["(",")", ";", ":", ",", ".", "?", "!"];

mod settings;

use settings::{Data};


struct Buttons {
    button_states: HashMap<&'static str, button::State>,
    lang_one_states: HashMap<&'static str, button::State>,
    lang_two_states: HashMap<&'static str, button::State>,
    punctuation_states: HashMap<&'static str, button::State>,
    language_states: HashMap<String, button::State>,
    table_states: HashMap<String, button::State>,
    settings: SettingButtons,
}

impl Default for Buttons {
    fn default() -> Self {
        Self {
            button_states: {
                let mut map = HashMap::new();
                
                let list = [
                    "gotomain", "gototesting","gotolang","gotosetting", "gotodata", 
                    "shift", "next", "submit", "space", "delete", 
                    "save"
                ];
                for i in list{
                    map.insert(i,button::State::default());
                }
                map
            },
            lang_one_states: {
                let mut map = HashMap::new();

                // Add characters from LANGONE to the map
                for i in LANGONE {
                    map.insert(i, button::State::default());
                }
                
                // Add characters from LANGTWO to the map
                for i in LANGTWO {
                    map.insert(i, button::State::default());
                }
                
                // Add characters from PUNCTUATION to the map
                for i in PUNCTUATION {
                    map.insert(i, button::State::default());
                }
                map
                    
            },
            lang_two_states: {
                let mut map = HashMap::new();        
                // Add characters from LANGTWO to the map
                for i in LANGTWO {
                    map.insert(i, button::State::default());
                }
                map
                    
            },
            punctuation_states: {
                let mut map = HashMap::new();
                // Add characters from PUNCTUATION to the map
                for i in PUNCTUATION {
                    map.insert(i, button::State::default());
                }
                map
                    
            },
            language_states: {
                let mut map = HashMap::new();
                let mut vec = Vec::new();
                for i in 0..loadamount() {
                    vec.push(format!("state{}",i));
                }
                for i in vec {
                    map.insert(i, button::State::default());
                }
                map
                    
            },
            table_states: {
                let mut map = HashMap::new();
                let mut vec = Vec::new();
                for i in 0..MAXSTATES.load(Ordering::SeqCst) {
                    vec.push(format!("state{}",i));
                }
                for i in vec {
                    map.insert(i, button::State::default());
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
    LangChange(usize),
    IndexSent(usize),
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
    let mut blocking_letters = LETTERS.blocking_write();
    blocking_letters.push(shiftfn(letter.to_string()));
    drop(blocking_letters);
    //println!("ADDED {} TO {}", letter,LETTERS.lock_mut().unwrap().concat());
    COLOUR.store(0, Ordering::SeqCst)
}

fn shiftfn(letter: String) -> String {
    if X.load(Ordering::SeqCst) == 1 {
        return letter.to_uppercase();
    } else {
        return letter.to_lowercase();
    }
}

fn shiftvaluefn() {
    if X.load(Ordering::SeqCst) == 1 {
        X.store(0, Ordering::SeqCst) ;
    } else {
        X.store(1, Ordering::SeqCst);
    }
}

fn shiftscreenfn(destination: usize) {
    let mut blocking_task = TIME.blocking_write();
    *blocking_task = Instant::now();
    drop(blocking_task);
    SCREEN.store(destination.try_into().unwrap(), Ordering::SeqCst);
    let blocking_english = ENGLISH.blocking_read();
    N.store(thread_rng().gen_range(0..blocking_english.len().try_into().unwrap()), Ordering::SeqCst);
    let mut blocking_letters = LETTERS.blocking_write();
    blocking_letters.clear();
    COLOUR.store(0, Ordering::SeqCst);
    drop(blocking_english);
    drop(blocking_letters);
}

fn sumbitfn() {
    //println!("{:?}",LETTERS.lock_mut().unwrap().concat());
    let blocking_letters = LETTERS.blocking_read();
    let blocking_vietnamese = VIETNAMESE.blocking_write();
    if format!("{}", blocking_letters.concat()) == blocking_vietnamese[N.load(Ordering::SeqCst) as usize]{
        //println!("true");
        COLOUR.store(2, Ordering::SeqCst);
        SCREEN.store(2, Ordering::SeqCst);
    } else {
        //println!("false");
        COLOUR.store(1, Ordering::SeqCst);
        SCREEN.store(2, Ordering::SeqCst);
    }
    drop(blocking_letters);
    drop(blocking_vietnamese);
}

fn popfn() {
    let blocking_letters = LETTERS.blocking_read();

    if blocking_letters.len() != 0 {
        drop(blocking_letters);
        let mut blocking_letters = LETTERS.blocking_write();

        blocking_letters.pop();
        //println!("{}",LETTERS.lock_mut().unwrap().concat());
        COLOUR.store(0, Ordering::SeqCst);
        drop(blocking_letters);
    } 
    
}

fn nextfn() {
    let mut blocking_letters = LETTERS.blocking_write();
    let mut blocking_task = TIME.blocking_write();
    *blocking_task = Instant::now();
    drop(blocking_task);
    //println!("{}", ENGLISH.lock_mut().unwrap().len());
    let blocking_english = ENGLISH.blocking_read();

    N.store(thread_rng().gen_range(0..blocking_english.len().try_into().unwrap()), Ordering::SeqCst);
    blocking_letters.clear();
    COLOUR.store(0, Ordering::SeqCst);
    drop(blocking_english);
    drop(blocking_letters);
}

async fn add_button<'a>(state: &'a mut button::State,content: String,message: Message) -> Button<'a, Message> {
    return Button::new(state, body(content)).on_press(message);
}


fn index(num: usize) {
    let mut languages: Vec<String> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        languages.push(file.unwrap().path().display().to_string())
    }
        // Sort the languages alphabetically
    languages.sort_by(|a, b| a.cmp(b));

/*     // Print the sorted languages
    for language in &languages {
        println!("{}", language);
    } */
    let connection = sqlite::open(
        format!(
            "{}", languages[LANG.load(Ordering::SeqCst) as usize]
        )
    ).unwrap();
    let mut statement2 = connection
    .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
    .unwrap();
    let mut tables: Vec<String> = Vec::new();
    while let Ok(SqlState::Row) = statement2.next() {
        tables.push(statement2.read::<String>(0).unwrap())
    }
    if num < tables.len() {
        SCREEN.store(1, Ordering::SeqCst);
        TABLE.store(num.try_into().unwrap(), Ordering::SeqCst);
        loaddata();
        nextfn();
    }
}

fn tablenames() -> Vec<String> {
    let mut languages: Vec<String> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        //println!("{}", file.unwrap().path().display());
        languages.push(file.unwrap().path().display().to_string())
    }

    let connection = sqlite::open(format!("{}", languages[LANG.load(Ordering::SeqCst) as usize])).unwrap();
    let mut statement2 = connection
    .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
    .unwrap();

    while let Ok(SqlState::Row) = statement2.next() {
        //println!("{}", statement2.read::<String>(0).unwrap());
        names.push(statement2.read::<String>(0).unwrap())
    }
    return names;
    

}
fn loadsizes() {
    let mut languages: Vec<String> = Vec::new();
    let mut templangvec: Vec<u32> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        //println!("{}", file.unwrap().path().display());
        languages.push(file.unwrap().path().display().to_string())
    }

    for i in languages {
        let connection = sqlite::open(format!("{}", i)).unwrap();
        let mut statement2 = connection
        .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
        .unwrap();
        let mut tables: Vec<String> = Vec::new();
    
        while let Ok(SqlState::Row) = statement2.next() {
            //println!("{}", statement2.read::<String>(0).unwrap());
            tables.push(statement2.read::<String>(0).unwrap())
        }
        templangvec.push(tables.len() as u32)
    }

    MAXSTATES.store(*templangvec.iter().max().unwrap(), Ordering::SeqCst)
}

fn loadnames() -> Vec<String> {
    let mut languages: Vec<String> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        //println!("{}", file.unwrap().path().display());
        languages.push
        (
            file
            .unwrap()
            .path()
            .display()
            .to_string()
            .strip_prefix("./resources/languages/").unwrap()
            .strip_suffix(".sqlite3").unwrap()
            .to_string()
        )
    }
    // for i in languages {
    //     println!("{}",i.strip_prefix("./resources/languages/").unwrap().strip_suffix(".sqlite3").unwrap())
    // }
    return languages;

}
fn loadamount() -> usize {
    let mut languages: Vec<String> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        //println!("{}", file.unwrap().path().display());
        languages.push
        (
            file
            .unwrap()
            .path()
            .display()
            .to_string()
            .strip_prefix("./resources/languages/").unwrap()
            .strip_suffix(".sqlite3").unwrap()
            .to_string()
        )
    }
    // for i in languages {
    //     println!("{}",i.strip_prefix("./resources/languages/").unwrap().strip_suffix(".sqlite3").unwrap())
    // }
    return languages.len();

}


fn loaddata() {
    let mut languages: Vec<String> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        //println!("{}", file.unwrap().path().display());
        languages.push(file.unwrap().path().display().to_string())
    }
    let connection = sqlite::open(format!("{}", languages[LANG.load(Ordering::SeqCst) as usize])).unwrap();
    
    let mut statement2 = connection
    .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
    .unwrap();
    let mut tables: Vec<String> = Vec::new();

    while let Ok(SqlState::Row) = statement2.next() {
        //println!("{}", statement2.read::<String>(0).unwrap());
        tables.push(statement2.read::<String>(0).unwrap())
    }
    let mut statement = connection
        .prepare(format!("SELECT * FROM {}", tables[TABLE.load(Ordering::SeqCst) as usize]))
        .unwrap();

    let mut vietnamese: Vec<String> = Vec::new();
    let mut english: Vec<String> = Vec::new();
    let mut typename: Vec<String> = Vec::new();

    while let Ok(SqlState::Row) = statement.next() {
        //println!("{} = {}", statement.read::<String>(0).unwrap(), statement.read::<String>(1).unwrap());
        //println!("{}",statement.read::<String>(0).unwrap());
        english.push(statement.read::<String>(0).unwrap());
        vietnamese.push(statement.read::<String>(1).unwrap());
        typename.push(statement.read::<String>(2).unwrap());
    }
    let mut blocking_english = ENGLISH.blocking_write();

    blocking_english.clear();
    for i in english {
        //println!("{}",i);

        blocking_english.push(i);
        
    }
    let mut blocking_vietnamese = VIETNAMESE.blocking_write();
    for i in vietnamese {
        blocking_vietnamese.push(i)
    }
    drop(blocking_vietnamese);
    drop(blocking_english);
}
fn changelang(num: usize) {
    LANG.store(num.try_into().unwrap(), Ordering::SeqCst);
    shiftscreenfn(0);
    loaddata();
}

fn makelang<'a>(selfx: &'a mut Buttons) -> Element<'a, Message>{
    let mut x = 0;
    let mut buttons: Vec<Button<Message>>=Vec::new();
    let names = loadnames();
    for i in selfx.language_states.values_mut() {
        buttons.push(block_on(add_button(i, names[x].to_string(), Message::LangChange(x))));
        x +=1;
    };

/*     let lang0 = add_button(selfx.button_states.get_mut("lang_state0").unwrap(), String::from("Lang0"), Message::LangButton0);
    let lang1 = add_button(selfx.button_states.get_mut("lang_state1").unwrap(), String::from("Lang1"), Message::LangButton1);
 */    
    let mut langcolumn = Column::new();
    for i in buttons {
        langcolumn = langcolumn.push(i);
    }
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
    let blocking_task = SETTINGS_USIZE.blocking_read();
    let blocking_bool = SETTINGS_BOOL.blocking_read();
    let mut file = std::fs::File::create("./settings.toml").expect("create failed");
    file.write_all(format!("[settings]\n").as_bytes()).expect("write failed");
    file.write_all(format!("seperate_check_synonyms = {} # 0\n", blocking_bool[0]).as_bytes()).expect("write failed");
    file.write_all(format!("\n").as_bytes()).expect("write failed");
    file.write_all(format!("[settings.sound]\n").as_bytes()).expect("write failed");
    file.write_all(format!("sound =  {} # 1\n", blocking_bool[1]).as_bytes()).expect("write failed");
    file.write_all(format!("volume = {} # 0\n", blocking_task[0]).as_bytes()).expect("write failed");
    file.write_all(format!("\n").as_bytes()).expect("write failed");

    file.write_all(format!("[settings.time]\n").as_bytes()).expect("write failed");
    file.write_all(format!("timed =  {} # 2\n", blocking_bool[2]).as_bytes()).expect("write failed");
    file.write_all(format!("length = {} # 1\n", blocking_task[1]).as_bytes()).expect("write failed");
    file.write_all(format!("\n").as_bytes()).expect("write failed");

    file.write_all(format!("[settings.textsize]\n").as_bytes()).expect("write failed");
    file.write_all(format!("h1 = {} # 2\n", blocking_task[2]).as_bytes()).expect("write failed");
    file.write_all(format!("h2 = {} # 3\n", blocking_task[3]).as_bytes()).expect("write failed");
    file.write_all(format!("h3 = {} # 4\n", blocking_task[4]).as_bytes()).expect("write failed");
    file.write_all(format!("h4 = {} # 5\n", blocking_task[5]).as_bytes()).expect("write failed");
    file.write_all(format!("body = {} # 6\n", blocking_task[6]).as_bytes()).expect("write failed");
    file.write_all(format!("\n").as_bytes()).expect("write failed");
    drop(blocking_task)

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
    let blocking_task = SETTINGS_USIZE.blocking_read();
    let blocking_bool = SETTINGS_BOOL.blocking_read();

    let h2_general= h2(String::from("General Settings"));
    let seperatecheckbox = Checkbox::new(blocking_bool[0], "Seperately check synonyms", Message::SeperateCheckBox);
    let h2_sound = h2(String::from("Sound Settings"));
    let soundbox = Checkbox::new(blocking_bool[1], "Sound Enabled", Message::SoundBox);
    let volumeslider = makeslider!(&mut selfx.settings.volume, 100, blocking_task[0], Message::VolumeSlider);
    let h2_time = h2(String::from("Time Settings"));
    let lengthbox = Checkbox::new(blocking_bool[2], "Timed", Message::TimedBox);
    let lengthslider = makeslider!(&mut selfx.settings.length, 1, 30, blocking_task[1], Message::LengthSlider);
    let h2_text= h2(String::from("Text Settings"));
    let h1slider = makeslider!(&mut selfx.settings.h1, 200, blocking_task[2], Message::H1Slider);
    let h2slider = makeslider!(&mut selfx.settings.h2, 200, blocking_task[3], Message::H2Slider);
    let h3slider = makeslider!(&mut selfx.settings.h3, 200, blocking_task[4], Message::H3Slider);
    let h4slider = makeslider!(&mut selfx.settings.h4, 200, blocking_task[5], Message::H4Slider);
    let bodyslider = makeslider!(&mut selfx.settings.body, 200, blocking_task[6], Message::BodySlider);
    let volume = h4(String::from(format!("Volume: {}", blocking_task[0])));
    let length = h4(String::from(format!("Time: {}", blocking_task[1])));
    let h1 = h1(String::from(format!("H1: {}", blocking_task[2])));
    let h2 = h2(String::from(format!("H2: {}", blocking_task[3])));
    let h3 = h3(String::from(format!("H3: {}", blocking_task[4])));
    let h4 = h4(String::from(format!("H4: {}", blocking_task[5])));
    let body = body(String::from(format!("Body: {}", blocking_task[6])));
    let reset = add_button(&mut selfx.settings.load_state, String::from("Reload Default"), Message::LoadButton);
    let [state0, state1] = selfx.button_states.get_many_mut(["save", "gotomain"]).unwrap();

    let save = add_button(state0, String::from("Save"), Message::ButtonPressed("Save".to_string())); 
    let exit = add_button(state1, String::from("Exit"), Message::ButtonPressed("Exit".to_string()));
    let row = Row::new().push(block_on(save)).push(block_on(exit)).push(block_on(reset));

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
    drop(blocking_task);
    return main;
}
fn loadsize() -> usize {
    let mut languages: Vec<String> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        //println!("{}", file.unwrap().path().display());
        languages.push(file.unwrap().path().display().to_string())
    }
    let connection = sqlite::open(format!("{}", languages[LANG.load(Ordering::SeqCst) as usize])).unwrap();
    
    let mut statement2 = connection
    .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
    .unwrap();
    let mut tables: Vec<String> = Vec::new();

    while let Ok(SqlState::Row) = statement2.next() {
        //println!("{}", statement2.read::<String>(0).unwrap());
        tables.push(statement2.read::<String>(0).unwrap())
    }
    return tables.len();
}


fn makemain(selfx: &mut Buttons) -> Element<Message>{
    let title = h1(String::from("October"));
    let [state0, state1, state2] = selfx.button_states.get_many_mut(["gotosetting", "gotolang", "gotodata"]).unwrap();

    let settings = add_button(state0, String::from("Settings"), Message::ButtonPressed("Settings".to_string()));
    let langs = add_button(state1, String::from("Languages"), Message::ButtonPressed("Languages".to_string()));
    let data = add_button(state2, String::from("Data"), Message::ButtonPressed("Data".to_string()));

    let mut x = 0;

    let mut maincolumn = Column::new().push(block_on(settings)).push(title).push(block_on(langs)).push(block_on(data));

    let mut buttons: Vec<Button<Message>>=Vec::new();
    let names = tablenames();
    for i in selfx.table_states.values_mut() {
        if x >= loadsize() {
        } else {
            buttons.push(block_on(add_button(i, names[x].to_string(), Message::IndexSent(x))));
            x +=1;
        }
    };

    for i in buttons {
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
fn makedata(selfx: &mut Buttons) -> Element<Message>{
    let state = selfx.button_states.get_mut("gotomain").unwrap();

    let n = Text::new(format!("N: {:?}", N));
    let table = Text::new(format!("table: {:?}", TABLE));
    let lang = Text::new(format!("Lang: {:?}", LANG));
    let blocking_english = ENGLISH.blocking_read();

    let english = h4(format!("{}",blocking_english[N.load(Ordering::SeqCst) as usize] ))
        .height(Length::Units(60));

    let blocking_vietnamese = VIETNAMESE.blocking_read();

    let vietnamese = h4(format!("{}",blocking_vietnamese[N.load(Ordering::SeqCst) as usize] ))
        .height(Length::Units(60));
    drop(blocking_vietnamese);
    let maincolumn = Column::new().push(n).push(table).push(lang).push(english).push(vietnamese);
    let exit = add_button(state, "exit".to_string(), Message::ButtonPressed("Exit".to_string()));
    let main: Element<Message> = Container::new(maincolumn.push(block_on(exit)))
        .padding(100)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into();
    drop(blocking_english);
    return main;
}
fn makereview(selfx: &mut Buttons) -> Element<Message>{
    let blocking_english = ENGLISH.blocking_read();

    let [state0, state1] = selfx.button_states.get_many_mut(["gotomain", "gototesting"]).unwrap();

    let exit = add_button(state0, String::from("Exit"), Message::ButtonPressed("Exit".to_string()));
    let colours = vec![Color::BLACK,Color::from_rgb(1.0, 0.0, 0.0),Color::from_rgb(0.0, 1.0, 0.0)];

    let subtitle1 = h3(String::from("Your Response"))
        .color(colours[COLOUR.load(Ordering::SeqCst) as usize])
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(Length::Fill);

    let subtitle2 = h3(String::from("Vietnamese"))
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(Length::Fill);

    let subtitle3 = h3(String::from("English"))
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(Length::Fill);

    let blocking_letters = LETTERS.blocking_read();

    let response = h4(format!("{}", blocking_letters.concat()))
        .height(Length::Units(60))
        .color(colours[COLOUR.load(Ordering::SeqCst) as usize]);
    let english = h4(format!("{}",blocking_english[N.load(Ordering::SeqCst) as usize] ))
        .height(Length::Units(60));

    let blocking_vietnamese = VIETNAMESE.blocking_read();

    let vietnamese = h4(format!("{}",blocking_vietnamese[N.load(Ordering::SeqCst) as usize] ))
        .height(Length::Units(60));
    drop(blocking_vietnamese);

    let resume = add_button(state1, String::from("Resume"), Message::ButtonPressed("Resume".to_string()));
    let column = Column::new()
        .push(block_on(exit))
        .push(subtitle1)
        .push(response)
        .push(subtitle2)
        .push(vietnamese)
        .push(subtitle3)
        .push(english)
        .push(block_on(resume));
    let review: Element<Message> = Container::new(column)
        .padding(100)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .into();

    drop(blocking_english);
    drop(blocking_letters);
    return review;
}

async fn get_time() -> String {
    // Borrow values from RwLock instead of cloning them
    let settings = SETTINGS_USIZE.read().await;
    let time = TIME.read().await;

    // Cache elapsed time value in a local variable
    let elapsed = time.elapsed().as_secs_f32();

    // Calculate and return result
    let new_time = settings[1] as f32 - elapsed;
    format!("{:.2}", new_time)
}

fn write_rwlock_usize(index: usize, num: i32) {
    let mut blocking_task = SETTINGS_USIZE.blocking_write();
    blocking_task[index] = num as usize;
    drop(blocking_task);
}
fn write_rwlock_bool(index: usize, bool: bool) {
    let mut blocking_task = SETTINGS_BOOL.blocking_write();
    blocking_task[index] = bool;
    drop(blocking_task);
}

fn makelevel(selfx: &mut Buttons) -> Element<Message>{
    let blocking_task = SETTINGS_USIZE.blocking_read();
    let blocking_task_two = TIME.blocking_read();
    let blocking_letters = LETTERS.blocking_read();
    let blocking_english = ENGLISH.blocking_read();

    let timer = h4(block_on(get_time()));
    
    if blocking_task_two.elapsed().as_secs() >= blocking_task[1] as u64 {
        sumbitfn()
    }


    let english = h2(format!("{}",blocking_english[N.load(Ordering::SeqCst) as usize] ))
    .height(Length::Units(150));
    
    let colours = vec![Color::BLACK,Color::from_rgb(1.0, 0.0, 0.0),Color::from_rgb(0.0, 1.0, 0.0)];
    let text1 = h2(format!("{}", blocking_letters.concat()))
    .height(Length::Units(150))
    .color(colours[COLOUR.load(Ordering::SeqCst) as usize]);
    let buttonnames = ["gotomain","shift","submit", "space", "delete","next"];
    let [
        state0, 
        state1, 
        state2, 
        state3, 
        state4, 
        state5
        ] = selfx.button_states.get_many_mut(buttonnames).unwrap();
    let mut buttonone: Vec<Button<Message>> = Vec::new();
    let mut x = 0;
    //let list: [&mut State; 26];

    let list = selfx.lang_one_states.get_many_mut(LANGONE).unwrap();
    
    for i in list{
        buttonone.push(block_on(add_button(i, shiftfn(LANGONE[x].to_string()),Message::LetterPressed(LANGONE[x].to_string()))));
        x+=1;
    } 
    x=0;
    let mut buttontwo: Vec<Button<Message>> = Vec::new();
    let list = selfx.lang_two_states.get_many_mut(LANGTWO).unwrap();
    for i in list{
        buttontwo.push(block_on(add_button(i, shiftfn(LANGTWO[x].to_string()),Message::LetterPressed(LANGTWO[x].to_string()))));
        x+=1;
    }
    x=0;
    let mut buttons: Vec<Button<Message>> = Vec::new();
    let list = selfx.punctuation_states.get_many_mut(PUNCTUATION).unwrap();
    for i in list{
        buttons.push(block_on(add_button(i, PUNCTUATION[x].to_string(),Message::LetterPressed(PUNCTUATION[x].to_string()))));
        x+=1;
    }

    let exit = add_button(state0, String::from("Exit Testing"), Message::ButtonPressed("Exit".to_string()));
    let shift = add_button(state1, String::from("Shift"), Message::ButtonPressed("Shift".to_string()));
    let submit = add_button(state2, String::from("submit"), Message::ButtonPressed("Submit".to_string()));
    let space = add_button(state3, String::from("space"), Message::ButtonPressed("Space".to_string()));
    let delete = add_button(state4, String::from("delete"), Message::ButtonPressed("Delete".to_string()));
    let next = add_button(state5, String::from("next"), Message::ButtonPressed("Next".to_string()));

    let mut userrow = Row::new();
    userrow = userrow
    .push(block_on(submit))
    .push(block_on(space))
    .push(block_on(delete))
    .push(block_on(next))
    .push(block_on(shift));


    let mut row1 = Row::new();
    for button in buttonone {
        row1 = row1.push(button);
    };
    let mut row2 = Row::new();
    for button in buttontwo {
        row2 = row2.push(button);
    };
    let mut row3 = Row::new();
    for button in buttons {
        row3 = row3.push(button);
    };
    let utilrow = Row::new().push(timer).push(block_on(exit));
    let column1 = Column::new().push(utilrow.width(Length::Fill)).push(english).push(text1).push(userrow).push(row1).push(row2).push(row3).width(Length::Fill).align_items(iced::Alignment::Center);
    let testing: Element<Message> = Container::new(column1)
        .padding(100)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into();
    drop(blocking_task);
    drop(blocking_task_two);
    drop(blocking_english);
    drop(blocking_letters);
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
            Message::IndexSent(usize) => {
                index(usize)
            }
            Message::LangChange(usize) => {
                changelang(usize)
            }
            Message::ButtonPressed(string) => {
                match string.as_str() {
                    "Save" => savefn(),
                    "Exit" => shiftscreenfn(0),
                    "Settings" => shiftscreenfn(4),
                    "Languages" => shiftscreenfn(3),
                    "Data" => shiftscreenfn(5),
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
                match event {
                    Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Space, modifiers: _ }) => {
                        pushfn(String::from(" "));
                    },
                    Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::LShift, modifiers: _ }) => {
                        shiftvaluefn();
                    },
                    Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Right, modifiers: _}) => {
                        nextfn();
                    },
                    Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Enter, modifiers: _ }) => {
                        sumbitfn();
                    },
                    Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Backspace, modifiers: _ }) => {
                        popfn();
                    }
                    _ => (),
                }
            },
            
            
            Message::SeperateCheckBox(state) => write_rwlock_bool(0, state),
            Message::SoundBox(state) => write_rwlock_bool(1, state),
            Message::TimedBox(state) => write_rwlock_bool(2, state),
            Message::VolumeSlider(num) => write_rwlock_usize(0, num),
            Message::LengthSlider(num) => write_rwlock_usize(1, num),
            Message::H1Slider(num) => write_rwlock_usize(2, num),
            Message::H2Slider(num) => write_rwlock_usize(3, num),
            Message::H3Slider(num) => write_rwlock_usize(4, num),
            Message::H4Slider(num) => write_rwlock_usize(5, num),
            Message::BodySlider(num) => write_rwlock_usize(6, num),
 
        };

        Command::none()

    }

    fn view(&mut self) -> Element<Message> {
        match SCREEN.load(Ordering::SeqCst) {
            0 => makemain(self),
            1 => makelevel(self),
            2 => makereview(self),
            3 => makelang(self),
            4 => makesettings(self),
            5 => makedata(self),
            _ => makemain(self),
        }   
    }
    
}
fn loadsettings() {
    let filename = "./settings.toml";
    let contents = fs::read_to_string(filename).unwrap();
    let mut blocking_bool = SETTINGS_BOOL.blocking_write();
    blocking_bool.clear();
    let mut blocking_task = SETTINGS_USIZE.blocking_write();
    blocking_task.clear();
    
    let data: Data = toml::from_str(&contents).unwrap();
    let boollist = 
    [
        data.settings.seperate_check_synonyms,
        data.settings.sound.sound,
        data.settings.time.timed
    ];
    for i in boollist {
        blocking_bool.push(i);
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
        blocking_task.push(i);
    }
    drop(blocking_task);
    drop(blocking_bool);

}
fn h1(text: String) -> Text {
    let blocking_task = SETTINGS_USIZE.blocking_read();
    return Text::new(text).size(blocking_task[2] as u16);
    
}
fn h2(text: String) -> Text {
    let blocking_task = SETTINGS_USIZE.blocking_read();
    return Text::new(text).size(blocking_task[3] as u16);
}
fn h3(text: String) -> Text {
    let blocking_task = SETTINGS_USIZE.blocking_read();
    return Text::new(text).size(blocking_task[4] as u16);
}
fn h4(text: String) -> Text {
    let blocking_task = SETTINGS_USIZE.blocking_read();
    return Text::new(text).size(blocking_task[5] as u16);
}
fn body(text: String) -> Text {
    let blocking_task = SETTINGS_USIZE.blocking_read();
    return Text::new(text).size(blocking_task[6] as u16);
}

fn main() -> iced::Result {
    loadsettings();
    loadsizes();
    let rgba = vec![0, 0, 0, 255];
    loaddata();
    let blocking_english = ENGLISH.blocking_read();
    N.store(thread_rng().gen_range(0..blocking_english.len()).try_into().unwrap(), Ordering::SeqCst);
    drop(blocking_english);
    let mut blocking_task = TIME.blocking_write();
    *blocking_task = Instant::now();
    drop(blocking_task);
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
        default_font: Some(include_bytes!("../resources/Arial Unicode MS Font.ttf")),
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

