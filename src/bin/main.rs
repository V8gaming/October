use iced::{button, Button, Element, Sandbox, Settings, Text, Container, Length, Column, Row, window, Color};
use iced::window::Position::Specific;
use iced::window::Icon;
use global::Global;
use rand::{thread_rng, Rng};
static LETTERS: Global<Vec<String>> = Global::new();
static ENGLISH: Global<Vec<&str>> = Global::new();
static VIETNAMESE: Global<Vec<&str>> = Global::new();
static N: Global<Vec<usize>> = Global::new();
static COLOUR: Global<Vec<usize>> = Global::new();
static X: Global<Vec<usize>> = Global::new();

#[derive(Default, Clone)]
struct MyButton {
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
}

#[derive(Debug, Clone)]enum Message {
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
}
fn pushfn(letter:String) {
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
fn sumbitfn() {
    let vietnamese = ["của chị ấy", "vâng", "có thể", "không thể", "làm", "ở", "với", "rất", "về", "một", "làm ơn", "phải", "trái"];
    for i in vietnamese {
        VIETNAMESE.lock_mut().unwrap().push(i)
    }
    if format!("{}", LETTERS.lock_mut().unwrap().concat()) == VIETNAMESE.lock_mut().unwrap()[N.lock_mut().unwrap()[0]]{
        COLOUR.lock_mut().unwrap()[0] = 2;
        println!("true")
    } else {
        COLOUR.lock_mut().unwrap()[0] = 1;
        println!("false")
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
fn nextfn() {
N.lock_mut().unwrap()[0] = thread_rng().gen_range(0..13);
LETTERS.lock_mut().unwrap().clear();
COLOUR.lock_mut().unwrap()[0] = 0
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
      }
    }

  fn view(&mut self) -> Element<Message> {
      fn add_button<'a>(a: &'a mut button::State,b: String,c: Message) -> Button<'a, Message> {

          return Button::new(a, Text::new(format!("{}",b))).on_press(c);

      }
        let tables = ["Prepositions"];
        let english = ["hers", "yes", "can, to be able to", "can not, don't be able to", "to do", "at, in", "with", "very (general comment/opinion - placed before the adjective)", "about", "one, a, an", "please", "must, right, right side", "left (side)"];
        for i in english {
           ENGLISH.lock_mut().unwrap().push(i)
        }
        let texttype = Text::new(format!("{}",tables[0] )).height(Length::Units(30)).size(20).color(Color::from_rgb(1.0,0.0,1.0));
        let english = Text::new(format!("{}",ENGLISH.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Units(100)).size(80);
        let colours = vec![Color::BLACK,Color::from_rgb(1.0, 0.0, 0.0),Color::from_rgb(0.0, 1.0, 0.0)];
        let text1 = Text::new(format!("{}", LETTERS.lock_mut().unwrap().concat())).height(Length::Units(100)).size(80).color(colours[COLOUR.lock_mut().unwrap()[0]]);
    let buttons1 = [
        add_button(&mut self.button_state0, shiftfn(String::from("a")), Message::ButtonPressed0),
        add_button(&mut self.button_state1, shiftfn(String::from("b")), Message::ButtonPressed1),
        add_button(&mut self.button_state2, shiftfn(String::from("c")), Message::ButtonPressed2),
        add_button(&mut self.button_state3, shiftfn(String::from("d")), Message::ButtonPressed3),
        add_button(&mut self.button_state4, shiftfn(String::from("e")), Message::ButtonPressed4),
        add_button(&mut self.button_state5, shiftfn(String::from("f")), Message::ButtonPressed5),
        add_button(&mut self.button_state6, shiftfn(String::from("g")), Message::ButtonPressed6),
        add_button(&mut self.button_state7, shiftfn(String::from("h")), Message::ButtonPressed7),
        add_button(&mut self.button_state8, shiftfn(String::from("i")), Message::ButtonPressed8),
        add_button(&mut self.button_state9, shiftfn(String::from("j")), Message::ButtonPressed9),
        add_button(&mut self.button_state10, shiftfn(String::from("k")), Message::ButtonPressed10),
        add_button(&mut self.button_state11, shiftfn(String::from("l")), Message::ButtonPressed11),
        add_button(&mut self.button_state12, shiftfn(String::from("m")), Message::ButtonPressed12),
        add_button(&mut self.button_state13, shiftfn(String::from("n")), Message::ButtonPressed13),
        add_button(&mut self.button_state14, shiftfn(String::from("o")), Message::ButtonPressed14),
        add_button(&mut self.button_state15, shiftfn(String::from("p")), Message::ButtonPressed15),
        add_button(&mut self.button_state16, shiftfn(String::from("q")), Message::ButtonPressed16),
        add_button(&mut self.button_state17, shiftfn(String::from("r")), Message::ButtonPressed17),
        add_button(&mut self.button_state18, shiftfn(String::from("s")), Message::ButtonPressed18),
        add_button(&mut self.button_state19, shiftfn(String::from("t")), Message::ButtonPressed19),
        add_button(&mut self.button_state20, shiftfn(String::from("u")), Message::ButtonPressed20),
        add_button(&mut self.button_state21, shiftfn(String::from("v")), Message::ButtonPressed21),
        add_button(&mut self.button_state22, shiftfn(String::from("w")), Message::ButtonPressed22),
        add_button(&mut self.button_state23, shiftfn(String::from("x")), Message::ButtonPressed23),
        add_button(&mut self.button_state24, shiftfn(String::from("y")), Message::ButtonPressed24),
        add_button(&mut self.button_state25, shiftfn(String::from("z")), Message::ButtonPressed25),
    ];
    let buttons2 = [
        add_button(&mut self.button_state26, shiftfn(String::from("ẳ")), Message::ButtonPressed26),
        add_button(&mut self.button_state27, shiftfn(String::from("á")), Message::ButtonPressed27),
        add_button(&mut self.button_state28, shiftfn(String::from("â")), Message::ButtonPressed28),
        add_button(&mut self.button_state29, shiftfn(String::from("à")), Message::ButtonPressed29),
        add_button(&mut self.button_state30, shiftfn(String::from("ạ")), Message::ButtonPressed30),
        add_button(&mut self.button_state31, shiftfn(String::from("ầ")), Message::ButtonPressed31),
        add_button(&mut self.button_state32, shiftfn(String::from("ậ")), Message::ButtonPressed32),
        add_button(&mut self.button_state33, shiftfn(String::from("ấ")), Message::ButtonPressed33),
        add_button(&mut self.button_state34, shiftfn(String::from("ả")), Message::ButtonPressed34),
        add_button(&mut self.button_state35, shiftfn(String::from("ặ")), Message::ButtonPressed35),
        add_button(&mut self.button_state36, shiftfn(String::from("đ")), Message::ButtonPressed36),
        add_button(&mut self.button_state37, shiftfn(String::from("ỏ")), Message::ButtonPressed37),
        add_button(&mut self.button_state38, shiftfn(String::from("ơ")), Message::ButtonPressed38),
        add_button(&mut self.button_state39, shiftfn(String::from("ờ")), Message::ButtonPressed39),
        add_button(&mut self.button_state40, shiftfn(String::from("ồ")), Message::ButtonPressed40),
        add_button(&mut self.button_state41, shiftfn(String::from("ó")), Message::ButtonPressed41),
        add_button(&mut self.button_state42, shiftfn(String::from("ô")), Message::ButtonPressed42),
        add_button(&mut self.button_state43, shiftfn(String::from("ọ")), Message::ButtonPressed43),
        add_button(&mut self.button_state44, shiftfn(String::from("ộ")), Message::ButtonPressed44),
        add_button(&mut self.button_state45, shiftfn(String::from("ớ")), Message::ButtonPressed45),
        add_button(&mut self.button_state46, shiftfn(String::from("ở")), Message::ButtonPressed46),
        add_button(&mut self.button_state47, shiftfn(String::from("ư")), Message::ButtonPressed47),
        add_button(&mut self.button_state48, shiftfn(String::from("ụ")), Message::ButtonPressed48),
        add_button(&mut self.button_state49, shiftfn(String::from("ữ")), Message::ButtonPressed49),
        add_button(&mut self.button_state50, shiftfn(String::from("ú")), Message::ButtonPressed50),
        add_button(&mut self.button_state51, shiftfn(String::from("ủ")), Message::ButtonPressed51),
        add_button(&mut self.button_state52, shiftfn(String::from("í")), Message::ButtonPressed52),
        add_button(&mut self.button_state53, shiftfn(String::from("ì")), Message::ButtonPressed53),
        add_button(&mut self.button_state54, shiftfn(String::from("ị")), Message::ButtonPressed54),
        add_button(&mut self.button_state55, shiftfn(String::from("ế")), Message::ButtonPressed55),
        add_button(&mut self.button_state56, shiftfn(String::from("ẹ")), Message::ButtonPressed56),
        add_button(&mut self.button_state57, shiftfn(String::from("ể")), Message::ButtonPressed57),
        add_button(&mut self.button_state58, shiftfn(String::from("ề")), Message::ButtonPressed58),
    ];
    let buttons3 = [
        add_button(&mut self.button_state59, shiftfn(String::from("(")), Message::ButtonPressed59),
        add_button(&mut self.button_state60, shiftfn(String::from(")")), Message::ButtonPressed60),
        add_button(&mut self.button_state61, shiftfn(String::from(";")), Message::ButtonPressed61),
        add_button(&mut self.button_state62, shiftfn(String::from(":")), Message::ButtonPressed62),
        add_button(&mut self.button_state63, shiftfn(String::from(",")), Message::ButtonPressed63),
        add_button(&mut self.button_state64, shiftfn(String::from(".")), Message::ButtonPressed64),
        add_button(&mut self.button_state65, shiftfn(String::from("?")), Message::ButtonPressed65),
        add_button(&mut self.button_state66, shiftfn(String::from("!")), Message::ButtonPressed66),
    ];
    let shift = add_button(&mut self.shift_state, String::from("shift"), Message::ShiftButton);
    let submit = add_button(&mut self.submit_state, String::from("submit"), Message::SubmitButton);
    let space = add_button(&mut self.space_state, String::from("space"), Message::SpaceButton);
    let delete = add_button(&mut self.delete_state, String::from("delete"), Message::DeleteButton);
    let deleteall = add_button(&mut self.deleteall_state, String::from("deleteall"), Message::DeleteallButton);
    let next = add_button(&mut self.next_state, String::from("next"), Message::NextButton);
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
    let column1 = Column::new().push(texttype).push(english).push(text1).push(userrow).push(row1).push(row2).push(row3).width(Length::Fill).align_items(iced::Alignment::Center);
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
    N.lock_mut().unwrap().push(thread_rng().gen_range(0..13));
    COLOUR.lock_mut().unwrap().push(0);
    X.lock_mut().unwrap().push(0);
    let setting: iced::Settings<()> = Settings {
        window: window::Settings {
            size: (900, 600),resizable: true,decorations: true,min_size: Some((900 as u32,600 as u32)),max_size: Some((2000 as u32,2000 as u32)),transparent: false,always_on_top: true,icon: Some(Icon::from_rgba(rgba, 1, 1).unwrap()),position: Specific(0, 0),        },default_font: Some(include_bytes!("../../resources/Arial Unicode MS Font.ttf")),antialiasing: true,id: Some("October".to_string()),flags: (),default_text_size: 20,text_multithreading: true,exit_on_close_request: true,try_opengles_first: false,
    };
    MyButton::run(setting)
}
