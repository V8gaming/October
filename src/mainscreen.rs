use crate::Mainstruct;
use crate::Message;
use crate::load::{loadsettings, loadsizes, loadhashmaps, loaddata, loadsize, tablenames};
use crate::elements::add_button;
use crate::basic::h1;
use iced::{Element, Container, Length, Button, Column};
use futures::executor::block_on;
use rand::{thread_rng, Rng};

pub fn mainscreen(selfx: &mut Mainstruct) -> Element<Message>{
    loadsettings(selfx);
    loadsizes(selfx);
    loadhashmaps(selfx);
    loaddata(selfx);
    selfx.colour = thread_rng().gen_range(0..selfx.langonevec.len().try_into().unwrap());
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