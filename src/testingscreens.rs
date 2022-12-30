use crate::{Mainstruct, Message, basic::*};
use crate::appfunctions::sumbitfn;
use crate::text::shiftfn;
use crate::elements::add_button;
use iced::{Length, Element, Color, Button, Row, Column, Container};
use futures::executor::block_on;

pub fn reviewscreen(selfx: &mut Mainstruct) -> Element<Message>{
    let timer = h4(selfx.settings_usize[5],block_on(get_time(selfx)));
    
    if selfx.time.elapsed().as_secs() >= selfx.settings_usize[1] as u64 {
        sumbitfn(selfx)
    }


    let english = h2(selfx.settings_usize[3],format!("{}",selfx.langonevec[selfx.word_index] ))
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
    
    for i in selfx.lang_one_states.values_mut() {
        buttonone.push(block_on(add_button(i, shiftfn(selfx.shift, selfx.langone[x].to_string()),Message::LetterPressed(selfx.langone[x].to_string()), selfx.settings_usize[6])));
        x+=1;
    }

    x=0;
    let mut buttontwo: Vec<Button<Message>> = Vec::new();
    for i in selfx.lang_two_states.values_mut(){
        buttontwo.push(block_on(add_button(i, shiftfn(selfx.shift,selfx.langtwo[x].to_string()),Message::LetterPressed(selfx.langtwo[x].to_string()), selfx.settings_usize[6])));
        x+=1;
    }
    x=0;
    let mut buttons: Vec<Button<Message>> = Vec::new();
    for i in selfx.punctuation_states.values_mut(){
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