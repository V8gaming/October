use iced::{button, Button, Element, Sandbox, Settings, Text, Container, Length, Column, Row, window};
use iced::window::Position::Specific;
use iced::window::Icon;
use global::Global;
use rand::{thread_rng, Rng};
static LETTERS: Global<Vec<String>> = Global::new();
static ENGLISH: Global<Vec<&str>> = Global::new();
static VIETNAMESE: Global<Vec<&str>> = Global::new();
static N: Global<Vec<usize>> = Global::new();

#[derive(Default, Clone)]
struct MyButton {
    submit_state: button::State,
    space_state: button::State,
    delete_state: button::State,
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
}

#[derive(Debug, Clone)]enum Message {
    SubmitButton,
    SpaceButton,
    DeleteButton,
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
}
fn pushfn(letter: &str) {
    LETTERS.lock_mut().unwrap().push(letter.to_string());
    println!("ADDED {} TO {}", letter,LETTERS.lock_mut().unwrap().concat());
}
fn sumbitfn() {
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
LETTERS.lock_mut().unwrap().clear();
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
        Message::ButtonPressed4 => pushfn("e"),
        Message::ButtonPressed5 => pushfn("f"),
        Message::ButtonPressed6 => pushfn("g"),
        Message::ButtonPressed7 => pushfn("h"),
        Message::ButtonPressed8 => pushfn("i"),
        Message::ButtonPressed9 => pushfn("j"),
        Message::ButtonPressed10 => pushfn("k"),
        Message::ButtonPressed11 => pushfn("l"),
        Message::ButtonPressed12 => pushfn("m"),
        Message::ButtonPressed13 => pushfn("n"),
        Message::ButtonPressed14 => pushfn("o"),
        Message::ButtonPressed15 => pushfn("p"),
        Message::ButtonPressed16 => pushfn("q"),
        Message::ButtonPressed17 => pushfn("r"),
        Message::ButtonPressed18 => pushfn("s"),
        Message::ButtonPressed19 => pushfn("t"),
        Message::ButtonPressed20 => pushfn("u"),
        Message::ButtonPressed21 => pushfn("v"),
        Message::ButtonPressed22 => pushfn("w"),
        Message::ButtonPressed23 => pushfn("x"),
        Message::ButtonPressed24 => pushfn("y"),
        Message::ButtonPressed25 => pushfn("z"),
        Message::ButtonPressed26 => pushfn("ẳ"),
        Message::ButtonPressed27 => pushfn("á"),
        Message::ButtonPressed28 => pushfn("â"),
        Message::ButtonPressed29 => pushfn("à"),
        Message::ButtonPressed30 => pushfn("ạ"),
        Message::ButtonPressed31 => pushfn("ầ"),
        Message::ButtonPressed32 => pushfn("ậ"),
        Message::ButtonPressed33 => pushfn("ấ"),
        Message::ButtonPressed34 => pushfn("ả"),
        Message::ButtonPressed35 => pushfn("ặ"),
        Message::ButtonPressed36 => pushfn("đ"),
        Message::ButtonPressed37 => pushfn("ỏ"),
        Message::ButtonPressed38 => pushfn("ơ"),
        Message::ButtonPressed39 => pushfn("ờ"),
        Message::ButtonPressed40 => pushfn("ồ"),
        Message::ButtonPressed41 => pushfn("ó"),
        Message::ButtonPressed42 => pushfn("ô"),
        Message::ButtonPressed43 => pushfn("ọ"),
        Message::ButtonPressed44 => pushfn("ư"),
        Message::ButtonPressed45 => pushfn("ụ"),
        Message::ButtonPressed46 => pushfn("ữ"),
        Message::ButtonPressed47 => pushfn("ú"),
        Message::ButtonPressed48 => pushfn("ủ"),
        Message::ButtonPressed49 => pushfn("í"),
        Message::ButtonPressed50 => pushfn("ì"),
        Message::ButtonPressed51 => pushfn("ị"),
        Message::ButtonPressed52 => pushfn("ế"),
        Message::ButtonPressed53 => pushfn("ẹ"),
        Message::ButtonPressed54 => pushfn("ể"),
      }
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
    let buttons1 = [
        add_button(&mut self.button_state0, "a", Message::ButtonPressed0),
        add_button(&mut self.button_state1, "b", Message::ButtonPressed1),
        add_button(&mut self.button_state2, "c", Message::ButtonPressed2),
        add_button(&mut self.button_state3, "d", Message::ButtonPressed3),
        add_button(&mut self.button_state4, "e", Message::ButtonPressed4),
        add_button(&mut self.button_state5, "f", Message::ButtonPressed5),
        add_button(&mut self.button_state6, "g", Message::ButtonPressed6),
        add_button(&mut self.button_state7, "h", Message::ButtonPressed7),
        add_button(&mut self.button_state8, "i", Message::ButtonPressed8),
        add_button(&mut self.button_state9, "j", Message::ButtonPressed9),
        add_button(&mut self.button_state10, "k", Message::ButtonPressed10),
        add_button(&mut self.button_state11, "l", Message::ButtonPressed11),
        add_button(&mut self.button_state12, "m", Message::ButtonPressed12),
        add_button(&mut self.button_state13, "n", Message::ButtonPressed13),
        add_button(&mut self.button_state14, "o", Message::ButtonPressed14),
        add_button(&mut self.button_state15, "p", Message::ButtonPressed15),
        add_button(&mut self.button_state16, "q", Message::ButtonPressed16),
        add_button(&mut self.button_state17, "r", Message::ButtonPressed17),
        add_button(&mut self.button_state18, "s", Message::ButtonPressed18),
        add_button(&mut self.button_state19, "t", Message::ButtonPressed19),
        add_button(&mut self.button_state20, "u", Message::ButtonPressed20),
        add_button(&mut self.button_state21, "v", Message::ButtonPressed21),
        add_button(&mut self.button_state22, "w", Message::ButtonPressed22),
        add_button(&mut self.button_state23, "x", Message::ButtonPressed23),
        add_button(&mut self.button_state24, "y", Message::ButtonPressed24),
        add_button(&mut self.button_state25, "z", Message::ButtonPressed25),
    ];
    let buttons2 = [
        add_button(&mut self.button_state26, "ẳ", Message::ButtonPressed26),
        add_button(&mut self.button_state27, "á", Message::ButtonPressed27),
        add_button(&mut self.button_state28, "â", Message::ButtonPressed28),
        add_button(&mut self.button_state29, "à", Message::ButtonPressed29),
        add_button(&mut self.button_state30, "ạ", Message::ButtonPressed30),
        add_button(&mut self.button_state31, "ầ", Message::ButtonPressed31),
        add_button(&mut self.button_state32, "ậ", Message::ButtonPressed32),
        add_button(&mut self.button_state33, "ấ", Message::ButtonPressed33),
        add_button(&mut self.button_state34, "ả", Message::ButtonPressed34),
        add_button(&mut self.button_state35, "ặ", Message::ButtonPressed35),
        add_button(&mut self.button_state36, "đ", Message::ButtonPressed36),
        add_button(&mut self.button_state37, "ỏ", Message::ButtonPressed37),
        add_button(&mut self.button_state38, "ơ", Message::ButtonPressed38),
        add_button(&mut self.button_state39, "ờ", Message::ButtonPressed39),
        add_button(&mut self.button_state40, "ồ", Message::ButtonPressed40),
        add_button(&mut self.button_state41, "ó", Message::ButtonPressed41),
        add_button(&mut self.button_state42, "ô", Message::ButtonPressed42),
        add_button(&mut self.button_state43, "ọ", Message::ButtonPressed43),
        add_button(&mut self.button_state44, "ư", Message::ButtonPressed44),
        add_button(&mut self.button_state45, "ụ", Message::ButtonPressed45),
        add_button(&mut self.button_state46, "ữ", Message::ButtonPressed46),
        add_button(&mut self.button_state47, "ú", Message::ButtonPressed47),
        add_button(&mut self.button_state48, "ủ", Message::ButtonPressed48),
        add_button(&mut self.button_state49, "í", Message::ButtonPressed49),
        add_button(&mut self.button_state50, "ì", Message::ButtonPressed50),
        add_button(&mut self.button_state51, "ị", Message::ButtonPressed51),
        add_button(&mut self.button_state52, "ế", Message::ButtonPressed52),
        add_button(&mut self.button_state53, "ẹ", Message::ButtonPressed53),
        add_button(&mut self.button_state54, "ể", Message::ButtonPressed54),
    ];
    let submit = add_button(&mut self.submit_state, "submit", Message::SubmitButton);
    let space = add_button(&mut self.space_state, "space", Message::SpaceButton);
    let delete = add_button(&mut self.delete_state, "delete", Message::DeleteButton);
    let next = add_button(&mut self.next_state, "next", Message::NextButton);
    let mut userrow = Row::new();
    userrow = userrow.push(submit);
    userrow = userrow.push(space);
    userrow = userrow.push(delete);
    userrow = userrow.push(next);
    let mut row1 = Row::new();
    for button in buttons1 {
        row1 = row1.push(button);
    };
    let mut row2 = Row::new();
    for button in buttons2 {
        row2 = row2.push(button);
    };
    let column1 = Column::new().push(english).push(text1).push(userrow).push(row1).push(row2).width(Length::Fill).align_items(iced::Alignment::Center);
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
            size: (800, 600),resizable: true,decorations: true,min_size: Some((100 as u32,100 as u32)),max_size: Some((2000 as u32,2000 as u32)),transparent: false,always_on_top: true,icon: Some(Icon::from_rgba(rgba, 1, 1).unwrap()),position: Specific(0, 0),        },default_font: Some(include_bytes!("../../resources/Arial Unicode MS Font.ttf")),antialiasing: true,id: Some("buttons".to_string()),flags: (),default_text_size: 20,text_multithreading: true,exit_on_close_request: true,try_opengles_first: false,
    };
    MyButton::run(setting)
}
