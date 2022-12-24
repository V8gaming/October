use crate::Data;
use crate::Mainstruct;
use crate::structs::SettingData;
use std::{fs, path::Path};
use iced::button;
use sqlite::State as SqlState;


pub fn read_languages_dir() -> Vec<Vec<String>> {
    let mut languages: Vec<Vec<String>> = Vec::new();
    let base_path = Path::new("./resources/languages");

    for dir in fs::read_dir(base_path).unwrap() {
        let dir_path = dir.unwrap().path();
        let mut variables: Vec<String> = Vec::new();
        variables.resize(2, "".to_string());
        //println!("{}",&dir_path.display());
        for file in fs::read_dir(dir_path).unwrap() { 
            let file_path = file.unwrap().path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            //println!("{}",file_name);

            if file_name.ends_with(".sqlite3") {
                variables.insert(0, file_path.display().to_string());
            } else if file_name.ends_with(".toml") {
                variables.insert(1, file_path.display().to_string());
            }
        }

        languages.push(variables);
    }

    languages
}

pub fn loadamount() -> usize {
    let languages = read_languages_dir();

    // Return the length of the list of languages
    languages.len()

}

pub fn loadhashmaps(selfx: &mut Mainstruct) {
    // Add characters from LANGONE to the map
    for i in selfx.langone {
        selfx.lang_one_states.insert(i, button::State::default());
    }
    
    // Add characters from LANGTWO to the map
    for i in selfx.langtwo {
        selfx.lang_one_states.insert(i, button::State::default());
        selfx.lang_two_states.insert(i, button::State::default());
    }
    
    // Add characters from PUNCTUATION to the map
    for i in selfx.punctuation {
        selfx.lang_one_states.insert(i, button::State::default());
        selfx.punctuation_states.insert(i, button::State::default());
    }
    let mut vec = Vec::new();
    for i in 0..selfx.maxstates {
        vec.push(format!("state{}",i));
    }
    for i in vec {
        selfx.table_states.insert(i, button::State::default());
    }
}
pub fn loaddata(selfx: &mut Mainstruct) {
    let languages = read_languages_dir();
    let connection = sqlite::open(format!("{}", languages[selfx.lang][0])).unwrap();
    
    let mut statement2 = connection
    .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
    .unwrap();
    let mut tables: Vec<String> = Vec::new();

    while let Ok(SqlState::Row) = statement2.next() {
        //println!("{}", statement2.read::<String>(0).unwrap());
        tables.push(statement2.read::<String>(0).unwrap())
    }
    let mut statement = connection
        .prepare(format!("SELECT * FROM {}", tables[selfx.table]))
        .unwrap();

    let mut vietnamese: Vec<String> = Vec::new();
    let mut english: Vec<String> = Vec::new();
    let mut typename: Vec<String> = Vec::new();

    while let Ok(SqlState::Row) = statement.next() {
        //println!("{} = {}", statement.read::<String>(0).unwrap(), statement.read::<String>(1).unwrap());
        //println!("{}",statement.read::<String>(0).unwrap());
        english.push(statement.read::<String>(0).unwrap());
        vietnamese.push(statement.read::<String>(1).unwrap());
        typename.push(statement.read::<String>(2).unwrap());
    }

    selfx.english.clear();
    for i in english {
        //println!("{}",i);

        selfx.english.push(i);
        
    }
    for i in vietnamese {
        selfx.vietnamese.push(i)
    }
}

pub fn loadsizes(selfx: &mut Mainstruct) {
    let languages = read_languages_dir();
    let mut templangvec = Vec::new();
    for i in languages {
        let connection = sqlite::open(format!("{}", i[0])).unwrap();
        let mut statement2 = connection
        .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
        .unwrap();
        let mut tables: Vec<String> = Vec::new();
    
        while let Ok(SqlState::Row) = statement2.next() {
            //println!("{}", statement2.read::<String>(0).unwrap());
            tables.push(statement2.read::<String>(0).unwrap())
        }
        templangvec.push(tables.len() as u32)
    }

    selfx.maxstates = *templangvec.iter().max().unwrap();
}

pub fn loadnames() -> Vec<String> {
    let mut languages: Vec<String> = Vec::new();
    let base_path = Path::new("./resources/languages");

    for dir in fs::read_dir(base_path).unwrap() {
        let dir_path = dir
            .unwrap()
            .path()
            .display()
            .to_string()
            .strip_prefix("./resources/languages\\")
            .unwrap()
            .to_string();
        languages.push
        (
            dir_path
        )
    }
    return languages;

}

pub fn loadlangsettings(selfx: &mut Mainstruct) {
    let languages = read_languages_dir();
    let setting = Path::new(&languages[selfx.lang][1]);
    
    let contents = fs::read_to_string(setting).unwrap();
    selfx.settings_language.clear();

    let data: SettingData = toml::from_str(&contents).unwrap();

    selfx.settings_language.insert("author", data.metadata.author);
    selfx.settings_language.insert("description", data.metadata.description);
    selfx.settings_language.insert("last update", data.metadata.last_update);
    selfx.settings_language.insert("email", data.metadata.email);
    selfx.settings_language.insert("link", data.metadata.link);
    selfx.settings_language.insert("lang one alphabet", data.language.first.alphabet);
    selfx.settings_language.insert("lang two alphabet", data.language.second.alphabet);
    
}
pub fn loadsettings(selfx: &mut Mainstruct) {
    let filename = "./settings.toml";
    let contents = fs::read_to_string(filename).unwrap();
    selfx.settings_bool.clear();
    selfx.settings_usize.clear();
    
    let data: Data = toml::from_str(&contents).unwrap();
    let boollist = 
    [
        data.settings.seperate_check_synonyms,
        data.settings.sound.sound,
        data.settings.time.timed
    ];
    for i in boollist {
        selfx.settings_bool.push(i);
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
        selfx.settings_usize.push(i);
    }
}
pub fn loadsize(lang: usize) -> usize {
    let languages = read_languages_dir();
    let connection = sqlite::open(format!("{}", languages[lang][0])).unwrap();
    
    let mut statement2 = connection
    .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
    .unwrap();
    let mut tables: Vec<String> = Vec::new();

    while let Ok(SqlState::Row) = statement2.next() {
        //println!("{}", statement2.read::<String>(0).unwrap());
        tables.push(statement2.read::<String>(0).unwrap())
    }
    return tables.len();
}

pub fn tablenames(lang: usize) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let languages = read_languages_dir();

    let connection = sqlite::open(format!("{}", languages[lang][0])).unwrap();
    let mut statement2 = connection
    .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
    .unwrap();

    while let Ok(SqlState::Row) = statement2.next() {
        //println!("{}", statement2.read::<String>(0).unwrap());
        names.push(statement2.read::<String>(0).unwrap())
    }
    return names;
    

}
