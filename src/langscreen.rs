use crate::Mainstruct;
use crate::Message;
use crate::load::loadnames;
use iced::{Element, Length, Column, Container, Button};
use futures::executor::block_on;
use crate::elements::add_button;

pub fn langscreen<'a>(selfx: &'a mut Mainstruct) -> Element<'a, Message>{
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