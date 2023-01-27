use crate::basic::h4;
use crate::elements::add_button;
use crate::Mainstruct;
use crate::Message;
use iced::{Column, Container, Element, Length, Text};
pub fn datascreen(selfx: &mut Mainstruct) -> Element<Message> {
    let state = selfx.button_states.get_mut("gotomain").unwrap();

    let n = Text::new(format!("Word_Index: {:?}", selfx.word_index));
    let table = Text::new(format!("table: {:?}", selfx.table));
    let lang = Text::new(format!("Lang: {:?}", selfx.lang));
    let english = h4(
        selfx.settings_usize[5],
        selfx.langonevec[selfx.word_index].to_string(),
    )
    .height(Length::Units(60));

    let vietnamese = h4(
        selfx.settings_usize[5],
        selfx.langtwovec[selfx.word_index].to_string(),
    )
    .height(Length::Units(60));
    let maincolumn = Column::new()
        .push(n)
        .push(table)
        .push(lang)
        .push(english)
        .push(vietnamese);
    let exit = add_button(
        state,
        "exit".to_string(),
        Message::ButtonPressed("Exit".to_string()),
        selfx.settings_usize[6],
    );
    let main: Element<Message> = Container::new(maincolumn.push(exit))
        .padding(100)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into();
    main
}
