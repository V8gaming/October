#![feature(map_many_mut)]
use iced::{
    window, window::Icon, window::Position::Specific, Application, Command, Element,
    Settings as IcedSettings, Subscription,
};
use iced_native::{keyboard, Event};
// Global variables for storing the letters to be typed, the English and Vietnamese words,
// and other program state such as the current screen, language, and time.

mod appfunctions;
mod basic;
mod datascreen;
mod elements;
mod file;
mod langscreen;
mod load;
mod mainscreen;
mod reviewscreen;
mod settingscreen;
mod structs;
mod testingscreens;
mod text;
mod types;

use appfunctions::{changelang, index, nextfn, shiftscreenfn, sumbitfn};
use datascreen::datascreen;
use file::savefn;
use langscreen::langscreen;
use load::loadsettings;
use mainscreen::mainscreen;
use reviewscreen::reviewscreen;
use settingscreen::settingscreen;
use structs::{Data, Mainstruct};
use testingscreens::reviewscreen as review;
use text::{popfn, pushfn, shiftvaluefn};
use types::{Executor, Flags, Message};

impl Application for Mainstruct {
    type Executor = Executor;
    type Message = Message;
    type Flags = Flags;

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn new(_flags: ()) -> (Mainstruct, Command<Message>) {
        (Mainstruct::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("October")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LoadButton => loadsettings(self),
            Message::IndexSent(usize) => index(self, usize),
            Message::LangChange(usize) => changelang(self, usize),
            Message::ButtonPressed(string) => match string.as_str() {
                "Save" => savefn(self),
                "Exit" => shiftscreenfn(self, 0),
                "Settings" => shiftscreenfn(self, 4),
                "Languages" => shiftscreenfn(self, 3),
                "Data" => shiftscreenfn(self, 5),
                "Resume" => shiftscreenfn(self, 1),
                "Shift" => shiftvaluefn(self),
                "Submit" => sumbitfn(self),
                "Space" => pushfn(self, String::from(" ")),
                "Delete" => popfn(self),
                "Next" => nextfn(self),
                _ => (),
            },
            Message::LetterPressed(string) => pushfn(self, string),
            Message::EventOccurred(event) => match event {
                Event::Keyboard(keyboard::Event::KeyReleased {
                    key_code: keyboard::KeyCode::LShift,
                    modifiers: _,
                }) => {
                    shiftvaluefn(self);
                }
                Event::Keyboard(keyboard::Event::KeyReleased {
                    key_code: keyboard::KeyCode::Right,
                    modifiers: _,
                }) => {
                    nextfn(self);
                }
                Event::Keyboard(keyboard::Event::KeyReleased {
                    key_code: keyboard::KeyCode::Enter,
                    modifiers: _,
                }) => {
                    sumbitfn(self);
                }
                Event::Keyboard(keyboard::Event::KeyReleased {
                    key_code: keyboard::KeyCode::Backspace,
                    modifiers: _,
                }) => {
                    popfn(self);
                }
                Event::Keyboard(keyboard::Event::CharacterReceived(char)) => {
                    pushfn(self, String::from(char))
                }
                _ => (),
            },

            Message::SeperateCheckBox(state) => self.settings_bool[0] = state,
            Message::SoundBox(state) => self.settings_bool[1] = state,
            Message::TimedBox(state) => self.settings_bool[2] = state,
            Message::VolumeSlider(num) => self.settings_usize[0] = num as usize,
            Message::LengthSlider(num) => self.settings_usize[1] = num as usize,
            Message::H1Slider(num) => self.settings_usize[2] = num as usize,
            Message::H2Slider(num) => self.settings_usize[3] = num as usize,
            Message::H3Slider(num) => self.settings_usize[4] = num as usize,
            Message::H4Slider(num) => self.settings_usize[5] = num as usize,
            Message::BodySlider(num) => self.settings_usize[6] = num as usize,
        };

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self.screen {
            0 => mainscreen(self),
            1 => review(self),
            2 => reviewscreen(self),
            3 => langscreen(self),
            4 => settingscreen(self),
            5 => datascreen(self),
            _ => mainscreen(self),
        }
    }
}

fn main() -> iced::Result {
    let rgba = vec![0, 0, 0, 255];
    let setting: IcedSettings<()> = IcedSettings {
        window: window::Settings {
            size: (800, 600),
            resizable: true,
            decorations: true,
            min_size: Some((100_u32, 100_u32)),
            max_size: Some((2000_u32, 2000_u32)),
            transparent: false,
            always_on_top: false,
            icon: Some(Icon::from_rgba(rgba, 1, 1).unwrap()),
            position: Specific(0, 0),
        },
        default_font: Some(include_bytes!("../resources/Arial Unicode MS Font.ttf")),
        antialiasing: false,
        id: Some("October".to_string()),
        flags: (),
        default_text_size: 20,
        text_multithreading: true,
        exit_on_close_request: true,
        try_opengles_first: false,
    };
    Mainstruct::run(setting)
}
