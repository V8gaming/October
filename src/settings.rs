// settings.rs

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    pub settings: Settings
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
