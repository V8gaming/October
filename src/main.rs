#![feature(map_many_mut)]
use std::{
    fs, io::Write, 
    time::Instant,
    collections::HashMap,
    convert::TryInto,
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
use rand::{thread_rng, Rng};
use iced_native::{Event, keyboard};
use sqlite::State as SqlState;
use toml::{self};
// Global variables for storing the letters to be typed, the English and Vietnamese words,
// and other program state such as the current screen, language, and time.

mod settings;

use settings::Data;


struct Buttons {
    settings_usize: Vec<usize>,
    settings_bool: Vec<bool>,
    letters: Vec<String>,
    english: Vec<String>,
    vietnamese: Vec<String>,
    langone: [&'static str; 26],
    langtwo: [&'static str; 33],
    punctuation: [&'static str; 8],
    time: Instant,
    screen: u32,
    colour: u32,
    maxstates: u32,
    table: usize,
    lang: usize,
    shift: bool,
    word_index: usize,
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
            letters: {
                let vec: Vec<String> = Vec::new();
                vec
            },
            english: {
                let vec: Vec<String> = Vec::new();
                vec
            },
            vietnamese: {
                let vec: Vec<String> = Vec::new();
                vec
            },
            settings_bool: {
                let vec: Vec<bool> = Vec::new();
                vec
            },
            settings_usize: {
                let vec: Vec<usize> = Vec::new();
                vec
            },
            time: {
                Instant::now()
            },
            langone: {
                ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"]
            },
            langtwo: {
                [
                    "ẳ","á","â","à","ạ","ầ","ậ", "ấ","ả","ặ",
                    "đ",
                    "ỏ","ơ","ờ","ồ","ó","ô","ọ","ộ","ớ","ở",
                    "ư","ụ","ữ","ú", "ủ",
                    "í","ì","ị",
                    "ế","ẹ", "ể", "ề"
                ]
            },
            punctuation: {
                ["(",")", ";", ":", ",", ".", "?", "!"]
            },
            shift: {
                false
            },
            colour: {
                0
            },
            maxstates: {
                0
            },
            word_index: {
                0
            },
            screen: {
                0
            },
            table: {
                0
            },
            lang: {
                0
            },
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
                let map: HashMap<&str, button::State> = HashMap::new();
                map
            },
            lang_two_states: {
                let map: HashMap<&str, button::State> = HashMap::new();
                map
            },
            punctuation_states: {
                let map: HashMap<&str, button::State> = HashMap::new();
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
                let map: HashMap<String, button::State> = HashMap::new();
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
fn loadhashmaps(selfx: &mut Buttons) {

    // Add characters from LANGONE to the map
    for i in selfx.langone {
        selfx.lang_one_states.insert(i, button::State::default());
    }
    
    // Add characters from LANGTWO to the map
    for i in selfx.langtwo {
        selfx.lang_one_states.insert(i, button::State::default());
        selfx.lang_two_states.insert(i, button::State::default());
    }
    
    // Add characters from PUNCTUATION to the map
    for i in selfx.punctuation {
        selfx.lang_one_states.insert(i, button::State::default());
        selfx.punctuation_states.insert(i, button::State::default());
    }
    let mut vec = Vec::new();
    for i in 0..selfx.maxstates {
        vec.push(format!("state{}",i));
    }
    for i in vec {
        selfx.table_states.insert(i, button::State::default());
    }
}

fn pushfn(selfx: &mut Buttons, letter: String) {
    selfx.letters.push(shiftfn(selfx.shift, letter.to_string()));
    //println!("ADDED {} TO {}", letter,LETTERS.lock_mut().unwrap().concat());
    
    selfx.colour = 0;
}

fn shiftfn(shift: bool, letter: String) -> String {
    if shift == true {
        return letter.to_uppercase();
    } else {
        return letter.to_lowercase();
    }
}

fn shiftvaluefn(selfx: &mut Buttons) {
    if selfx.shift == true {
        selfx.shift = false ;
    } else {
        selfx.shift = true;
    }
}

fn shiftscreenfn(selfx: &mut Buttons,destination: usize) {
    selfx.time = Instant::now();
    selfx.screen = destination.try_into().unwrap();
    selfx.word_index = thread_rng().gen_range(0..selfx.english.len().try_into().unwrap());
    selfx.letters.clear();
    selfx.colour = 0;
}

fn sumbitfn(selfx: &mut Buttons) {
    //println!("{:?}",LETTERS.lock_mut().unwrap().concat());
    if format!("{}", selfx.letters.concat()) == selfx.vietnamese[selfx.word_index]{
        //println!("true");
        selfx.colour = 2;
        selfx.screen = 2;
    } else {
        //println!("false");
        selfx.colour = 1;
        selfx.screen = 2;
    }
}

fn popfn(selfx: &mut Buttons) {
    if selfx.letters.len() != 0 {
        selfx.letters.pop();
        //println!("{}",LETTERS.lock_mut().unwrap().concat());
        selfx.colour = 0;
    } 
    
}

fn nextfn(selfx: &mut Buttons) {
    selfx.time = Instant::now();
    //println!("{}", ENGLISH.lock_mut().unwrap().len());
    selfx.word_index = thread_rng().gen_range(0..selfx.english.len().try_into().unwrap());
    selfx.letters.clear();
    selfx.colour = 0;

}

async fn add_button<'a>(state: &'a mut button::State,content: String,message: Message, usize: usize) -> Button<'a, Message> {
    return Button::new(state, body(usize,content)).on_press(message);
}


fn index(selfx: &mut Buttons, num: usize) {
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
            "{}", languages[selfx.lang]
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
        selfx.screen = 1;
        selfx.table = num;
        loaddata(selfx);
        nextfn(selfx);
    }
}

fn tablenames(lang: usize) -> Vec<String> {
    let mut languages: Vec<String> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        //println!("{}", file.unwrap().path().display());
        languages.push(file.unwrap().path().display().to_string())
    }

    let connection = sqlite::open(format!("{}", languages[lang])).unwrap();
    let mut statement2 = connection
    .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
    .unwrap();

    while let Ok(SqlState::Row) = statement2.next() {
        //println!("{}", statement2.read::<String>(0).unwrap());
        names.push(statement2.read::<String>(0).unwrap())
    }
    return names;
    

}
fn loadsizes(selfx: &mut Buttons) {
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

    selfx.maxstates = *templangvec.iter().max().unwrap();
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


fn loaddata(selfx: &mut Buttons) {
    let mut languages: Vec<String> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        //println!("{}", file.unwrap().path().display());
        languages.push(file.unwrap().path().display().to_string())
    }
    let connection = sqlite::open(format!("{}", languages[selfx.lang])).unwrap();
    
    let mut statement2 = connection
    .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
    .unwrap();
    let mut tables: Vec<String> = Vec::new();

    while let Ok(SqlState::Row) = statement2.next() {
        //println!("{}", statement2.read::<String>(0).unwrap());
        tables.push(statement2.read::<String>(0).unwrap())
    }
    let mut statement = connection
        .prepare(format!("SELECT * FROM {}", tables[selfx.table]))
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

    selfx.english.clear();
    for i in english {
        //println!("{}",i);

        selfx.english.push(i);
        
    }
    for i in vietnamese {
        selfx.vietnamese.push(i)
    }
}
fn changelang(selfx: &mut Buttons, num: usize) {
    selfx.lang = num;
    selfx.table = 0;
    shiftscreenfn(selfx, 0);
    loaddata(selfx);
}

fn makelang<'a>(selfx: &'a mut Buttons) -> Element<'a, Message>{
    let mut x = 0;
    let mut buttons: Vec<Button<Message>>=Vec::new();
    let names = loadnames();
    for i in selfx.language_states.values_mut() {
        buttons.push(block_on(add_button(i, names[x].to_string(), Message::LangChange(x), selfx.settings_usize[6])));
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

fn savefn(selfx: &mut Buttons) {
    let mut file = std::fs::File::create("./settings.toml").expect("create failed");
    file.write_all(format!("[settings]\n").as_bytes()).expect("write failed");
    file.write_all(format!("seperate_check_synonyms = {} # 0\n", selfx.settings_bool[0]).as_bytes()).expect("write failed");
    file.write_all(format!("\n").as_bytes()).expect("write failed");
    file.write_all(format!("[settings.sound]\n").as_bytes()).expect("write failed");
    file.write_all(format!("sound =  {} # 1\n", selfx.settings_bool[1]).as_bytes()).expect("write failed");
    file.write_all(format!("volume = {} # 0\n", selfx.settings_usize[0]).as_bytes()).expect("write failed");
    file.write_all(format!("\n").as_bytes()).expect("write failed");

    file.write_all(format!("[settings.time]\n").as_bytes()).expect("write failed");
    file.write_all(format!("timed =  {} # 2\n", selfx.settings_bool[2]).as_bytes()).expect("write failed");
    file.write_all(format!("length = {} # 1\n", selfx.settings_usize[1]).as_bytes()).expect("write failed");
    file.write_all(format!("\n").as_bytes()).expect("write failed");

    file.write_all(format!("[settings.textsize]\n").as_bytes()).expect("write failed");
    file.write_all(format!("h1 = {} # 2\n", selfx.settings_usize[2]).as_bytes()).expect("write failed");
    file.write_all(format!("h2 = {} # 3\n", selfx.settings_usize[3]).as_bytes()).expect("write failed");
    file.write_all(format!("h3 = {} # 4\n", selfx.settings_usize[4]).as_bytes()).expect("write failed");
    file.write_all(format!("h4 = {} # 5\n", selfx.settings_usize[5]).as_bytes()).expect("write failed");
    file.write_all(format!("body = {} # 6\n", selfx.settings_usize[6]).as_bytes()).expect("write failed");
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
    let h2_general= h2(selfx.settings_usize[3],String::from("General Settings"));
    let seperatecheckbox = Checkbox::new(selfx.settings_bool[0], "Seperately check synonyms", Message::SeperateCheckBox);
    let h2_sound = h2(selfx.settings_usize[3],String::from("Sound Settings"));
    let soundbox = Checkbox::new(selfx.settings_bool[1], "Sound Enabled", Message::SoundBox);
    let volumeslider = makeslider!(&mut selfx.settings.volume, 100, selfx.settings_usize[0], Message::VolumeSlider);
    let h2_time = h2(selfx.settings_usize[3],String::from("Time Settings"));
    let lengthbox = Checkbox::new(selfx.settings_bool[2], "Timed", Message::TimedBox);
    let lengthslider = makeslider!(&mut selfx.settings.length, 1, 30, selfx.settings_usize[1], Message::LengthSlider);
    let h2_text= h2(selfx.settings_usize[3],String::from("Text Settings"));
    let h1slider = makeslider!(&mut selfx.settings.h1, 200, selfx.settings_usize[2], Message::H1Slider);
    let h2slider = makeslider!(&mut selfx.settings.h2, 200, selfx.settings_usize[3], Message::H2Slider);
    let h3slider = makeslider!(&mut selfx.settings.h3, 200, selfx.settings_usize[4], Message::H3Slider);
    let h4slider = makeslider!(&mut selfx.settings.h4, 200, selfx.settings_usize[5], Message::H4Slider);
    let bodyslider = makeslider!(&mut selfx.settings.body, 200, selfx.settings_usize[6], Message::BodySlider);
    let volume = h4(selfx.settings_usize[5],String::from(format!("Volume: {}", selfx.settings_usize[0])));
    let length = h4(selfx.settings_usize[5],String::from(format!("Time: {}", selfx.settings_usize[1])));
    let h1 = h1(selfx.settings_usize[2],String::from(format!("H1: {}", selfx.settings_usize[2])));
    let h2 = h2(selfx.settings_usize[3],String::from(format!("H2: {}", selfx.settings_usize[3])));
    let h3 = h3(selfx.settings_usize[4],String::from(format!("H3: {}", selfx.settings_usize[4])));
    let h4 = h4(selfx.settings_usize[5],String::from(format!("H4: {}", selfx.settings_usize[5])));
    let body = body(selfx.settings_usize[6],String::from(format!("Body: {}", selfx.settings_usize[6])));
    let reset = add_button(&mut selfx.settings.load_state, String::from("Reload Default"), Message::LoadButton, selfx.settings_usize[6]);
    let [state0, state1] = selfx.button_states.get_many_mut(["save", "gotomain"]).unwrap();

    let save = add_button(state0, String::from("Save"), Message::ButtonPressed("Save".to_string()), selfx.settings_usize[6]); 
    let exit = add_button(state1, String::from("Exit"), Message::ButtonPressed("Exit".to_string()), selfx.settings_usize[6]);
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
    return main;
}
fn loadsize(lang: usize) -> usize {
    let mut languages: Vec<String> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        //println!("{}", file.unwrap().path().display());
        languages.push(file.unwrap().path().display().to_string())
    }
    let connection = sqlite::open(format!("{}", languages[lang])).unwrap();
    
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
    loadsettings(selfx);
    loadsizes(selfx);
    loadhashmaps(selfx);
    loaddata(selfx);
    selfx.colour = thread_rng().gen_range(0..selfx.english.len().try_into().unwrap());
    let title = h1(selfx.settings_usize[2],String::from("October"));
    let [state0, state1, state2] = selfx.button_states.get_many_mut(["gotosetting", "gotolang", "gotodata"]).unwrap();

    let settings = add_button(state0, String::from("Settings"), Message::ButtonPressed("Settings".to_string()), selfx.settings_usize[6]);
    let langs = add_button(state1, String::from("Languages"), Message::ButtonPressed("Languages".to_string()), selfx.settings_usize[6]);
    let data = add_button(state2, String::from("Data"), Message::ButtonPressed("Data".to_string()), selfx.settings_usize[6]);

    let mut x = 0;

    let mut maincolumn = Column::new().push(block_on(settings)).push(title).push(block_on(langs)).push(block_on(data));

    let mut buttons: Vec<Button<Message>>=Vec::new();
    let names = tablenames(selfx.lang);
    for i in selfx.table_states.values_mut() {
        if x >= loadsize(selfx.lang) {
        } else {
            buttons.push(block_on(add_button(i, names[x].to_string(), Message::IndexSent(x), selfx.settings_usize[6])));
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

    let n = Text::new(format!("Word_Index: {:?}", selfx.word_index));
    let table = Text::new(format!("table: {:?}", selfx.table));
    let lang = Text::new(format!("Lang: {:?}", selfx.lang));
    let english = h4(selfx.settings_usize[5],format!("{}",selfx.english[selfx.word_index] ))
        .height(Length::Units(60));

    let vietnamese = h4(selfx.settings_usize[5],format!("{}",selfx.vietnamese[selfx.word_index] ))
        .height(Length::Units(60));
    let maincolumn = Column::new().push(n).push(table).push(lang).push(english).push(vietnamese);
    let exit = add_button(state, "exit".to_string(), Message::ButtonPressed("Exit".to_string()), selfx.settings_usize[6]);
    let main: Element<Message> = Container::new(maincolumn.push(block_on(exit)))
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

    let exit = add_button(state0, String::from("Exit"), Message::ButtonPressed("Exit".to_string()), selfx.settings_usize[6]);
    let colours = vec![Color::BLACK,Color::from_rgb(1.0, 0.0, 0.0),Color::from_rgb(0.0, 1.0, 0.0)];

    let subtitle1 = h3(selfx.settings_usize[4],String::from("Your Response"))
        .color(colours[selfx.colour as usize])
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(Length::Fill);

    let subtitle2 = h3(selfx.settings_usize[4],String::from("Vietnamese"))
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(Length::Fill);

    let subtitle3 = h3(selfx.settings_usize[4],String::from("English"))
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(Length::Fill);

    let response = h4(selfx.settings_usize[5],format!("{}", selfx.letters.concat()))
        .height(Length::Units(60))
        .color(colours[selfx.colour as usize]);
    let english = h4(selfx.settings_usize[5],format!("{}",selfx.english[selfx.word_index] ))
        .height(Length::Units(60));

    let vietnamese = h4(selfx.settings_usize[5],format!("{}",selfx.vietnamese[selfx.word_index] ))
        .height(Length::Units(60));

    let resume = add_button(state1, String::from("Resume"), Message::ButtonPressed("Resume".to_string()), selfx.settings_usize[6]);
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

    return review;
}

async fn get_time(selfx: &mut Buttons) -> String {

    // Borrow values from RwLock instead of cloning them
    let settings = selfx.settings_usize[1];
    let time = selfx.time;

    // Cache elapsed time value in a local variable
    let elapsed = time.elapsed().as_secs_f32();

    // Calculate and return result
    let new_time = settings as f32 - elapsed;
    format!("{:.2}", new_time)
}

fn makelevel(selfx: &mut Buttons) -> Element<Message>{
    let timer = h4(selfx.settings_usize[5],block_on(get_time(selfx)));
    
    if selfx.time.elapsed().as_secs() >= selfx.settings_usize[1] as u64 {
        sumbitfn(selfx)
    }


    let english = h2(selfx.settings_usize[3],format!("{}",selfx.english[selfx.word_index] ))
    .height(Length::Units(150));
    
    let colours = vec![Color::BLACK,Color::from_rgb(1.0, 0.0, 0.0),Color::from_rgb(0.0, 1.0, 0.0)];
    let text1 = h2(selfx.settings_usize[3],format!("{}", selfx.letters.concat()))
    .height(Length::Units(150))
    .color(colours[selfx.colour as usize]);
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

    let list = selfx.lang_one_states.get_many_mut(selfx.langone).unwrap();
    
    for i in list{
        buttonone.push(block_on(add_button(i, shiftfn(selfx.shift, selfx.langone[x].to_string()),Message::LetterPressed(selfx.langone[x].to_string()), selfx.settings_usize[6])));
        x+=1;
    } 
    x=0;
    let mut buttontwo: Vec<Button<Message>> = Vec::new();
    let list = selfx.lang_two_states.get_many_mut(selfx.langtwo).unwrap();
    for i in list{
        buttontwo.push(block_on(add_button(i, shiftfn(selfx.shift,selfx.langtwo[x].to_string()),Message::LetterPressed(selfx.langtwo[x].to_string()), selfx.settings_usize[6])));
        x+=1;
    }
    x=0;
    let mut buttons: Vec<Button<Message>> = Vec::new();
    let list = selfx.punctuation_states.get_many_mut(selfx.punctuation).unwrap();
    for i in list{
        buttons.push(block_on(add_button(i, selfx.punctuation[x].to_string(),Message::LetterPressed(selfx.punctuation[x].to_string()), selfx.settings_usize[6])));
        x+=1;
    }

    let exit = add_button(state0, String::from("Exit Testing"), Message::ButtonPressed("Exit".to_string()), selfx.settings_usize[6]);
    let shift = add_button(state1, String::from("Shift"), Message::ButtonPressed("Shift".to_string()), selfx.settings_usize[6]);
    let submit = add_button(state2, String::from("submit"), Message::ButtonPressed("Submit".to_string()), selfx.settings_usize[6]);
    let space = add_button(state3, String::from("space"), Message::ButtonPressed("Space".to_string()), selfx.settings_usize[6]);
    let delete = add_button(state4, String::from("delete"), Message::ButtonPressed("Delete".to_string()), selfx.settings_usize[6]);
    let next = add_button(state5, String::from("next"), Message::ButtonPressed("Next".to_string()), selfx.settings_usize[6]);

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
        return (Buttons::default(), Command::none());
    }

    fn title(&self) -> String {
        String::from("October")
    }
    
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LoadButton => loadsettings(self),
            Message::IndexSent(usize) => {
                index(self, usize)
            }
            Message::LangChange(usize) => {
                changelang(self,usize)
            }
            Message::ButtonPressed(string) => {
                match string.as_str() {
                    "Save" => savefn(self),
                    "Exit" => shiftscreenfn(self, 0),
                    "Settings" => shiftscreenfn(self,4),
                    "Languages" => shiftscreenfn(self,3),
                    "Data" => shiftscreenfn(self,5),
                    "Resume" => shiftscreenfn(self,1),
                    "Shift" => shiftvaluefn(self),
                    "Submit" => sumbitfn(self),
                    "Space" => pushfn(self,String::from(" ")),
                    "Delete" => popfn(self),
                    "Next" => nextfn(self),
                    _ => (),
                }
            }
            Message::LetterPressed(string) => pushfn(self, string),
            Message::EventOccurred(event) => {
                match event {
                    Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Space, modifiers: _ }) => {
                        pushfn(self, String::from(" "));
                    },
                    Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::LShift, modifiers: _ }) => {
                        shiftvaluefn(self);
                    },
                    Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Right, modifiers: _}) => {
                        nextfn(self);
                    },
                    Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Enter, modifiers: _ }) => {
                        sumbitfn(self);
                    },
                    Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Backspace, modifiers: _ }) => {
                        popfn(self);
                    }
                    _ => (),
                }
            },
            
            Message::SeperateCheckBox(state) => self.settings_bool[0] = state,
            Message::SoundBox(state) => self.settings_bool[1] = state,
            Message::TimedBox(state) => self.settings_bool[2] = state,
            Message::VolumeSlider(num) => self.settings_usize[0] = num as usize,
            Message::LengthSlider(num) => self.settings_usize[1] = num as usize,
            Message::H1Slider(num) => self.settings_usize[2] = num as usize,
            Message::H2Slider(num) => self.settings_usize[3] = num as usize,
            Message::H3Slider(num) => self.settings_usize[4] = num as usize,
            Message::H4Slider(num) => self.settings_usize[5] = num as usize,
            Message::BodySlider(num) => self.settings_usize[6] = num as usize,
 
        };

        Command::none()

    }

    fn view(&mut self) -> Element<Message> {
        match self.screen {
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
fn loadsettings(selfx: &mut Buttons) {
    let filename = "./settings.toml";
    let contents = fs::read_to_string(filename).unwrap();
    selfx.settings_bool.clear();
    selfx.settings_usize.clear();
    
    let data: Data = toml::from_str(&contents).unwrap();
    let boollist = 
    [
        data.settings.seperate_check_synonyms,
        data.settings.sound.sound,
        data.settings.time.timed
    ];
    for i in boollist {
        selfx.settings_bool.push(i);
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
        selfx.settings_usize.push(i);
    }
}

fn h1(usize: usize, text: String) -> Text {
    return Text::new(text).size(usize as u16);
}
fn h2(usize: usize, text: String) -> Text {
    return Text::new(text).size(usize as u16);
}
fn h3(usize: usize, text: String) -> Text {
    return Text::new(text).size(usize as u16);
}
fn h4(usize: usize, text: String) -> Text {
    return Text::new(text).size(usize as u16);
}
fn body(usize: usize, text: String) -> Text {
    return Text::new(text).size(usize as u16);
}

fn main() -> iced::Result {
    let rgba = vec![0, 0, 0, 255];
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