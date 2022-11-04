use serde_derive::Deserialize;
use std::fs;
use toml;


#[derive(Deserialize, Debug)]
struct Data {
    settings: Settings
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct TextsizeData {
    h1: i32,
    h2: i32,
    h3: i32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct SoundData {
    sound: bool,
    volume: i32,

}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct TimeData {
    timed: bool,
    length: i32,
}

#[derive(Deserialize, Debug)]
struct Settings {
    seperate_check_synonyms: bool,
    sound: SoundData,
    textsize: TextsizeData,
    time: TimeData,
}

fn main() {
    let filename = "./settings.toml";
    let contents = fs::read_to_string(filename).unwrap();

    let data: Data = toml::from_str(&contents).unwrap();
    println!("seperate check synonyms: {}", data.settings.seperate_check_synonyms);
    println!("{:?}", data.settings.sound);
    println!("{:?}", data.settings.time);
    println!("{:?}", data.settings.textsize);

}