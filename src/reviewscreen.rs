use crate::basic::{h3, h4};
use crate::elements::add_button;
use crate::Mainstruct;
use crate::Message;
use iced::{alignment, Color, Column, Container, Element, Length};

pub fn reviewscreen(selfx: &mut Mainstruct) -> Element<Message> {
    let [state0, state1] = selfx
        .button_states
        .get_many_mut(["gotomain", "gototesting"])
        .unwrap();

    let exit = add_button(
        state0,
        String::from("Exit"),
        Message::ButtonPressed("Exit".to_string()),
        selfx.settings_usize[6],
    );
    let colours = vec![
        Color::BLACK,
        Color::from_rgb(1.0, 0.0, 0.0),
        Color::from_rgb(0.0, 1.0, 0.0),
    ];

    let subtitle1 = h3(selfx.settings_usize[4], String::from("Your Response"))
        .color(colours[selfx.colour as usize])
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(Length::Fill);

    let subtitle2 = h3(selfx.settings_usize[4], String::from("Vietnamese"))
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(Length::Fill);

    let subtitle3 = h3(selfx.settings_usize[4], String::from("English"))
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(Length::Fill);

    let response = h4(selfx.settings_usize[5], selfx.letters.concat())
        .height(Length::Units(60))
        .color(colours[selfx.colour as usize]);
    let langone = h4(
        selfx.settings_usize[5],
        selfx.langonevec[selfx.word_index].to_string(),
    )
    .height(Length::Units(60));

    let langtwo = h4(
        selfx.settings_usize[5],
        selfx.langtwovec[selfx.word_index].to_string(),
    )
    .height(Length::Units(60));

    let resume = add_button(
        state1,
        String::from("Resume"),
        Message::ButtonPressed("Resume".to_string()),
        selfx.settings_usize[6],
    );
    let column = Column::new()
        .push(exit)
        .push(subtitle1)
        .push(response)
        .push(subtitle2)
        .push(langtwo)
        .push(subtitle3)
        .push(langone)
        .push(resume);
    let review: Element<Message> = Container::new(column)
        .padding(100)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .into();

    review
}
