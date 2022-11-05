use std::io::Write;
use sqlite::State;
use std::fs;

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() {
    let mut languages2 = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        languages2.push(file.unwrap().path().display().to_string());

    }
    let mut lengths: Vec<usize> = Vec::new();
    for lang in 0..languages2.len() {
        let connection = sqlite::open(format!("{}", languages2[lang])).unwrap();
        let mut statement2 = connection
            .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
            .unwrap();
        
        let mut len: Vec<String> = Vec::new();
        while let Ok(State::Row) = statement2.next() {
            len.push(statement2.read::<String>(0).unwrap())
            //println!("{}", statement2.read::<String>(0).unwrap());
        }
        //println!("{}",len.len());
        lengths.push(len.len());
    }
    let max = lengths.iter().max().unwrap();
    let min = lengths.iter().min().unwrap();
    let connection = sqlite::open("./resources/languages/English-Vietnamese.sqlite3").unwrap();
    
    let mut tables: Vec<String> = Vec::new();

    let mut statement2 = connection
        .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
        .unwrap();

    while let Ok(State::Row) = statement2.next() {
        //println!("{}", statement2.read::<String>(0).unwrap());
        tables.push(statement2.read::<String>(0).unwrap())
    }

    let latinletters: [&str; 26] = ["a","b","c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    let vietnameseletters: [&str; 33] = [
    "ẳ","á","â","à","ạ","ầ","ậ", "ấ","ả","ặ",
    "đ",
    "ỏ","ơ","ờ","ồ","ó","ô","ọ","ộ","ớ","ở",
    "ư","ụ","ữ","ú", "ủ",
    "í","ì","ị",
    "ế","ẹ", "ể", "ề"
    ];
    let punctuation: [&str; 8] = ["(",")", ";", ":", ",", ".", "?", "!"];

    /* 
    for i in vietnameseletters {
        println!("{}", i.to_uppercase())
    }
    */

    let listlengths: usize = latinletters.len()+vietnameseletters.len()+punctuation.len();
    
    /* 
    let english = ["hers","yes","can","can not", "to do", "at", "with"];
    let vietnamese = ["của chị ấy","vâng","có thể","không thể","làm","ở","với"];
    */
    let mut languages: Vec<String> = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        //println!("{}", file.unwrap().path().display());
        languages.push(file.unwrap().path().display().to_string())
    }

    let customstates: [&str; 6] = ["shift","submit","space","delete","deleteall","next"];
    let customfunctions: [&str; 6] = ["shiftvaluefn()","sumbitfn()", "pushfn(String::from(\" \"))",  "popfn()","clearfn()", "nextfn()"];
    let mut custombuttons = Vec::new();
    for i in customstates {
        custombuttons.push(some_kind_of_uppercase_first_letter(i).to_owned())
    }
    
    let mut file = std::fs::File::create("./src/bin/main.rs").expect("create failed");
    
    file.write_all(format!("#![windows_subsystem = \"windows\"]\n").as_bytes()).expect("write failed");

    file.write_all(format!("use iced::{{alignment,Application,Command, Subscription, executor, button, Button, Element, Settings, Text, Container, Length, Column, Row, window, Color}};\n").as_bytes()).expect("write failed");
    file.write_all(format!("use iced::window::Position::Specific;\n").as_bytes()).expect("write failed");
    file.write_all(format!("use iced::window::Icon;\n").as_bytes()).expect("write failed");
    file.write_all(format!("use global::Global;\n").as_bytes()).expect("write failed");
    file.write_all(format!("use rand::{{thread_rng, Rng}};\n").as_bytes()).expect("write failed");
    file.write_all(format!("use sqlite::State;\n").as_bytes()).expect("write failed");
    file.write_all(format!("use iced_native::{{Event, keyboard}};\n").as_bytes()).expect("write failed");
    file.write_all(format!("use std::fs;\n").as_bytes()).expect("write failed");
    file.write_all(format!("use serde_derive::Deserialize;\n").as_bytes()).expect("write failed");
    file.write_all(format!("use toml;\n").as_bytes()).expect("write failed");
    
    

    // static LETTERS: Global<Vec<String>> = Global::new();
    file.write_all(format!("static LETTERS: Global<Vec<String>> = Global::new();\n").as_bytes()).expect("write failed");

    /*
    static ENGLISH: Global<Vec<&str>> = Global::new();
    static VIETNAMESE: Global<Vec<&str>> = Global::new();
    static N: Global<Vec<usize>> = Global::new();
    */

    file.write_all(format!("static ENGLISH: Global<Vec<String>> = Global::new();\n").as_bytes()).expect("write failed");
    file.write_all(format!("static VIETNAMESE: Global<Vec<String>> = Global::new();\n").as_bytes()).expect("write failed");
    file.write_all(format!("static N: Global<Vec<usize>> = Global::new();\n").as_bytes()).expect("write failed");
    file.write_all(format!("static COLOUR: Global<Vec<usize>> = Global::new();\n").as_bytes()).expect("write failed");
    file.write_all(format!("static X: Global<Vec<usize>> = Global::new();\n").as_bytes()).expect("write failed");
    file.write_all(format!("static TABLE: Global<Vec<usize>> = Global::new();\n").as_bytes()).expect("write failed");
    file.write_all(format!("static TEXTTYPE: Global<Vec<String>> = Global::new();\n").as_bytes()).expect("write failed");
    file.write_all(format!("static SCREEN: Global<Vec<usize>> = Global::new();\n").as_bytes()).expect("write failed");
    file.write_all(format!("static LANG: Global<Vec<usize>> = Global::new();\n").as_bytes()).expect("write failed");
    file.write_all(format!("static SETTINGS_USIZE: Global<Vec<usize>> = Global::new();\n").as_bytes()).expect("write failed");
    file.write_all(format!("static SETTINGS_BOOL: Global<Vec<bool>> = Global::new();\n").as_bytes()).expect("write failed");

    /*
    #[derive(Deserialize, Debug)]
    struct Data {
        settings: Settings
    }
    */
    file.write_all("#[derive(Deserialize, Debug)]\n".as_bytes()).expect("write failed");
    file.write_all("struct Data {\n".as_bytes()).expect("write failed");
    file.write_all("    settings: ImportSettings\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");

    file.write_all("#[derive(Deserialize, Debug)]\n".as_bytes()).expect("write failed");
    file.write_all("struct TextsizeData {\n".as_bytes()).expect("write failed");
    for i in ["h1: usize", "h2: usize", "h3: usize", "h4: usize", "body: usize",] {
        file.write_all(format!("    {},\n", i).as_bytes()).expect("write failed");
    }
    file.write_all("}\n".as_bytes()).expect("write failed");

    file.write_all("#[derive(Deserialize, Debug)]\n".as_bytes()).expect("write failed");
    file.write_all("struct SoundData {\n".as_bytes()).expect("write failed");
    for i in ["sound: bool,", "volume: usize,"] {
        file.write_all(format!("    {}\n", i).as_bytes()).expect("write failed");
    }
    file.write_all("}\n".as_bytes()).expect("write failed");

    file.write_all("#[derive(Deserialize, Debug)]\n".as_bytes()).expect("write failed");
    file.write_all("struct TimeData {\n".as_bytes()).expect("write failed");
    for i in ["timed: bool,", "length: usize,"] {
        file.write_all(format!("    {}\n", i).as_bytes()).expect("write failed");
    }
    file.write_all("}\n".as_bytes()).expect("write failed");

    file.write_all("#[derive(Deserialize, Debug)]\n".as_bytes()).expect("write failed");
    file.write_all("struct ImportSettings {\n".as_bytes()).expect("write failed");
    for i in ["seperate_check_synonyms: bool,", "sound: SoundData,", "textsize: TextsizeData,", "time: TimeData,"] {
        file.write_all(format!("    {}\n", i).as_bytes()).expect("write failed");
    }
    file.write_all("}\n".as_bytes()).expect("write failed");


    file.write_all("\n".as_bytes()).expect("write failed");
    file.write_all(format!("#[derive(Default, Clone)]\nstruct MyButton {{\n").as_bytes()).expect("write failed");
    
    file.write_all(format!("    gotomain_state: button::State,\n").as_bytes()).expect("write failed");
    file.write_all(format!("    gotolang_state: button::State,\n").as_bytes()).expect("write failed");

    file.write_all(format!("    resume_state: button::State,\n").as_bytes()).expect("write failed");

    for i in 0..*max {
        file.write_all(format!("    table{}_state: button::State,\n",i).as_bytes()).expect("write failed");
    }
    for i in customstates {
        file.write_all(format!("    {}_state: button::State,\n",i).as_bytes()).expect("write failed");
    }

    for i in 0..listlengths {
        file.write_all(format!("    button_state{}: button::State,\n", i).as_bytes()).expect("write failed");
    }
    for i in 0..languages.len() {
        file.write_all(format!("    lang_state{}: button::State,\n", i).as_bytes()).expect("write failed");
    }
    file.write_all(format!("}}\n").as_bytes()).expect("write failed");
    file.write_all("\n".as_bytes()).expect("write failed");
    file.write_all("#[derive(Debug, Clone)]".as_bytes()).expect("write failed");

    // emum Message
    file.write_all("enum Message {\n".as_bytes()).expect("write failed");
    file.write_all("    EventOccurred(iced_native::Event),\n".as_bytes()).expect("write failed");

    file.write_all("    GotoMainButton,\n".as_bytes()).expect("write failed");
    file.write_all("    GotoLangButton,\n".as_bytes()).expect("write failed");
    file.write_all("    ResumeButton,\n".as_bytes()).expect("write failed");

    for i in 0..*max {
        file.write_all(format!("    TableButton{},\n",i).as_bytes()).expect("write failed");
    }
    for i in &custombuttons {
        file.write_all(format!("    {}Button,\n",i).as_bytes()).expect("write failed");
    }
    for i in 0..listlengths {
        file.write_all(format!("    ButtonPressed{},\n", i).as_bytes()).expect("write failed");
    }
    for i in 0..languages.len() {
        file.write_all(format!("    LangButton{},\n", i).as_bytes()).expect("write failed");
    }
    file.write_all("}\n".as_bytes()).expect("write failed");

    /* 
    fn pushfn(letter: String) {
        LETTERS.lock_mut().unwrap().push(letter.to_string());
        println!("ADDED {} TO {}", letter,LETTERS.lock_mut().unwrap().concat());
        COLOUR.lock_mut().unwrap()[0] = 0
    }
    */
    file.write_all("fn pushfn(letter:String) {\n".as_bytes()).expect("write failed");
    file.write_all("    LETTERS.lock_mut().unwrap().push(shiftfn(letter.to_string()));\n".as_bytes()).expect("write failed");
    file.write_all("    println!(\"ADDED {} TO {}\", letter,LETTERS.lock_mut().unwrap().concat());\n".as_bytes()).expect("write failed");
    file.write_all("        COLOUR.lock_mut().unwrap()[0] = 0\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");
    /*
    fn shiftfn(letter: String) -> String {
        if X.lock_mut().unwrap()[0] == 1 {
            return letter.to_uppercase();
        } else {
            return letter.to_lowercase();
        }
    }

    */

    file.write_all("fn shiftscreenfn(destination: usize) {\n".as_bytes()).expect("write failed");
    file.write_all("   SCREEN.lock_mut().unwrap()[0] = destination;\n".as_bytes()).expect("write failed");
    file.write_all("   N.lock_mut().unwrap()[0] = thread_rng().gen_range(0..ENGLISH.lock_mut().unwrap().len());\n".as_bytes()).expect("write failed");
    file.write_all("   LETTERS.lock_mut().unwrap().clear();\n".as_bytes()).expect("write failed");
    file.write_all("   COLOUR.lock_mut().unwrap()[0] = 0;\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");

    file.write_all("fn shiftfn(letter: String) -> String {\n".as_bytes()).expect("write failed");
    file.write_all("    if X.lock_mut().unwrap()[0] == 1 {\n".as_bytes()).expect("write failed");
    file.write_all("        return letter.to_uppercase();\n".as_bytes()).expect("write failed");
    file.write_all("    } else {\n".as_bytes()).expect("write failed");
    file.write_all("        return letter.to_lowercase();\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");
    
    /*
    fn shiftvaluefn() {
        if X.lock_mut().unwrap()[0] == 1 {
            X.lock_mut().unwrap()[0] = 0;
        } else {
            X.lock_mut().unwrap()[0] = 1;
        }
    }
    */

    file.write_all("fn shiftvaluefn() {\n".as_bytes()).expect("write failed");
    file.write_all("    if X.lock_mut().unwrap()[0] == 1 {\n".as_bytes()).expect("write failed");
    file.write_all("        X.lock_mut().unwrap()[0] = 0;\n".as_bytes()).expect("write failed");
    file.write_all("    } else {\n".as_bytes()).expect("write failed");
    file.write_all("        X.lock_mut().unwrap()[0] = 1;\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");
    

    /* 
    fn sumbitfn() {
        println!("{:?}",LETTERS.lock_mut().unwrap().concat());
        let vietnamese = ["của chị ấy","vâng","có thể","không thể",];
        for i in vietnamese {
            VIETNAMESE.lock_mut().unwrap().push(i)
        }
        
        if format!("{}", LETTERS.lock_mut().unwrap().concat()) == VIETNAMESE.lock_mut().unwrap()[N.lock_mut().unwrap()[0]]{
            println!("true")
        } else {
            println!("false")
        }
    }
    */
    file.write_all("fn sumbitfn() {\n".as_bytes()).expect("write failed");
    file.write_all("    if format!(\"{}\", LETTERS.lock_mut().unwrap().concat()) == VIETNAMESE.lock_mut().unwrap()[N.lock_mut().unwrap()[0]]{\n".as_bytes()).expect("write failed");
    file.write_all("        COLOUR.lock_mut().unwrap()[0] = 2;\n".as_bytes()).expect("write failed");
    file.write_all("        SCREEN.lock_mut().unwrap()[0] = 2;\n".as_bytes()).expect("write failed");

    //file.write_all("        println!(\"true\")\n".as_bytes()).expect("write failed");
    file.write_all("    } else {\n".as_bytes()).expect("write failed");
    file.write_all("        COLOUR.lock_mut().unwrap()[0] = 1;\n".as_bytes()).expect("write failed");
    //file.write_all("        println!(\"false\")\n".as_bytes()).expect("write failed");
    file.write_all("        SCREEN.lock_mut().unwrap()[0] = 2;\n".as_bytes()).expect("write failed");

    file.write_all("    }\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");

    /* 
    fn popfn() {
        if LETTERS.lock_mut().unwrap().len() != 0 {
            LETTERS.lock_mut().unwrap().pop();
            println!("{}",LETTERS.lock_mut().unwrap().concat());
            COLOUR.lock_mut().unwrap()[0] = 0
        } 
    }
    */
    file.write_all("fn popfn() {\n".as_bytes()).expect("write failed");
    file.write_all("    if LETTERS.lock_mut().unwrap().len() != 0 {\n".as_bytes()).expect("write failed");
    file.write_all("        LETTERS.lock_mut().unwrap().pop();\n".as_bytes()).expect("write failed");
    file.write_all("        println!(\"{}\",LETTERS.lock_mut().unwrap().concat());\n".as_bytes()).expect("write failed");
    file.write_all("        COLOUR.lock_mut().unwrap()[0] = 0\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");


    file.write_all("fn clearfn() {\n".as_bytes()).expect("write failed");
    file.write_all("    if LETTERS.lock_mut().unwrap().len() != 0 {\n".as_bytes()).expect("write failed");
    file.write_all("        LETTERS.lock_mut().unwrap().clear();\n".as_bytes()).expect("write failed");
    file.write_all("        println!(\"{}\",LETTERS.lock_mut().unwrap().concat());\n".as_bytes()).expect("write failed");
    file.write_all("        COLOUR.lock_mut().unwrap()[0] = 0\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");

    /*
    fn index() {
        if TABLE.lock_mut().unwrap()[0] == 1{
            TABLE.lock_mut().unwrap()[0] = 0
        } else if TABLE.lock_mut().unwrap()[0] == 0 {
            TABLE.lock_mut().unwrap()[0] = 1
        } else {
            TABLE.lock_mut().unwrap()[0] = 0
        }
        loaddata();
        nextfn();
    }
    */
    file.write_all("fn add_button<'a>(a: &'a mut button::State,b: String,c: Message) -> Button<'a, Message> {\n".as_bytes()).expect("write failed");
    file.write_all("    return Button::new(a, body(format!(\"{}\",b))).on_press(c);\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");
    /*
        let connection = sqlite::open("./resources/languages/English-Vietnamese.sqlite3").unwrap();
        
        let mut tables: Vec<String> = Vec::new();

        let mut statement2 = connection
            .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
            .unwrap();

        while let Ok(State::Row) = statement2.next() {
            tables.push(statement2.read::<String>(0).unwrap())
        }

    */

    let mut languages1 = Vec::new();
    for file in fs::read_dir("./resources/languages/").unwrap() {
        languages1.push(file.unwrap().path().display().to_string());

    }

    file.write_all("fn loadtables<'a>(".as_bytes()).expect("write failed");
    for i in 0..*max {
        file.write_all(format!("self{}: &'a mut button::State,", i).as_bytes()).expect("write failed");
    }
    file.write_all(format!(") -> Vec<Button<'a, Message>> {{\n", ).as_bytes()).expect("write failed");
    file.write_all(format!("    if LANG.lock_mut().unwrap()[0] == 0 {{\n").as_bytes()).expect("write failed");
    file.write_all(format!("        return vec![\n").as_bytes()).expect("write failed");

    let connection = sqlite::open(format!("{}", languages1[0])).unwrap();
    let mut statement2 = connection
        .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
        .unwrap();
    let mut x1 = 0;
    while let Ok(State::Row) = statement2.next() {
        //println!("{}", statement2.read::<String>(0).unwrap());
        file.write_all(format!("            add_button(self{}, String::from(\"{}\"), Message::TableButton{}),\n", x1, some_kind_of_uppercase_first_letter(&String::from(statement2.read::<String>(0).unwrap())), x1).as_bytes()).expect("write failed");
        x1 +=1
    }
    file.write_all(format!("            ];\n").as_bytes()).expect("write failed");

    for lang in 1..languages1.len() {
        //if LANG.lock_mut().unwrap()[0] == 0 {
        file.write_all(format!("    }} else if LANG.lock_mut().unwrap()[0] == {} {{\n", lang).as_bytes()).expect("write failed");
        file.write_all(format!("        return vec![\n").as_bytes()).expect("write failed");

        let connection = sqlite::open(format!("{}", languages1[lang])).unwrap();
        let mut statement2 = connection
            .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'" )
            .unwrap();
            let mut x1 = 0;
        while let Ok(State::Row) = statement2.next() {
            //println!("{}", statement2.read::<String>(0).unwrap());
            file.write_all(format!("            add_button(self{}, String::from(\"{}\"), Message::TableButton{}),\n", x1, some_kind_of_uppercase_first_letter(&String::from(statement2.read::<String>(0).unwrap())), x1).as_bytes()).expect("write failed");
            x1 +=1
        }
        file.write_all(format!("            ];\n").as_bytes()).expect("write failed");

    }
    file.write_all(format!("    }} else  {{\n").as_bytes()).expect("write failed");
    file.write_all(format!("        return vec![\n").as_bytes()).expect("write failed");

    for x1 in 0..*min {
        //println!("{}", statement2.read::<String>(0).unwrap());
        file.write_all(format!("            add_button(self{}, String::from(\"Level {}\"), Message::TableButton{}),\n", x1, x1, x1).as_bytes()).expect("write failed");
    }
    file.write_all(format!("            ];\n").as_bytes()).expect("write failed");


    file.write_all("    }\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");

    file.write_all("fn index(num: usize) {\n".as_bytes()).expect("write failed");
    
    file.write_all("    let mut languages: Vec<String> = Vec::new();\n".as_bytes()).expect("write failed");
    file.write_all("    for file in fs::read_dir(\"./resources/languages/\").unwrap() {\n".as_bytes()).expect("write failed");
    file.write_all("        languages.push(file.unwrap().path().display().to_string())\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");

    file.write_all("    let connection = sqlite::open(format!(\"{}\", languages[LANG.lock_mut().unwrap()[0]])).unwrap();\n".as_bytes()).expect("write failed");
    
    file.write_all("    let mut statement2 = connection\n".as_bytes()).expect("write failed");
    file.write_all("    .prepare(\"SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'\" )\n    .unwrap();\n".as_bytes()).expect("write failed");
    file.write_all("    let mut tables: Vec<String> = Vec::new();\n".as_bytes()).expect("write failed");
    file.write_all("    while let Ok(State::Row) = statement2.next() {\n".as_bytes()).expect("write failed");
    file.write_all("        tables.push(statement2.read::<String>(0).unwrap())\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");
    file.write_all("    if num < tables.len() {\n".as_bytes()).expect("write failed");
    file.write_all("        SCREEN.lock_mut().unwrap()[0] = 1;\n".as_bytes()).expect("write failed");
    file.write_all("        TABLE.lock_mut().unwrap()[0] = num;\n".as_bytes()).expect("write failed");
    file.write_all("        loaddata();\n".as_bytes()).expect("write failed");
    file.write_all("        nextfn();\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");


    file.write_all("}\n".as_bytes()).expect("write failed");
    
    file.write_all("fn makemain(selfx: &mut MyButton) -> Element<Message>{\n".as_bytes()).expect("write failed");

    file.write_all("    let mut languages: Vec<String> = Vec::new();\n".as_bytes()).expect("write failed");
    file.write_all("    for file in fs::read_dir(\"./resources/languages/\").unwrap() {\n".as_bytes()).expect("write failed");
    file.write_all("        languages.push(file.unwrap().path().display().to_string())\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");

    file.write_all("    let title = h1(String::from(\"October\")).height(Length::FillPortion(1)).horizontal_alignment(alignment::Horizontal::Center).width(Length::Fill);\n".as_bytes()).expect("write failed");

    file.write_all("    let curlang = h3(format!(\"Language: {}\", languages[LANG.lock_mut().unwrap()[0]].strip_suffix(\".sqlite3\").unwrap().strip_prefix(\"./resources/languages/\").unwrap())).height(Length::FillPortion(1));\n".as_bytes()).expect("write failed");
    file.write_all("    let langs = add_button(&mut selfx.gotolang_state, String::from(\"Languages\"), Message::GotoLangButton);\n".as_bytes()).expect("write failed");
    file.write_all(format!("    let buttons = loadtables(").as_bytes()).expect("write failed");
    for i in 0..*max {
        file.write_all(format!("&mut selfx.table{}_state,", i).as_bytes()).expect("write failed");
    }
    file.write_all(format!(");\n").as_bytes()).expect("write failed");

    file.write_all("    let mut maincolumn = Column::new().push(title).push(curlang).push(langs).align_items(alignment::Alignment::Center).width(Length::Fill);\n".as_bytes()).expect("write failed");
    file.write_all("    for i in buttons  {\n".as_bytes()).expect("write failed");
    file.write_all("        maincolumn = maincolumn.push(i);\n".as_bytes()).expect("write failed");
    file.write_all("    };\n".as_bytes()).expect("write failed");


    file.write_all("    let main: Element<Message> = Container::new(maincolumn)\n".as_bytes()).expect("write failed");
    file.write_all("        .padding(100)\n".as_bytes()).expect("write failed");
    file.write_all("        .width(Length::Fill)\n".as_bytes()).expect("write failed");
    file.write_all("        .height(Length::Fill)\n".as_bytes()).expect("write failed");
    file.write_all("        .center_x()\n".as_bytes()).expect("write failed");
    file.write_all("        .center_y()\n".as_bytes()).expect("write failed");
    file.write_all("        .into();\n".as_bytes()).expect("write failed");
    file.write_all("    return main;\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");


    file.write_all("fn makelang(selfx: &mut MyButton) -> Element<Message>{\n".as_bytes()).expect("write failed");
    file.write_all("    TABLE.lock_mut().unwrap()[0] = 0;\n".as_bytes()).expect("write failed");
    file.write_all("    let langcolumn = Column::new()".as_bytes()).expect("write failed");
    for i in 0..languages.len() {
        file.write_all(format!(".push(add_button(&mut selfx.lang_state{}, String::from(\"{}\"), Message::LangButton{}))", i, languages[i].strip_prefix("./resources/languages/").unwrap().strip_suffix(".sqlite3").unwrap(), i).as_bytes()).expect("write failed");
    }
    file.write_all("    ;\n".as_bytes()).expect("write failed");

    file.write_all("    let main: Element<Message> = Container::new(langcolumn)\n".as_bytes()).expect("write failed");
    file.write_all("        .padding(100)\n".as_bytes()).expect("write failed");
    file.write_all("        .width(Length::Fill)\n".as_bytes()).expect("write failed");
    file.write_all("        .height(Length::Fill)\n".as_bytes()).expect("write failed");
    file.write_all("        .center_x()\n".as_bytes()).expect("write failed");
    file.write_all("        .center_y()\n".as_bytes()).expect("write failed");
    file.write_all("        .into();\n".as_bytes()).expect("write failed");
    file.write_all("    return main;\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");


    file.write_all("fn makereview(selfx: &mut MyButton) -> Element<Message>{\n".as_bytes()).expect("write failed");
    file.write_all("    let exit = add_button(&mut selfx.gotomain_state, String::from(\"Exit\"), Message::GotoMainButton);\n".as_bytes()).expect("write failed");
    file.write_all("    let colours = vec![Color::BLACK,Color::from_rgb(1.0, 0.0, 0.0),Color::from_rgb(0.0, 1.0, 0.0)];\n".as_bytes()).expect("write failed");
    file.write_all("    let subtitle1 = h3(String::from(\"Your answer\")).color(colours[COLOUR.lock_mut().unwrap()[0]]).horizontal_alignment(alignment::Horizontal::Center).width(Length::Fill);\n".as_bytes()).expect("write failed");
    file.write_all("    let subtitle2 = h3(String::from(\"Vietnamese\")).horizontal_alignment(alignment::Horizontal::Center).width(Length::Fill);\n".as_bytes()).expect("write failed");
    file.write_all("    let subtitle3 = h3(String::from(\"English\")).horizontal_alignment(alignment::Horizontal::Center).width(Length::Fill);\n".as_bytes()).expect("write failed");
    file.write_all("    let youranswer = h4(format!(\"{}\", LETTERS.lock_mut().unwrap().concat())).height(Length::Units(80)).color(colours[COLOUR.lock_mut().unwrap()[0]]);\n".as_bytes()).expect("write failed");
    file.write_all("    let english = h4(format!(\"{}\",ENGLISH.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Units(80));\n".as_bytes()).expect("write failed");
    file.write_all("    let vietnamese = h4(format!(\"{}\",VIETNAMESE.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Units(80));\n".as_bytes()).expect("write failed");
    file.write_all("    let resume = add_button(&mut selfx.resume_state, String::from(\"Resume\"), Message::ResumeButton);\n".as_bytes()).expect("write failed");
    file.write_all("    let column = Column::new().push(exit).push(subtitle1).push(youranswer).push(subtitle2).push(vietnamese).push(subtitle3).push(english).push(resume);\n".as_bytes()).expect("write failed");


    file.write_all("    let main: Element<Message> = Container::new(column)\n".as_bytes()).expect("write failed");
    file.write_all("        .padding(100)\n".as_bytes()).expect("write failed");
    file.write_all("        .width(Length::Fill)\n".as_bytes()).expect("write failed");
    file.write_all("        .height(Length::Fill)\n".as_bytes()).expect("write failed");
    file.write_all("        .center_x()\n".as_bytes()).expect("write failed");
    file.write_all("        .into();\n".as_bytes()).expect("write failed");
    file.write_all("    return main;\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");


    file.write_all(" fn makelevel(selfx: &mut MyButton) -> Element<Message>{\n".as_bytes()).expect("write failed");
    file.write_all("    let english = h2(format!(\"{}\",ENGLISH.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Units(80));\n".as_bytes()).expect("write failed");
    file.write_all("    let colours = vec![Color::BLACK,Color::from_rgb(1.0, 0.0, 0.0),Color::from_rgb(0.0, 1.0, 0.0)];\n".as_bytes()).expect("write failed");
    file.write_all("    let text1 = h2(format!(\"{}\", LETTERS.lock_mut().unwrap().concat())).height(Length::Units(80)).color(colours[COLOUR.lock_mut().unwrap()[0]]);\n".as_bytes()).expect("write failed");
    file.write_all("    let texttype = h4(format!(\"{}\",TEXTTYPE.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Fill).size(40);\n".as_bytes()).expect("write failed");

    // button1
    file.write_all("    let buttons1 = [\n".as_bytes()).expect("write failed");
    for i in 0..latinletters.len() {
        file.write_all(format!("        add_button(&mut selfx.button_state{}, shiftfn(String::from(\"{}\")), Message::ButtonPressed{}),\n",i,latinletters[i], i).as_bytes()).expect("write failed");
    }
    file.write_all("    ];\n".as_bytes()).expect("write failed");

    // button2
    file.write_all("    let buttons2 = [\n".as_bytes()).expect("write failed");
    for i in 0..vietnameseletters.len() {
        file.write_all(format!("        add_button(&mut selfx.button_state{}, shiftfn(String::from(\"{}\")), Message::ButtonPressed{}),\n",i+latinletters.len(),vietnameseletters[i], i+latinletters.len()).as_bytes()).expect("write failed");
    }
    file.write_all("    ];\n".as_bytes()).expect("write failed");

    file.write_all("    let buttons3 = [\n".as_bytes()).expect("write failed");
    for i in 0..punctuation.len() {
        file.write_all(format!("        add_button(&mut selfx.button_state{}, shiftfn(String::from(\"{}\")), Message::ButtonPressed{}),\n",i+latinletters.len()+vietnameseletters.len(),punctuation[i], i+latinletters.len()+vietnameseletters.len()).as_bytes()).expect("write failed");
    }
    file.write_all("    ];\n".as_bytes()).expect("write failed");
    for i in 0..customstates.len() {
        file.write_all(format!("    let {} = add_button(&mut selfx.{}_state, String::from(\"{}\"), Message::{}Button);\n", customstates[i], customstates[i],customstates[i], custombuttons[i]).as_bytes()).expect("write failed");
    }

    // userrow
    file.write_all("    let mut userrow = Row::new();\n".as_bytes()).expect("write failed");
    for i in customstates {
        file.write_all(format!("    userrow = userrow.push({});\n", i).as_bytes()).expect("write failed");
    }
    // row1
    file.write_all("    let mut row1 = Row::new();\n".as_bytes()).expect("write failed");
    file.write_all("    for button in buttons1 {\n".as_bytes()).expect("write failed");
    file.write_all("        row1 = row1.push(button);\n".as_bytes()).expect("write failed");
    file.write_all("    };\n".as_bytes()).expect("write failed");

    // row2
    file.write_all("    let mut row2 = Row::new();\n".as_bytes()).expect("write failed");
    file.write_all("    for button in buttons2 {\n".as_bytes()).expect("write failed");
    file.write_all("        row2 = row2.push(button);\n".as_bytes()).expect("write failed");
    file.write_all("    };\n".as_bytes()).expect("write failed");

    // row3
    file.write_all("    let mut row3 = Row::new();\n".as_bytes()).expect("write failed");
    file.write_all("    for button in buttons3 {\n".as_bytes()).expect("write failed");
    file.write_all("        row3 = row3.push(button);\n".as_bytes()).expect("write failed");
    file.write_all("    };\n".as_bytes()).expect("write failed");
    file.write_all("    let exit = add_button(&mut selfx.gotomain_state, String::from(\"Exit\"), Message::GotoMainButton);\n".as_bytes()).expect("write failed");

    file.write_all("    let utilrow = Row::new().push(exit);\n".as_bytes()).expect("write failed");

    
    file.write_all("    let column1 = Column::new().push(utilrow.width(Length::Fill)).push(texttype).push(english).push(text1).push(userrow).push(row1).push(row2).push(row3).width(Length::Fill).align_items(iced::Alignment::Center);\n".as_bytes()).expect("write failed");
    file.write_all("    Container::new(column1)\n".as_bytes()).expect("write failed");
    file.write_all("    .padding(100)\n    .width(Length::Fill)\n    .height(Length::Fill)\n    .center_x()\n    .center_y()\n    .into()".as_bytes()).expect("write failed");
    

    file.write_all("}\n".as_bytes()).expect("write failed");
    
    /*
    fn nextfn() {
        N.lock_mut().unwrap()[0] = thread_rng().gen_range(0..4);
        LETTERS.lock_mut().unwrap().clear();
        COLOUR.lock_mut().unwrap()[0] = 0
    }
    */
    file.write_all("fn nextfn() {\n".as_bytes()).expect("write failed");
    file.write_all("    N.lock_mut().unwrap()[0] = thread_rng().gen_range(0..ENGLISH.lock_mut().unwrap().len());\n".as_bytes()).expect("write failed");
    file.write_all("    LETTERS.lock_mut().unwrap().clear();\n".as_bytes()).expect("write failed");
    file.write_all("    COLOUR.lock_mut().unwrap()[0] = 0\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");

    file.write_all("fn changelang(num: usize) {\n".as_bytes()).expect("write failed");
    file.write_all("    LANG.lock_mut().unwrap()[0] = num;\n".as_bytes()).expect("write failed");
    file.write_all("    shiftscreenfn(0);\n".as_bytes()).expect("write failed");
    file.write_all("    loaddata();\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");



    file.write_all("fn loaddata() {\n".as_bytes()).expect("write failed");
    
    file.write_all("    let mut languages: Vec<String> = Vec::new();\n".as_bytes()).expect("write failed");
    file.write_all("    for file in fs::read_dir(\"./resources/languages/\").unwrap() {\n".as_bytes()).expect("write failed");
    file.write_all("        languages.push(file.unwrap().path().display().to_string())\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");

    file.write_all("    let connection = sqlite::open(format!(\"{}\", languages[LANG.lock_mut().unwrap()[0]])).unwrap();\n".as_bytes()).expect("write failed");
    
    file.write_all("    let mut statement2 = connection\n".as_bytes()).expect("write failed");
    file.write_all("    .prepare(\"SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%'\" )\n    .unwrap();\n".as_bytes()).expect("write failed");
    file.write_all("    let mut tables: Vec<String> = Vec::new();\n".as_bytes()).expect("write failed");
    file.write_all("    while let Ok(State::Row) = statement2.next() {\n".as_bytes()).expect("write failed");
    file.write_all("        tables.push(statement2.read::<String>(0).unwrap())\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");
    file.write_all("    let mut statement = connection\n".as_bytes()).expect("write failed");
    file.write_all("        .prepare(format!(\"SELECT * FROM {}\", tables[TABLE.lock_mut().unwrap()[0]]))\n".as_bytes()).expect("write failed");
    file.write_all("        .unwrap();\n".as_bytes()).expect("write failed");
    file.write_all("    let mut vietnamese: Vec<String> = Vec::new();\n".as_bytes()).expect("write failed");
    file.write_all("    let mut english: Vec<String> = Vec::new();\n".as_bytes()).expect("write failed");
    file.write_all("    let mut typename: Vec<String> = Vec::new();\n".as_bytes()).expect("write failed");
    file.write_all("    while let Ok(State::Row) = statement.next() {\n".as_bytes()).expect("write failed");
    file.write_all("        english.push(statement.read::<String>(0).unwrap());\n".as_bytes()).expect("write failed");
    file.write_all("        vietnamese.push(statement.read::<String>(1).unwrap());\n".as_bytes()).expect("write failed");
    file.write_all("        typename.push(statement.read::<String>(2).unwrap());\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");
    file.write_all("    ENGLISH.lock_mut().unwrap().clear();\n".as_bytes()).expect("write failed");
    file.write_all("    VIETNAMESE.lock_mut().unwrap().clear();\n".as_bytes()).expect("write failed");
    file.write_all("    TEXTTYPE.lock_mut().unwrap().clear();\n".as_bytes()).expect("write failed");
    file.write_all("    for i in english {\n".as_bytes()).expect("write failed");
    file.write_all("        ENGLISH.lock_mut().unwrap().push(i);\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");
    file.write_all("    for i in vietnamese {\n".as_bytes()).expect("write failed");
    file.write_all("        VIETNAMESE.lock_mut().unwrap().push(i);\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");
    file.write_all("    for i in typename {\n".as_bytes()).expect("write failed");
    file.write_all("        TEXTTYPE.lock_mut().unwrap().push(i);\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");
    // impl Sandbox for MyButton
    file.write_all("impl Application for MyButton {\n".as_bytes()).expect("write failed");
    file.write_all("    type Message = Message;\n".as_bytes()).expect("write failed");
    file.write_all("    type Executor = executor::Default;\n".as_bytes()).expect("write failed");
    file.write_all("    type Flags = ();\n".as_bytes()).expect("write failed");
    
    file.write_all("    fn subscription(&self) -> Subscription<Message> {\n".as_bytes()).expect("write failed");
    file.write_all("        iced_native::subscription::events().map(Message::EventOccurred)\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");

    file.write_all("    fn new(_flags: ()) -> (MyButton, Command<Message>) {\n".as_bytes()).expect("write failed");
    file.write_all("        (MyButton::default(), Command::none())\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");
    // fn title
    file.write_all("\n  fn title(&self) -> String {\n        String::from(\"October\")\n   }".as_bytes()).expect("write failed");


    // fn update
    file.write_all("\n  fn update(&mut self, message: Message) -> Command<Message> {\n        match message {\n".as_bytes()).expect("write failed");
    file.write_all(format!("        Message::GotoMainButton => shiftscreenfn(0),\n").as_bytes()).expect("write failed");
    file.write_all(format!("        Message::GotoLangButton => shiftscreenfn(3),\n").as_bytes()).expect("write failed");
    file.write_all(format!("        Message::ResumeButton => shiftscreenfn(1),\n").as_bytes()).expect("write failed");

    for i in 0..*max {
        file.write_all(format!("        Message::TableButton{} => index({}),\n", i, i).as_bytes()).expect("write failed");
    }
    for i in 0..customstates.len() {
        file.write_all(format!("        Message::{}Button => {},\n", custombuttons[i], customfunctions[i]).as_bytes()).expect("write failed");
    }

    for i in 0..listlengths {
        if i < latinletters.len(){
            file.write_all(format!("        Message::ButtonPressed{} => pushfn(String::from(\"{}\")),\n", i, latinletters[i]).as_bytes()).expect("write failed");
        } else if i >= latinletters.len() && i < latinletters.len() + vietnameseletters.len() {
            file.write_all(format!("        Message::ButtonPressed{} => pushfn(String::from(\"{}\")),\n", i, vietnameseletters[i-latinletters.len()]).as_bytes()).expect("write failed");
        } else if i >= latinletters.len() + vietnameseletters.len() {
            file.write_all(format!("        Message::ButtonPressed{} => pushfn(String::from(\"{}\")),\n", i, punctuation[i-(latinletters.len()+vietnameseletters.len())]).as_bytes()).expect("write failed");
        } 
    }
    for i in 0..languages.len() {
        file.write_all(format!("        Message::LangButton{} => changelang({}),\n", i,i).as_bytes()).expect("write failed");

        
    }
    file.write_all(format!("        Message::EventOccurred(event) => {{\n").as_bytes()).expect("write failed");
    file.write_all("        if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Space, modifiers: _ }) = event {\n".as_bytes()).expect("write failed");
    file.write_all("            pushfn(String::from(\" \"))\n".as_bytes()).expect("write failed");
    file.write_all("        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::LShift, modifiers: _ }) = event {\n".as_bytes()).expect("write failed");
    file.write_all("            shiftvaluefn()\n".as_bytes()).expect("write failed");
    file.write_all("        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Right, modifiers: _ }) = event {\n".as_bytes()).expect("write failed");
    file.write_all("            nextfn()\n".as_bytes()).expect("write failed");
    file.write_all("        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Enter, modifiers: _ }) = event {\n".as_bytes()).expect("write failed");
    file.write_all("            sumbitfn()\n".as_bytes()).expect("write failed");
    file.write_all("        } else if let Event::Keyboard(keyboard::Event::KeyReleased { key_code: keyboard::KeyCode::Backspace, modifiers: _ }) = event {\n".as_bytes()).expect("write failed");
    file.write_all("            popfn()\n".as_bytes()).expect("write failed");

    for i in latinletters {
        file.write_all(format!("        }} else if let Event::Keyboard(keyboard::Event::KeyReleased {{ key_code: keyboard::KeyCode::{}, modifiers: _ }}) = event {{\n", i.to_uppercase()).as_bytes()).expect("write failed");
        file.write_all(format!("            pushfn(String::from(\"{}\"))\n", i).as_bytes()).expect("write failed");
    }

    file.write_all("      }\n".as_bytes()).expect("write failed");
    


    file.write_all("        },\n".as_bytes()).expect("write failed");
    file.write_all("    };\n".as_bytes()).expect("write failed");
    file.write_all("    Command::none()\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");

    // fn view
    file.write_all("\n  fn view(&mut self) -> Element<Message> {".as_bytes()).expect("write failed");
    file.write_all("\n      if SCREEN.lock_mut().unwrap()[0] == 0 {".as_bytes()).expect("write failed");
    file.write_all("\n          return makemain(self);".as_bytes()).expect("write failed");
    file.write_all("\n      } else if SCREEN.lock_mut().unwrap()[0] == 1 {".as_bytes()).expect("write failed");
    file.write_all("\n          return makelevel(self);".as_bytes()).expect("write failed");
    file.write_all("\n      } else if SCREEN.lock_mut().unwrap()[0] == 2 {".as_bytes()).expect("write failed");
    file.write_all("\n          return makereview(self);".as_bytes()).expect("write failed");
    file.write_all("\n      } else if SCREEN.lock_mut().unwrap()[0] == 3 {".as_bytes()).expect("write failed");
    file.write_all("\n          return makelang(self);".as_bytes()).expect("write failed");

    file.write_all("\n      } else {".as_bytes()).expect("write failed");
    file.write_all("\n          return makemain(self); ".as_bytes()).expect("write failed");

    file.write_all("\n      } ".as_bytes()).expect("write failed");
    file.write_all("\n    }\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");


    file.write_all("fn loadsettings() {\n".as_bytes()).expect("write failed");
    file.write_all("    SETTINGS_BOOL.lock_mut().unwrap().clear();\n".as_bytes()).expect("write failed");
    file.write_all("    SETTINGS_USIZE.lock_mut().unwrap().clear();\n".as_bytes()).expect("write failed");
    file.write_all("    let filename = \"./settings.toml\";\n".as_bytes()).expect("write failed");
    file.write_all("    let contents = fs::read_to_string(filename).unwrap();\n".as_bytes()).expect("write failed");

    file.write_all("    let data: Data = toml::from_str(&contents).unwrap();\n".as_bytes()).expect("write failed");
    file.write_all("    let boollist = \n".as_bytes()).expect("write failed");
    file.write_all("    [ \n".as_bytes()).expect("write failed");
    for i in ["seperate_check_synonyms","sound.sound", "time.timed"] {
        file.write_all(format!("        data.settings.{},\n", i).as_bytes()).expect("write failed");
    }
    file.write_all("    ]; \n".as_bytes()).expect("write failed");
    file.write_all("    for i in boollist {\n".as_bytes()).expect("write failed");
    file.write_all("        SETTINGS_BOOL.lock_mut().unwrap().push(i);\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");

    file.write_all("    let usizelist = \n".as_bytes()).expect("write failed");
    file.write_all("    [ \n".as_bytes()).expect("write failed");
    for i in ["sound.volume","time.length", "textsize.h1", "textsize.h2", "textsize.h3", "textsize.h4", "textsize.body"] {
        file.write_all(format!("        data.settings.{},\n", i).as_bytes()).expect("write failed");
    }
    file.write_all("    ]; \n".as_bytes()).expect("write failed");
    file.write_all("    for i in usizelist {\n".as_bytes()).expect("write failed");
    file.write_all("        SETTINGS_USIZE.lock_mut().unwrap().push(i);\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");

    file.write_all("}\n".as_bytes()).expect("write failed");

    file.write_all("fn h1(text: String) -> Text {\n".as_bytes()).expect("write failed");
    file.write_all("    return Text::new(text).size(SETTINGS_USIZE.lock_mut().unwrap()[2] as u16);\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");

    file.write_all("fn h2(text: String) -> Text {\n".as_bytes()).expect("write failed");
    file.write_all("    return Text::new(text).size(SETTINGS_USIZE.lock_mut().unwrap()[3] as u16);\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");

    file.write_all("fn h3(text: String) -> Text {\n".as_bytes()).expect("write failed");
    file.write_all("    return Text::new(text).size(SETTINGS_USIZE.lock_mut().unwrap()[4] as u16);\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");

    file.write_all("fn h4(text: String) -> Text {\n".as_bytes()).expect("write failed");
    file.write_all("    return Text::new(text).size(SETTINGS_USIZE.lock_mut().unwrap()[5] as u16);\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");
    
    file.write_all("fn body(text: String) -> Text {\n".as_bytes()).expect("write failed");
    file.write_all("    return Text::new(text).size(SETTINGS_USIZE.lock_mut().unwrap()[6] as u16);\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");

    file.write_all("fn main() -> iced::Result {\n".as_bytes()).expect("write failed");
    file.write_all("    loadsettings();\n".as_bytes()).expect("write failed");

    file.write_all("    let rgba = vec![0, 0, 0, 255];\n".as_bytes()).expect("write failed");
    file.write_all("    TABLE.lock_mut().unwrap().push(0);\n".as_bytes()).expect("write failed");
    file.write_all("    LANG.lock_mut().unwrap().push(0);\n".as_bytes()).expect("write failed");
    file.write_all("    loaddata();\n".as_bytes()).expect("write failed");

    file.write_all("    N.lock_mut().unwrap().push(thread_rng().gen_range(0..ENGLISH.lock_mut().unwrap().len()));\n".as_bytes()).expect("write failed");
    file.write_all("    COLOUR.lock_mut().unwrap().push(0);\n".as_bytes()).expect("write failed");
    file.write_all("    SCREEN.lock_mut().unwrap().push(0);\n".as_bytes()).expect("write failed");

    file.write_all("    X.lock_mut().unwrap().push(0);\n".as_bytes()).expect("write failed");


    file.write_all("    let setting: iced::Settings<()> = Settings {\n".as_bytes()).expect("write failed");
    file.write_all("        window: window::Settings {\n".as_bytes()).expect("write failed");

    file.write_all("            size: (900, 600),resizable: true,decorations: true,min_size: Some((900 as u32,600 as u32)),max_size: Some((2000 as u32,2000 as u32)),transparent: false,always_on_top: false,icon: Some(Icon::from_rgba(rgba, 1, 1).unwrap()),position: Specific(0, 0),".as_bytes()).expect("write failed");
    file.write_all("        },default_font: Some(include_bytes!(\"../../resources/Arial Unicode MS Font.ttf\")),antialiasing: true,id: Some(\"October\".to_string()),flags: (),default_text_size: 20,text_multithreading: true,exit_on_close_request: true,try_opengles_first: false,\n".as_bytes()).expect("write failed");

    file.write_all("    };\n".as_bytes()).expect("write failed");
    file.write_all("    MyButton::run(setting)\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");
}