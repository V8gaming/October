// settings.rs
use iced::{button, scrollable, slider};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Deserialize, Debug)]
pub struct Data {
    pub settings: Settings,
}

#[derive(Deserialize, Debug)]
pub struct TextsizeData {
    pub h1: usize,
    pub h2: usize,
    pub h3: usize,
    pub h4: usize,
    pub body: usize,
}

#[derive(Deserialize, Debug)]
pub struct SoundData {
    pub sound: bool,
    pub volume: usize,
}

#[derive(Deserialize, Debug)]
pub struct TimeData {
    pub timed: bool,
    pub length: usize,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub seperate_check_synonyms: bool,
    pub sound: SoundData,
    pub textsize: TextsizeData,
    pub time: TimeData,
}
pub struct Mainstruct {
    pub settings_usize: Vec<usize>,
    pub settings_bool: Vec<bool>,
    pub settings_language: HashMap<&'static str, String>,
    pub letters: Vec<String>,
    pub langonevec: Vec<String>,
    pub langtwovec: Vec<String>,
    pub langone: Vec<&'static str>,
    pub langtwo: Vec<&'static str>,
    pub punctuation: [&'static str; 8],
    pub time: Instant,
    pub screen: u32,
    pub colour: u32,
    pub maxstates: u32,
    pub table: usize,
    pub lang: usize,
    pub shift: bool,
    pub word_index: usize,
    pub button_states: HashMap<&'static str, button::State>,
    pub lang_one_states: HashMap<&'static str, button::State>,
    pub lang_two_states: HashMap<&'static str, button::State>,
    pub punctuation_states: HashMap<&'static str, button::State>,
    pub language_states: HashMap<String, button::State>,
    pub table_states: HashMap<String, button::State>,
    pub settings: SettingButtons,
}

#[derive(Clone, Debug, Default)]
pub struct SettingButtons {
    pub volume: slider::State,
    pub length: slider::State,
    pub h1: slider::State,
    pub h2: slider::State,
    pub h3: slider::State,
    pub h4: slider::State,
    pub body: slider::State,
    pub scrollable_state: scrollable::State,
    pub load_state: button::State,
}
impl Default for Mainstruct {
    fn default() -> Self {
        Self {
            letters: {
                let vec: Vec<String> = Vec::new();
                vec
            },
            langonevec: {
                let vec: Vec<String> = Vec::new();
                vec
            },
            langtwovec: {
                let vec: Vec<String> = Vec::new();
                vec
            },
            settings_bool: {
                let vec: Vec<bool> = Vec::new();
                vec
            },
            settings_usize: {
                let vec: Vec<usize> = Vec::new();
                vec
            },
            time: { Instant::now() },
            langone: { Vec::new() },
            langtwo: { Vec::new() },
            punctuation: { ["(", ")", ";", ":", ",", ".", "?", "!"] },
            shift: { false },
            colour: { 0 },
            maxstates: { 0 },
            word_index: { 0 },
            screen: { 0 },
            table: { 0 },
            lang: { 0 },
            button_states: {
                let mut map = HashMap::new();

                let list = [
                    "gotomain",
                    "gototesting",
                    "gotolang",
                    "gotosetting",
                    "gotodata",
                    "shift",
                    "next",
                    "submit",
                    "space",
                    "delete",
                    "save",
                ];
                for i in list {
                    map.insert(i, button::State::default());
                }
                map
            },
            settings_language: {
                let map: HashMap<&str, String> = HashMap::new();
                map
            },
            lang_one_states: {
                let map: HashMap<&str, button::State> = HashMap::new();
                map
            },
            lang_two_states: {
                let map: HashMap<&str, button::State> = HashMap::new();
                map
            },
            punctuation_states: {
                let map: HashMap<&str, button::State> = HashMap::new();
                map
            },
            language_states: { HashMap::new() },
            table_states: {
                let map: HashMap<String, button::State> = HashMap::new();
                map
            },
            settings: SettingButtons::default(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SettingData {
    pub metadata: Metadata,
    pub language: Language,
}
#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub author: String,
    pub description: String,
    pub last_update: String,
    pub email: String,
    pub link: String,
}
#[derive(Deserialize, Debug)]
pub struct Language {
    pub first: LangBase,
    pub second: LangBase,
}
#[derive(Deserialize, Debug)]
pub struct LangBase {
    pub alphabet: String,
}
