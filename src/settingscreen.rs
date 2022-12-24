use crate::Mainstruct;
use crate::Message;
use crate::makeslider;
use crate::basic::{h1, h2, h3, h4, body};
use crate::elements::add_button;
use iced::{Element, Length, Column, Container, Checkbox, Row, Scrollable};
use futures::executor::block_on;

pub fn settingscreen(selfx: &mut Mainstruct) -> Element<Message>{
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