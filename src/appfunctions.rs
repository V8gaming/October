use crate::{Mainstruct, load::{loadalphabet, loadlangsettings, loadhashmaps}};
use std::time::Instant;
use rand::{Rng, thread_rng};
use sqlite::State as SqlState;
use crate::load::{loaddata, read_languages_dir};

pub fn nextfn(selfx: &mut Mainstruct) {
    selfx.time = Instant::now();
    //println!("{}", ENGLISH.lock_mut().unwrap().len());
    selfx.word_index = thread_rng().gen_range(0..selfx.langonevec.len().try_into().unwrap());
    selfx.letters.clear();
    selfx.colour = 0;

}

pub fn index(selfx: &mut Mainstruct, num: usize) {
    let mut languages = read_languages_dir();
        // Sort the languages alphabetically
    languages.sort_by(|a, b| a.cmp(b));

/*     // Print the sorted languages
    for language in &languages {
        println!("{}", language);
    } */
    let connection = sqlite::open(
        format!(
            "{}", languages[selfx.lang][0]
        )
    ).unwrap();
    let mut statement2 = connection
    .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
    .unwrap();
    let mut tables: Vec<String> = Vec::new();
    while let Ok(SqlState::Row) = statement2.next() {
        tables.push(statement2.read::<String>(0).unwrap())
    }
    if num < tables.len() {
        selfx.screen = 1;
        selfx.table = num;
        loaddata(selfx);
        nextfn(selfx);
    }
}
pub fn changelang(selfx: &mut Mainstruct, num: usize) {
    selfx.lang = num;
    selfx.table = 0;
    loadlangsettings(selfx);
    loadalphabet(selfx);
    loadhashmaps(selfx);

    shiftscreenfn(selfx, 0);
    loaddata(selfx);
}

pub fn initalise(selfx: &mut Mainstruct) {
    selfx.lang = 0;
    selfx.table = 0;
    loadlangsettings(selfx);
    loadalphabet(selfx);
    loadhashmaps(selfx);
    loaddata(selfx);
}

pub fn shiftscreenfn(selfx: &mut Mainstruct,destination: usize) {
    selfx.time = Instant::now();
    selfx.screen = destination.try_into().unwrap();
    selfx.word_index = thread_rng().gen_range(0..selfx.langonevec.len().try_into().unwrap());
    selfx.letters.clear();
    selfx.colour = 0;
}

pub fn sumbitfn(selfx: &mut Mainstruct) {
    //println!("{:?}",LETTERS.lock_mut().unwrap().concat());
    if format!("{}", selfx.letters.concat()) == selfx.langtwovec[selfx.word_index]{
        //println!("true");
        selfx.colour = 2;
        selfx.screen = 2;
    } else {
        //println!("false");
        selfx.colour = 1;
        selfx.screen = 2;
    }
}