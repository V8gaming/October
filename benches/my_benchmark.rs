use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_derive::Deserialize;
use global::Global;
use std::fs;


static SETTINGS_USIZE: Global<Vec<usize>> = Global::new();
static SETTINGS_BOOL: Global<Vec<bool>> = Global::new();


#[derive(Deserialize, Debug)]
struct Data {
    settings: Settings
}

#[derive(Deserialize, Debug)]
struct TextsizeData {
    h1: usize,
    h2: usize,
    h3: usize,
    h4: usize,
    body: usize,
}


#[derive(Deserialize, Debug)]
struct SoundData {
    sound: bool,
    volume: usize,

}

#[derive(Deserialize, Debug)]
struct TimeData {
    timed: bool,
    length: usize,
}

#[derive(Deserialize, Debug)]
struct Settings {
    seperate_check_synonyms: bool,
    sound: SoundData,
    textsize: TextsizeData,
    time: TimeData,
}

fn loadsettings() {
    let filename = "./settings.toml";
    let contents = fs::read_to_string(filename).unwrap();

    SETTINGS_BOOL.lock_mut().unwrap().clear();
    SETTINGS_USIZE.lock_mut().unwrap().clear();
    let data: Data = toml::from_str(&contents).unwrap();
    let boollist = 
    [
        data.settings.seperate_check_synonyms,
        data.settings.sound.sound,
        data.settings.time.timed
    ];
    for i in boollist {
        SETTINGS_BOOL.lock_mut().unwrap().push(i);
    }
    let usizelist = 
    [
        data.settings.sound.volume,
        data.settings.time.length,
        data.settings.textsize.h1,
        data.settings.textsize.h2,
        data.settings.textsize.h3,
        data.settings.textsize.h4,
        data.settings.textsize.body,

    ];
    for i in usizelist {
        SETTINGS_USIZE.lock_mut().unwrap().push(i);
    }

}
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("loadsettings()", |b| b.iter(|| loadsettings()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);