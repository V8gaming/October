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

    languages.sort_by(|a, b| a[0].cmp(&b[0]));
    languages
}

pub fn loadamount() -> usize {
    let languages = read_languages_dir();

/*     let mut x = 0;
    for i in &languages {
        println!("{} = {}", x, i[0]);
        x+=1;
    } */
    // Return the length of the list of languages
    languages.len()

}
pub fn loadalphabet(selfx: &mut Mainstruct){
    let english = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    let vietnamese = vec![
        "ẳ","á","â","à","ạ","ầ","ậ", "ấ","ả","ặ",
        "đ",
        "ỏ","ơ","ờ","ồ","ó","ô","ọ","ộ","ớ","ở",
        "ư","ụ","ữ","ú", "ủ",
        "í","ì","ị",
        "ế","ẹ", "ể", "ề"
    ];
    let greek = vec!["α", "β", "γ", "δ", "ε", "ζ", "η", "θ", "ι", "κ", "λ", "μ", "ν", "ξ", "ο", "π", "ρ", "σ", "ς", "ϲ", "τ", "υ", "φ", "χ", "ψ", "ω"];
    let cyrillic = vec!["а","б","в","г","д","е","ё","ж","з","и","й","к","л","м","н","о","п","р","с","т","у","ф","х","ц","ч","ш","щ","ъ","ы","ь","э","ю","я"];
    
    
    let alphabet: &str = selfx.settings_language.get("lang one alphabet").unwrap();
    match alphabet {
        "english" => selfx.langone = english.clone(),
        "vietnamese" => selfx.langone = vietnamese.clone(),
        "greek" => selfx.langone = greek.clone(),
        "cyrillic" => selfx.langone = cyrillic.clone(),
        _ => selfx.langone = english.clone(),
    }
    let alphabet: &str = selfx.settings_language.get("lang two alphabet").unwrap();
    match alphabet {
        "english" => selfx.langtwo = english,
        "vietnamese" => selfx.langtwo = vietnamese,
        "greek" => selfx.langtwo = greek,
        "cyrillic" => selfx.langone = cyrillic.clone(),
        _ => selfx.langtwo = english,
    }
    
}

pub fn loadhashmaps(selfx: &mut Mainstruct) {
    // Add characters from LANGONE to the map
    selfx.lang_one_states.clear();
    selfx.lang_two_states.clear();
    selfx.punctuation_states.clear();
    for i in selfx.langone.iter() {
        selfx.lang_one_states.insert(i, button::State::default());
    }
    
    // Add characters from LANGTWO to the map
    for i in selfx.langtwo.iter() {
        selfx.lang_two_states.insert(i, button::State::default());
    }
    
    // Add characters from PUNCTUATION to the map
    for i in selfx.punctuation {
        selfx.punctuation_states.insert(i, button::State::default());
    }
    let mut vec = Vec::new();
    for i in 0..selfx.maxstates {
        vec.push(format!("state{}",i));
    }
    for i in vec {
        selfx.table_states.insert(i, button::State::default());
    }

    let mut vec = Vec::new();
    for i in 0..loadamount() {
        vec.push(format!("state{}",i));
    }
    for i in vec {
        selfx.language_states.insert(i, button::State::default());
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

    let mut langtwo: Vec<String> = Vec::new();
    let mut langone: Vec<String> = Vec::new();
    let mut typename: Vec<String> = Vec::new();

    while let Ok(SqlState::Row) = statement.next() {
        //println!("{} = {}", statement.read::<String>(0).unwrap(), statement.read::<String>(1).unwrap());
        //println!("{}",statement.read::<String>(0).unwrap());
        langone.push(statement.read::<String>(0).unwrap());
        langtwo.push(statement.read::<String>(1).unwrap());
        typename.push(statement.read::<String>(2).unwrap());
    }

    selfx.english.clear();
    selfx.vietnamese.clear();
    for i in langone {
        //println!("{}",i);

        selfx.english.push(i);
        
    }
    for i in langtwo {
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
        #[cfg(target_family = "windows")]
        let dir_path = dir
            .unwrap()
            .path()
            .display()
            .to_string()
            .strip_prefix("./resources/languages\\")
            .unwrap()
            .to_string();
        #[cfg(target_family = "unix")]
        let dir_path = dir
            .unwrap()
            .path()
            .display()
            .to_string()
            .strip_prefix("./resources/languages/")
            .unwrap()
            .to_string();    
        languages.push
        (
            dir_path
        )
    }
    languages.sort_by(|a,b| a.cmp(b));
    return languages;

}

pub fn loadlangsettings(selfx: &mut Mainstruct) {
    let languages = read_languages_dir();
    let setting = Path::new(&languages[selfx.lang][1]);
    
    let contents = fs::read_to_string(setting).unwrap();
    selfx.settings_language.clear();
    //println!("{}", contents);
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
