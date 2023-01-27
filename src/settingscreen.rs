use crate::basic::{body, h1, h2, h3, h4};
use crate::elements::add_button;
use crate::makeslider;
use crate::Mainstruct;
use crate::Message;
use iced::{Checkbox, Column, Container, Element, Length, Row, Scrollable};

pub fn settingscreen(selfx: &mut Mainstruct) -> Element<Message> {
    let h2_general = h2(selfx.settings_usize[3], "General Settings".to_string());
    let seperatecheckbox = Checkbox::new(
        selfx.settings_bool[0],
        "Seperately check synonyms",
        Message::SeperateCheckBox,
    );
    let h2_sound = h2(selfx.settings_usize[3], "Sound Settings".to_string());
    let soundbox = Checkbox::new(selfx.settings_bool[1], "Sound Enabled", Message::SoundBox);
    let volumeslider = makeslider!(
        &mut selfx.settings.volume,
        100,
        selfx.settings_usize[0],
        Message::VolumeSlider
    );
    let h2_time = h2(selfx.settings_usize[3], "Time Settings".to_string());
    let lengthbox = Checkbox::new(selfx.settings_bool[2], "Timed", Message::TimedBox);
    let lengthslider = makeslider!(
        &mut selfx.settings.length,
        1,
        30,
        selfx.settings_usize[1],
        Message::LengthSlider
    );
    let h2_text = h2(selfx.settings_usize[3], "Text Settings".to_string());
    let h1slider = makeslider!(
        &mut selfx.settings.h1,
        200,
        selfx.settings_usize[2],
        Message::H1Slider
    );
    let h2slider = makeslider!(
        &mut selfx.settings.h2,
        200,
        selfx.settings_usize[3],
        Message::H2Slider
    );
    let h3slider = makeslider!(
        &mut selfx.settings.h3,
        200,
        selfx.settings_usize[4],
        Message::H3Slider
    );
    let h4slider = makeslider!(
        &mut selfx.settings.h4,
        200,
        selfx.settings_usize[5],
        Message::H4Slider
    );
    let bodyslider = makeslider!(
        &mut selfx.settings.body,
        200,
        selfx.settings_usize[6],
        Message::BodySlider
    );
    let volume = h4(
        selfx.settings_usize[5],
        format!("Volume: {}", selfx.settings_usize[0]),
    );
    let length = h4(
        selfx.settings_usize[5],
        format!("Time: {}", selfx.settings_usize[1]),
    );
    let h1 = h1(
        selfx.settings_usize[2],
        format!("H1: {}", selfx.settings_usize[2]),
    );
    let h2 = h2(
        selfx.settings_usize[3],
        format!("H2: {}", selfx.settings_usize[3]),
    );
    let h3 = h3(
        selfx.settings_usize[4],
        format!("H3: {}", selfx.settings_usize[4]),
    );
    let h4 = h4(
        selfx.settings_usize[5],
        format!("H4: {}", selfx.settings_usize[5]),
    );
    let body = body(
        selfx.settings_usize[6],
        format!("Body: {}", selfx.settings_usize[6]),
    );
    let reset = add_button(
        &mut selfx.settings.load_state,
        "Reload Default".to_string(),
        Message::LoadButton,
        selfx.settings_usize[6],
    );
    let [state0, state1] = selfx
        .button_states
        .get_many_mut(["save", "gotomain"])
        .unwrap();

    let save = add_button(
        state0,
        "Save".to_string(),
        Message::ButtonPressed("Save".to_string()),
        selfx.settings_usize[6],
    );
    let exit = add_button(
        state1,
        "Exit".to_string(),
        Message::ButtonPressed("Exit".to_string()),
        selfx.settings_usize[6],
    );
    let row = Row::new().push(save).push(exit).push(reset);

    let settingcolumn = Column::new()
        .push(h2_general)
        .push(seperatecheckbox)
        .push(h2_sound)
        .push(soundbox)
        .push(volume)
        .push(volumeslider)
        .push(h2_time)
        .push(lengthbox)
        .push(length)
        .push(lengthslider)
        .push(h2_text)
        .push(h1)
        .push(h1slider)
        .push(h2)
        .push(h2slider)
        .push(h3)
        .push(h3slider)
        .push(h4)
        .push(h4slider)
        .push(body)
        .push(bodyslider)
        .push(row);

    let padding = Container::new(settingcolumn)
        .padding(50)
        .width(Length::Fill);
    let scroll = Scrollable::new(&mut selfx.settings.scrollable_state).push(padding);
    let main: Element<Message> = Container::new(scroll)
        .padding(50)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into();
    main
}
