#![windows_subsystem = "windows"]
use iced::{alignment,Application,Command, Subscription, executor, button, Button, Element, Settings, Text, Container, Length, Column, Row, window, Color};
use iced::window::Position::Specific;
use iced::window::Icon;
use global::Global;
use rand::{thread_rng, Rng};
use sqlite::State;
use iced_native::{Event, keyboard};
use std::fs;
use serde_derive::Deserialize;
use toml;
static LETTERS: Global<Vec<String>> = Global::new();
static ENGLISH: Global<Vec<String>> = Global::new();
static VIETNAMESE: Global<Vec<String>> = Global::new();
static N: Global<Vec<usize>> = Global::new();
static COLOUR: Global<Vec<usize>> = Global::new();
static X: Global<Vec<usize>> = Global::new();
static TABLE: Global<Vec<usize>> = Global::new();
static TEXTTYPE: Global<Vec<String>> = Global::new();
static SCREEN: Global<Vec<usize>> = Global::new();
static LANG: Global<Vec<usize>> = Global::new();
static SETTINGS_USIZE: Global<Vec<usize>> = Global::new();
static SETTINGS_BOOL: Global<Vec<bool>> = Global::new();
#[derive(Deserialize, Debug)]
struct Data {
    settings: ImportSettings
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
struct ImportSettings {
    seperate_check_synonyms: bool,
    sound: SoundData,
    textsize: TextsizeData,
    time: TimeData,
}

#[derive(Default, Clone)]
struct MyButton {
    gotomain_state: button::State,
    gotolang_state: button::State,
    resume_state: button::State,
    table0_state: button::State,
    table1_state: button::State,
    table2_state: button::State,
    shift_state: button::State,
    submit_state: button::State,
    space_state: button::State,
    delete_state: button::State,
    deleteall_state: button::State,
    next_state: button::State,
    button_state0: button::State,
    button_state1: button::State,
    button_state2: button::State,
    button_state3: button::State,
    button_state4: button::State,
    button_state5: button::State,
    button_state6: button::State,
    button_state7: button::State,
    button_state8: button::State,
    button_state9: button::State,
    button_state10: button::State,
    button_state11: button::State,
    button_state12: button::State,
    button_state13: button::State,
    button_state14: button::State,
    button_state15: button::State,
    button_state16: button::State,
    button_state17: button::State,
    button_state18: button::State,
    button_state19: button::State,
    button_state20: button::State,
    button_state21: button::State,
    button_state22: button::State,
    button_state23: button::State,
    button_state24: button::State,
    button_state25: button::State,
    button_state26: button::State,
    button_state27: button::State,
    button_state28: button::State,
    button_state29: button::State,
    button_state30: button::State,
    button_state31: button::State,
    button_state32: button::State,
    button_state33: button::State,
    button_state34: button::State,
    button_state35: button::State,
    button_state36: button::State,
    button_state37: button::State,
    button_state38: button::State,
    button_state39: button::State,
    button_state40: button::State,
    button_state41: button::State,
    button_state42: button::State,
    button_state43: button::State,
    button_state44: button::State,
    button_state45: button::State,
    button_state46: button::State,
    button_state47: button::State,
    button_state48: button::State,
    button_state49: button::State,
    button_state50: button::State,
    button_state51: button::State,
    button_state52: button::State,
    button_state53: button::State,
    button_state54: button::State,
    button_state55: button::State,
    button_state56: button::State,
    button_state57: button::State,
    button_state58: button::State,
    button_state59: button::State,
    button_state60: button::State,
    button_state61: button::State,
    button_state62: button::State,
    button_state63: button::State,
    button_state64: button::State,
    button_state65: button::State,
    button_state66: button::State,
    lang_state0: button::State,
    lang_state1: button::State,
}

#[derive(Debug, Clone)]enum Message {
    EventOccurred(iced_native::Event),
    GotoMainButton,
    GotoLangButton,
    ResumeButton,
    TableButton0,
    TableButton1,
    TableButton2,
    ShiftButton,
    SubmitButton,
    SpaceButton,
    DeleteButton,
    DeleteallButton,
    NextButton,
    ButtonPressed0,
    ButtonPressed1,
    ButtonPressed2,
    ButtonPressed3,
    ButtonPressed4,
    ButtonPressed5,
    ButtonPressed6,
    ButtonPressed7,
    ButtonPressed8,
    ButtonPressed9,
    ButtonPressed10,
    ButtonPressed11,
    ButtonPressed12,
    ButtonPressed13,
    ButtonPressed14,
    ButtonPressed15,
    ButtonPressed16,
    ButtonPressed17,
    ButtonPressed18,
    ButtonPressed19,
    ButtonPressed20,
    ButtonPressed21,
    ButtonPressed22,
    ButtonPressed23,
    ButtonPressed24,
    ButtonPressed25,
    ButtonPressed26,
    ButtonPressed27,
    ButtonPressed28,
    ButtonPressed29,
    ButtonPressed30,
    ButtonPressed31,
    ButtonPressed32,
    ButtonPressed33,
    ButtonPressed34,
    ButtonPressed35,
    ButtonPressed36,
    ButtonPressed37,
    ButtonPressed38,
    ButtonPressed39,
    ButtonPressed40,
    ButtonPressed41,
    ButtonPressed42,
    ButtonPressed43,
    ButtonPressed44,
    ButtonPressed45,
    ButtonPressed46,
    ButtonPressed47,
    ButtonPressed48,
    ButtonPressed49,
    ButtonPressed50,
    ButtonPressed51,
    ButtonPressed52,
    ButtonPressed53,
    ButtonPressed54,
    ButtonPressed55,
    ButtonPressed56,
    ButtonPressed57,
    ButtonPressed58,
    ButtonPressed59,
    ButtonPressed60,
    ButtonPressed61,
    ButtonPressed62,
    ButtonPressed63,
    ButtonPressed64,
    ButtonPressed65,
    ButtonPressed66,
    LangButton0,
    LangButton1,
}
fn pushfn(letter:String) {
    LETTERS.lock_mut().unwrap().push(shiftfn(letter.to_string()));
    println!("ADDED {} TO {}", letter,LETTERS.lock_mut().unwrap().concat());
        COLOUR.lock_mut().unwrap()[0] = 0
}
fn shiftscreenfn(destination: usize) {
   SCREEN.lock_mut().unwrap()[0] = destination;
   N.lock_mut().unwrap()[0] = thread_rng().gen_range(0..ENGLISH.lock_mut().unwrap().len());
   LETTERS.lock_mut().unwrap().clear();
   COLOUR.lock_mut().unwrap()[0] = 0;
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
fn sumbitfn() {
    if format!("{}", LETTERS.lock_mut().unwrap().concat()) == VIETNAMESE.lock_mut().unwrap()[N.lock_mut().unwrap()[0]]{
        COLOUR.lock_mut().unwrap()[0] = 2;
        SCREEN.lock_mut().unwrap()[0] = 2;
    } else {
        COLOUR.lock_mut().unwrap()[0] = 1;
        SCREEN.lock_mut().unwrap()[0] = 2;
    }
}
fn popfn() {
    if LETTERS.lock_mut().unwrap().len() != 0 {
        LETTERS.lock_mut().unwrap().pop();
        println!("{}",LETTERS.lock_mut().unwrap().concat());
        COLOUR.lock_mut().unwrap()[0] = 0
    }
}
fn clearfn() {
    if LETTERS.lock_mut().unwrap().len() != 0 {
        LETTERS.lock_mut().unwrap().clear();
        println!("{}",LETTERS.lock_mut().unwrap().concat());
        COLOUR.lock_mut().unwrap()[0] = 0
    }
}
fn add_button<'a>(a: &'a mut button::State,b: String,c: Message) -> Button<'a, Message> {
    return Button::new(a, body(format!("{}",b))).on_press(c);
}
fn loadtables<'a>(self0: &'a mut button::State,self1: &'a mut button::State,self2: &'a mut button::State,) -> Vec<Button<'a, Message>> {
    if LANG.lock_mut().unwrap()[0] == 0 {
        return vec![
            add_button(self0, String::from("Basic"), Message::TableButton0),
            add_button(self1, String::from("Intermediate"), Message::TableButton1),
            add_button(self2, String::from("Advanced"), Message::TableButton2),
            ];
    } else if LANG.lock_mut().unwrap()[0] == 1 {
        return vec![
            add_button(self0, String::from("Basic"), Message::TableButton0),
            add_button(self1, String::from("Intermediate"), Message::TableButton1),
            ];
    } else  {
        return vec![
            add_button(self0, String::from("Level 0"), Message::TableButton0),
            add_button(self1, String::from("Level 1"), Message::TableButton1),
            ];
    }
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
fn makemain(selfx: &mut MyButton) -> Element<Message>{
    let mut languages: Vec<String> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        languages.push(file.unwrap().path().display().to_string())
    }
    let title = h1(String::from("October")).height(Length::FillPortion(1)).horizontal_alignment(alignment::Horizontal::Center).width(Length::Fill);
    let curlang = h3(format!("Language: {}", languages[LANG.lock_mut().unwrap()[0]].strip_suffix(".sqlite3").unwrap().strip_prefix("./resources/languages/").unwrap())).height(Length::FillPortion(1));
    let langs = add_button(&mut selfx.gotolang_state, String::from("Languages"), Message::GotoLangButton);
    let buttons = loadtables(&mut selfx.table0_state,&mut selfx.table1_state,&mut selfx.table2_state,);
    let mut maincolumn = Column::new().push(title).push(curlang).push(langs).align_items(alignment::Alignment::Center).width(Length::Fill);
    for i in buttons  {
        maincolumn = maincolumn.push(i);
    };
    let main: Element<Message> = Container::new(maincolumn)
        .padding(100)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into();
    return main;
}
fn makelang(selfx: &mut MyButton) -> Element<Message>{
    TABLE.lock_mut().unwrap()[0] = 0;
    let langcolumn = Column::new().push(add_button(&mut selfx.lang_state0, String::from("English-Vietnamese"), Message::LangButton0)).push(add_button(&mut selfx.lang_state1, String::from("test"), Message::LangButton1))    ;
    let main: Element<Message> = Container::new(langcolumn)
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
    let subtitle1 = h3(String::from("Your answer")).color(colours[COLOUR.lock_mut().unwrap()[0]]).horizontal_alignment(alignment::Horizontal::Center).width(Length::Fill);
    let subtitle2 = h3(String::from("Vietnamese")).horizontal_alignment(alignment::Horizontal::Center).width(Length::Fill);
    let subtitle3 = h3(String::from("English")).horizontal_alignment(alignment::Horizontal::Center).width(Length::Fill);
    let youranswer = h4(format!("{}", LETTERS.lock_mut().unwrap().concat())).height(Length::Units(80)).color(colours[COLOUR.lock_mut().unwrap()[0]]);
    let english = h4(format!("{}",ENGLISH.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Units(80));
    let vietnamese = h4(format!("{}",VIETNAMESE.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Units(80));
    let resume = add_button(&mut selfx.resume_state, String::from("Resume"), Message::ResumeButton);
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
    let english = h2(format!("{}",ENGLISH.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Units(80));
    let colours = vec![Color::BLACK,Color::from_rgb(1.0, 0.0, 0.0),Color::from_rgb(0.0, 1.0, 0.0)];
    let text1 = h2(format!("{}", LETTERS.lock_mut().unwrap().concat())).height(Length::Units(80)).color(colours[COLOUR.lock_mut().unwrap()[0]]);
    let texttype = h4(format!("{}",TEXTTYPE.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Fill).size(40);
    let buttons1 = [
        add_button(&mut selfx.button_state0, shiftfn(String::from("a")), Message::ButtonPressed0),
        add_button(&mut selfx.button_state1, shiftfn(String::from("b")), Message::ButtonPressed1),
        add_button(&mut selfx.button_state2, shiftfn(String::from("c")), Message::ButtonPressed2),
        add_button(&mut selfx.button_state3, shiftfn(String::from("d")), Message::ButtonPressed3),
        add_button(&mut selfx.button_state4, shiftfn(String::from("e")), Message::ButtonPressed4),
        add_button(&mut selfx.button_state5, shiftfn(String::from("f")), Message::ButtonPressed5),
        add_button(&mut selfx.button_state6, shiftfn(String::from("g")), Message::ButtonPressed6),
        add_button(&mut selfx.button_state7, shiftfn(String::from("h")), Message::ButtonPressed7),
        add_button(&mut selfx.button_state8, shiftfn(String::from("i")), Message::ButtonPressed8),
        add_button(&mut selfx.button_state9, shiftfn(String::from("j")), Message::ButtonPressed9),
        add_button(&mut selfx.button_state10, shiftfn(String::from("k")), Message::ButtonPressed10),
        add_button(&mut selfx.button_state11, shiftfn(String::from("l")), Message::ButtonPressed11),
        add_button(&mut selfx.button_state12, shiftfn(String::from("m")), Message::ButtonPressed12),
        add_button(&mut selfx.button_state13, shiftfn(String::from("n")), Message::ButtonPressed13),
        add_button(&mut selfx.button_state14, shiftfn(String::from("o")), Message::ButtonPressed14),
        add_button(&mut selfx.button_state15, shiftfn(String::from("p")), Message::ButtonPressed15),
        add_button(&mut selfx.button_state16, shiftfn(String::from("q")), Message::ButtonPressed16),
        add_button(&mut selfx.button_state17, shiftfn(String::from("r")), Message::ButtonPressed17),
        add_button(&mut selfx.button_state18, shiftfn(String::from("s")), Message::ButtonPressed18),
        add_button(&mut selfx.button_state19, shiftfn(String::from("t")), Message::ButtonPressed19),
        add_button(&mut selfx.button_state20, shiftfn(String::from("u")), Message::ButtonPressed20),
        add_button(&mut selfx.button_state21, shiftfn(String::from("v")), Message::ButtonPressed21),
        add_button(&mut selfx.button_state22, shiftfn(String::from("w")), Message::ButtonPressed22),
        add_button(&mut selfx.button_state23, shiftfn(String::from("x")), Message::ButtonPressed23),
        add_button(&mut selfx.button_state24, shiftfn(String::from("y")), Message::ButtonPressed24),
        add_button(&mut selfx.button_state25, shiftfn(String::from("z")), Message::ButtonPressed25),
    ];
    let buttons2 = [
        add_button(&mut selfx.button_state26, shiftfn(String::from("ẳ")), Message::ButtonPressed26),
        add_button(&mut selfx.button_state27, shiftfn(String::from("á")), Message::ButtonPressed27),
        add_button(&mut selfx.button_state28, shiftfn(String::from("â")), Message::ButtonPressed28),
        add_button(&mut selfx.button_state29, shiftfn(String::from("à")), Message::ButtonPressed29),
        add_button(&mut selfx.button_state30, shiftfn(String::from("ạ")), Message::ButtonPressed30),
        add_button(&mut selfx.button_state31, shiftfn(String::from("ầ")), Message::ButtonPressed31),
        add_button(&mut selfx.button_state32, shiftfn(String::from("ậ")), Message::ButtonPressed32),
        add_button(&mut selfx.button_state33, shiftfn(String::from("ấ")), Message::ButtonPressed33),
        add_button(&mut selfx.button_state34, shiftfn(String::from("ả")), Message::ButtonPressed34),
        add_button(&mut selfx.button_state35, shiftfn(String::from("ặ")), Message::ButtonPressed35),
        add_button(&mut selfx.button_state36, shiftfn(String::from("đ")), Message::ButtonPressed36),
        add_button(&mut selfx.button_state37, shiftfn(String::from("ỏ")), Message::ButtonPressed37),
        add_button(&mut selfx.button_state38, shiftfn(String::from("ơ")), Message::ButtonPressed38),
        add_button(&mut selfx.button_state39, shiftfn(String::from("ờ")), Message::ButtonPressed39),
        add_button(&mut selfx.button_state40, shiftfn(String::from("ồ")), Message::ButtonPressed40),
        add_button(&mut selfx.button_state41, shiftfn(String::from("ó")), Message::ButtonPressed41),
        add_button(&mut selfx.button_state42, shiftfn(String::from("ô")), Message::ButtonPressed42),
        add_button(&mut selfx.button_state43, shiftfn(String::from("ọ")), Message::ButtonPressed43),
        add_button(&mut selfx.button_state44, shiftfn(String::from("ộ")), Message::ButtonPressed44),
        add_button(&mut selfx.button_state45, shiftfn(String::from("ớ")), Message::ButtonPressed45),
        add_button(&mut selfx.button_state46, shiftfn(String::from("ở")), Message::ButtonPressed46),
        add_button(&mut selfx.button_state47, shiftfn(String::from("ư")), Message::ButtonPressed47),
        add_button(&mut selfx.button_state48, shiftfn(String::from("ụ")), Message::ButtonPressed48),
        add_button(&mut selfx.button_state49, shiftfn(String::from("ữ")), Message::ButtonPressed49),
        add_button(&mut selfx.button_state50, shiftfn(String::from("ú")), Message::ButtonPressed50),
        add_button(&mut selfx.button_state51, shiftfn(String::from("ủ")), Message::ButtonPressed51),
        add_button(&mut selfx.button_state52, shiftfn(String::from("í")), Message::ButtonPressed52),
        add_button(&mut selfx.button_state53, shiftfn(String::from("ì")), Message::ButtonPressed53),
        add_button(&mut selfx.button_state54, shiftfn(String::from("ị")), Message::ButtonPressed54),
        add_button(&mut selfx.button_state55, shiftfn(String::from("ế")), Message::ButtonPressed55),
        add_button(&mut selfx.button_state56, shiftfn(String::from("ẹ")), Message::ButtonPressed56),
        add_button(&mut selfx.button_state57, shiftfn(String::from("ể")), Message::ButtonPressed57),
        add_button(&mut selfx.button_state58, shiftfn(String::from("ề")), Message::ButtonPressed58),
    ];
    let buttons3 = [
        add_button(&mut selfx.button_state59, shiftfn(String::from("(")), Message::ButtonPressed59),
        add_button(&mut selfx.button_state60, shiftfn(String::from(")")), Message::ButtonPressed60),
        add_button(&mut selfx.button_state61, shiftfn(String::from(";")), Message::ButtonPressed61),
        add_button(&mut selfx.button_state62, shiftfn(String::from(":")), Message::ButtonPressed62),
        add_button(&mut selfx.button_state63, shiftfn(String::from(",")), Message::ButtonPressed63),
        add_button(&mut selfx.button_state64, shiftfn(String::from(".")), Message::ButtonPressed64),
        add_button(&mut selfx.button_state65, shiftfn(String::from("?")), Message::ButtonPressed65),
        add_button(&mut selfx.button_state66, shiftfn(String::from("!")), Message::ButtonPressed66),
    ];
    let shift = add_button(&mut selfx.shift_state, String::from("shift"), Message::ShiftButton);
    let submit = add_button(&mut selfx.submit_state, String::from("submit"), Message::SubmitButton);
    let space = add_button(&mut selfx.space_state, String::from("space"), Message::SpaceButton);
    let delete = add_button(&mut selfx.delete_state, String::from("delete"), Message::DeleteButton);
    let deleteall = add_button(&mut selfx.deleteall_state, String::from("deleteall"), Message::DeleteallButton);
    let next = add_button(&mut selfx.next_state, String::from("next"), Message::NextButton);
    let mut userrow = Row::new();
    userrow = userrow.push(shift);
    userrow = userrow.push(submit);
    userrow = userrow.push(space);
    userrow = userrow.push(delete);
    userrow = userrow.push(deleteall);
    userrow = userrow.push(next);
    let mut row1 = Row::new();
    for button in buttons1 {
        row1 = row1.push(button);
    };
    let mut row2 = Row::new();
    for button in buttons2 {
        row2 = row2.push(button);
    };
    let mut row3 = Row::new();
    for button in buttons3 {
        row3 = row3.push(button);
    };
    let exit = add_button(&mut selfx.gotomain_state, String::from("Exit"), Message::GotoMainButton);
    let utilrow = Row::new().push(exit);
    let column1 = Column::new().push(utilrow.width(Length::Fill)).push(texttype).push(english).push(text1).push(userrow).push(row1).push(row2).push(row3).width(Length::Fill).align_items(iced::Alignment::Center);
    Container::new(column1)
    .padding(100)
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()}
fn nextfn() {
    N.lock_mut().unwrap()[0] = thread_rng().gen_range(0..ENGLISH.lock_mut().unwrap().len());
    LETTERS.lock_mut().unwrap().clear();
    COLOUR.lock_mut().unwrap()[0] = 0
}
fn changelang(num: usize) {
    LANG.lock_mut().unwrap()[0] = num;
    shiftscreenfn(0);
    loaddata();
}
fn loaddata() {
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
    let mut statement = connection
        .prepare(format!("SELECT * FROM {}", tables[TABLE.lock_mut().unwrap()[0]]))
        .unwrap();
    let mut vietnamese: Vec<String> = Vec::new();
    let mut english: Vec<String> = Vec::new();
    let mut typename: Vec<String> = Vec::new();
    while let Ok(State::Row) = statement.next() {
        english.push(statement.read::<String>(0).unwrap());
        vietnamese.push(statement.read::<String>(1).unwrap());
        typename.push(statement.read::<String>(2).unwrap());
    }
    ENGLISH.lock_mut().unwrap().clear();
    VIETNAMESE.lock_mut().unwrap().clear();
    TEXTTYPE.lock_mut().unwrap().clear();
    for i in english {
        ENGLISH.lock_mut().unwrap().push(i);
    }
    for i in vietnamese {
        VIETNAMESE.lock_mut().unwrap().push(i);
    }
    for i in typename {
        TEXTTYPE.lock_mut().unwrap().push(i);
    }
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
        String::from("October")
   }
  fn update(&mut self, message: Message) -> Command<Message> {
        match message {
        Message::GotoMainButton => shiftscreenfn(0),
        Message::GotoLangButton => shiftscreenfn(3),
        Message::ResumeButton => shiftscreenfn(1),
        Message::TableButton0 => index(0),
        Message::TableButton1 => index(1),
        Message::TableButton2 => index(2),
        Message::ShiftButton => shiftvaluefn(),
        Message::SubmitButton => sumbitfn(),
        Message::SpaceButton => pushfn(String::from(" ")),
        Message::DeleteButton => popfn(),
        Message::DeleteallButton => clearfn(),
        Message::NextButton => nextfn(),
        Message::ButtonPressed0 => pushfn(String::from("a")),
        Message::ButtonPressed1 => pushfn(String::from("b")),
        Message::ButtonPressed2 => pushfn(String::from("c")),
        Message::ButtonPressed3 => pushfn(String::from("d")),
        Message::ButtonPressed4 => pushfn(String::from("e")),
        Message::ButtonPressed5 => pushfn(String::from("f")),
        Message::ButtonPressed6 => pushfn(String::from("g")),
        Message::ButtonPressed7 => pushfn(String::from("h")),
        Message::ButtonPressed8 => pushfn(String::from("i")),
        Message::ButtonPressed9 => pushfn(String::from("j")),
        Message::ButtonPressed10 => pushfn(String::from("k")),
        Message::ButtonPressed11 => pushfn(String::from("l")),
        Message::ButtonPressed12 => pushfn(String::from("m")),
        Message::ButtonPressed13 => pushfn(String::from("n")),
        Message::ButtonPressed14 => pushfn(String::from("o")),
        Message::ButtonPressed15 => pushfn(String::from("p")),
        Message::ButtonPressed16 => pushfn(String::from("q")),
        Message::ButtonPressed17 => pushfn(String::from("r")),
        Message::ButtonPressed18 => pushfn(String::from("s")),
        Message::ButtonPressed19 => pushfn(String::from("t")),
        Message::ButtonPressed20 => pushfn(String::from("u")),
        Message::ButtonPressed21 => pushfn(String::from("v")),
        Message::ButtonPressed22 => pushfn(String::from("w")),
        Message::ButtonPressed23 => pushfn(String::from("x")),
        Message::ButtonPressed24 => pushfn(String::from("y")),
        Message::ButtonPressed25 => pushfn(String::from("z")),
        Message::ButtonPressed26 => pushfn(String::from("ẳ")),
        Message::ButtonPressed27 => pushfn(String::from("á")),
        Message::ButtonPressed28 => pushfn(String::from("â")),
        Message::ButtonPressed29 => pushfn(String::from("à")),
        Message::ButtonPressed30 => pushfn(String::from("ạ")),
        Message::ButtonPressed31 => pushfn(String::from("ầ")),
        Message::ButtonPressed32 => pushfn(String::from("ậ")),
        Message::ButtonPressed33 => pushfn(String::from("ấ")),
        Message::ButtonPressed34 => pushfn(String::from("ả")),
        Message::ButtonPressed35 => pushfn(String::from("ặ")),
        Message::ButtonPressed36 => pushfn(String::from("đ")),
        Message::ButtonPressed37 => pushfn(String::from("ỏ")),
        Message::ButtonPressed38 => pushfn(String::from("ơ")),
        Message::ButtonPressed39 => pushfn(String::from("ờ")),
        Message::ButtonPressed40 => pushfn(String::from("ồ")),
        Message::ButtonPressed41 => pushfn(String::from("ó")),
        Message::ButtonPressed42 => pushfn(String::from("ô")),
        Message::ButtonPressed43 => pushfn(String::from("ọ")),
        Message::ButtonPressed44 => pushfn(String::from("ộ")),
        Message::ButtonPressed45 => pushfn(String::from("ớ")),
        Message::ButtonPressed46 => pushfn(String::from("ở")),
        Message::ButtonPressed47 => pushfn(String::from("ư")),
        Message::ButtonPressed48 => pushfn(String::from("ụ")),
        Message::ButtonPressed49 => pushfn(String::from("ữ")),
        Message::ButtonPressed50 => pushfn(String::from("ú")),
        Message::ButtonPressed51 => pushfn(String::from("ủ")),
        Message::ButtonPressed52 => pushfn(String::from("í")),
        Message::ButtonPressed53 => pushfn(String::from("ì")),
        Message::ButtonPressed54 => pushfn(String::from("ị")),
        Message::ButtonPressed55 => pushfn(String::from("ế")),
        Message::ButtonPressed56 => pushfn(String::from("ẹ")),
        Message::ButtonPressed57 => pushfn(String::from("ể")),
        Message::ButtonPressed58 => pushfn(String::from("ề")),
        Message::ButtonPressed59 => pushfn(String::from("(")),
        Message::ButtonPressed60 => pushfn(String::from(")")),
        Message::ButtonPressed61 => pushfn(String::from(";")),
        Message::ButtonPressed62 => pushfn(String::from(":")),
        Message::ButtonPressed63 => pushfn(String::from(",")),
        Message::ButtonPressed64 => pushfn(String::from(".")),
        Message::ButtonPressed65 => pushfn(String::from("?")),
        Message::ButtonPressed66 => pushfn(String::from("!")),
        Message::LangButton0 => changelang(0),
        Message::LangButton1 => changelang(1),
        Message::EventOccurred(event) => {
        if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Space, modifiers: _ }) = event {
            pushfn(String::from(" "))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::LShift, modifiers: _ }) = event {
            shiftvaluefn()
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Right, modifiers: _ }) = event {
            nextfn()
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Enter, modifiers: _ }) = event {
            sumbitfn()
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Backspace, modifiers: _ }) = event {
            popfn()
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::A, modifiers: _ }) = event {
            pushfn(String::from("a"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::B, modifiers: _ }) = event {
            pushfn(String::from("b"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::C, modifiers: _ }) = event {
            pushfn(String::from("c"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::D, modifiers: _ }) = event {
            pushfn(String::from("d"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::E, modifiers: _ }) = event {
            pushfn(String::from("e"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::F, modifiers: _ }) = event {
            pushfn(String::from("f"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::G, modifiers: _ }) = event {
            pushfn(String::from("g"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::H, modifiers: _ }) = event {
            pushfn(String::from("h"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::I, modifiers: _ }) = event {
            pushfn(String::from("i"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::J, modifiers: _ }) = event {
            pushfn(String::from("j"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::K, modifiers: _ }) = event {
            pushfn(String::from("k"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::L, modifiers: _ }) = event {
            pushfn(String::from("l"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::M, modifiers: _ }) = event {
            pushfn(String::from("m"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::N, modifiers: _ }) = event {
            pushfn(String::from("n"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::O, modifiers: _ }) = event {
            pushfn(String::from("o"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::P, modifiers: _ }) = event {
            pushfn(String::from("p"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Q, modifiers: _ }) = event {
            pushfn(String::from("q"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::R, modifiers: _ }) = event {
            pushfn(String::from("r"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::S, modifiers: _ }) = event {
            pushfn(String::from("s"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::T, modifiers: _ }) = event {
            pushfn(String::from("t"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::U, modifiers: _ }) = event {
            pushfn(String::from("u"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::V, modifiers: _ }) = event {
            pushfn(String::from("v"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::W, modifiers: _ }) = event {
            pushfn(String::from("w"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::X, modifiers: _ }) = event {
            pushfn(String::from("x"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Y, modifiers: _ }) = event {
            pushfn(String::from("y"))
        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Z, modifiers: _ }) = event {
            pushfn(String::from("z"))
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
      } else if SCREEN.lock_mut().unwrap()[0] == 3 {
          return makelang(self);
      } else {
          return makemain(self); 
      } 
    }
}
fn loadsettings() {
    SETTINGS_BOOL.lock_mut().unwrap().clear();
    SETTINGS_USIZE.lock_mut().unwrap().clear();
    let filename = "./settings.toml";
    let contents = fs::read_to_string(filename).unwrap();
    let data: Data = toml::from_str(&contents).unwrap();
    let boollist = 
    [ 
        data.settings.seperate_check_synonyms,
        data.settings.sound.sound,
        data.settings.time.timed,
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
    SCREEN.lock_mut().unwrap().push(0);
    X.lock_mut().unwrap().push(0);
    let setting: iced::Settings<()> = Settings {
        window: window::Settings {
            size: (900, 600),resizable: true,decorations: true,min_size: Some((900 as u32,600 as u32)),max_size: Some((2000 as u32,2000 as u32)),transparent: false,always_on_top: false,icon: Some(Icon::from_rgba(rgba, 1, 1).unwrap()),position: Specific(0, 0),        },default_font: Some(include_bytes!("../../resources/Arial Unicode MS Font.ttf")),antialiasing: true,id: Some("October".to_string()),flags: (),default_text_size: 20,text_multithreading: true,exit_on_close_request: true,try_opengles_first: false,
    };
    MyButton::run(setting)
}
