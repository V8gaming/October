use std::io::Write;

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() {
    let charlist: [&str; 26] = ["a","b","c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    let charlist2: [&str; 29] = [
    "ẳ","á","â","à","ạ","ầ","ậ", "ấ","ả","ặ",
    "đ",
    "ỏ","ơ","ờ","ồ","ó","ô","ọ",
    "ư","ụ","ữ","ú", "ủ",
    "í","ì","ị",
    "ế","ẹ", "ể"
    ];
    let customstates: [&str; 4] = ["submit","space","delete","next"];
    let customfunctions: [&str; 4] = ["sumbitfn()", "pushfn(\" \")",  "popfn()", "nextfn()"];
    let mut custombuttons = Vec::new();
    for i in customstates {
        custombuttons.push(some_kind_of_uppercase_first_letter(i).to_owned())
    }
    
    let mut file = std::fs::File::create("./src/bin/main.rs").expect("create failed");
    file.write_all(format!("use iced::{{button, Button, Element, Sandbox, Settings, Text, Container, Length, Column, Row, window}};\n").as_bytes()).expect("write failed");
    file.write_all(format!("use iced::window::Position::Specific;\n").as_bytes()).expect("write failed");
    file.write_all(format!("use iced::window::Icon;\n").as_bytes()).expect("write failed");
    file.write_all(format!("use global::Global;\n").as_bytes()).expect("write failed");
    file.write_all(format!("use rand::{{thread_rng, Rng}};\n").as_bytes()).expect("write failed");
    
    // static LETTERS: Global<Vec<String>> = Global::new();
    file.write_all(format!("static LETTERS: Global<Vec<String>> = Global::new();\n").as_bytes()).expect("write failed");

    /*
    static ENGLISH: Global<Vec<&str>> = Global::new();
    static VIETNAMESE: Global<Vec<&str>> = Global::new();
    static N: Global<Vec<usize>> = Global::new();
    */

    file.write_all(format!("static ENGLISH: Global<Vec<&str>> = Global::new();\n").as_bytes()).expect("write failed");
    file.write_all(format!("static VIETNAMESE: Global<Vec<&str>> = Global::new();\n").as_bytes()).expect("write failed");
    file.write_all(format!("static N: Global<Vec<usize>> = Global::new();\n").as_bytes()).expect("write failed");


    file.write_all("\n".as_bytes()).expect("write failed");
    file.write_all(format!("#[derive(Default, Clone)]\nstruct MyButton {{\n").as_bytes()).expect("write failed");

    
    for i in customstates {
        file.write_all(format!("    {}_state: button::State,\n",i).as_bytes()).expect("write failed");
    }

    for i in 0..charlist.len()+charlist2.len() {
        file.write_all(format!("    button_state{}: button::State,\n", i).as_bytes()).expect("write failed");
    }
    file.write_all(format!("}}\n").as_bytes()).expect("write failed");
    file.write_all("\n".as_bytes()).expect("write failed");
    file.write_all("#[derive(Debug, Clone)]".as_bytes()).expect("write failed");

    // emum Message
    file.write_all("enum Message {\n".as_bytes()).expect("write failed");
    for i in &custombuttons {
        file.write_all(format!("    {}Button,\n",i).as_bytes()).expect("write failed");
    }
    for i in 0..charlist.len()+charlist2.len() {
        file.write_all(format!("    ButtonPressed{},\n", i).as_bytes()).expect("write failed");
    }
    file.write_all("}\n".as_bytes()).expect("write failed");

    /* 
    fn pushfn(letter: &str) {
        LETTERS.lock_mut().unwrap().push(letter.to_string());
        println!("ADDED {} TO {}", letter,LETTERS.lock_mut().unwrap().concat());
    }
    */
    file.write_all("fn pushfn(letter: &str) {\n".as_bytes()).expect("write failed");
    file.write_all("    LETTERS.lock_mut().unwrap().push(letter.to_string());\n".as_bytes()).expect("write failed");
    file.write_all("    println!(\"ADDED {} TO {}\", letter,LETTERS.lock_mut().unwrap().concat());\n".as_bytes()).expect("write failed");
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
    file.write_all("    let vietnamese = [\"của chị ấy\",\"vâng\",\"có thể\",\"không thể\",];\n".as_bytes()).expect("write failed");
    file.write_all("    for i in vietnamese {\n".as_bytes()).expect("write failed");
    file.write_all("        VIETNAMESE.lock_mut().unwrap().push(i)\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");

    file.write_all("    if format!(\"{}\", LETTERS.lock_mut().unwrap().concat()) == VIETNAMESE.lock_mut().unwrap()[N.lock_mut().unwrap()[0]]{\n".as_bytes()).expect("write failed");
    file.write_all("        println!(\"true\")\n".as_bytes()).expect("write failed");
    file.write_all("    } else {\n".as_bytes()).expect("write failed");
    file.write_all("        println!(\"false\")\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");

    /* 
    fn popfn() {
        if LETTERS.lock_mut().unwrap().len() != 0 {
            LETTERS.lock_mut().unwrap().pop();
            println!("{}",LETTERS.lock_mut().unwrap().concat());
        } 
    }
    */
    file.write_all("fn popfn() {\n".as_bytes()).expect("write failed");
    file.write_all("    if LETTERS.lock_mut().unwrap().len() != 0 {\n".as_bytes()).expect("write failed");
    file.write_all("        LETTERS.lock_mut().unwrap().pop();\n".as_bytes()).expect("write failed");
    file.write_all("        println!(\"{}\",LETTERS.lock_mut().unwrap().concat());\n".as_bytes()).expect("write failed");
    file.write_all("    }\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");

    /*
    fn nextfn() {
        N.lock_mut().unwrap()[0] = thread_rng().gen_range(0..4);
    }
    */
    file.write_all("fn nextfn() {\n".as_bytes()).expect("write failed");
    file.write_all("N.lock_mut().unwrap()[0] = thread_rng().gen_range(0..4);\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");

    // impl Sandbox for MyButton
    file.write_all("impl Sandbox for MyButton {\n".as_bytes()).expect("write failed");
    file.write_all("    type Message = Message;\n   fn new() -> Self {\n        Self::default()\n  }".as_bytes()).expect("write failed");

    // fn title
    file.write_all("\n  fn title(&self) -> String {\n        String::from(\"Button\")\n   }".as_bytes()).expect("write failed");


    // fn update
    file.write_all("\n  fn update(&mut self, message: Message) {\n        match message {\n".as_bytes()).expect("write failed");

    for i in 0..customstates.len() {
        file.write_all(format!("      Message::{}Button => {},\n", custombuttons[i], customfunctions[i]).as_bytes()).expect("write failed");
    }

    for i in 0..charlist.len()+charlist2.len() {
        if i < charlist.len(){
            file.write_all(format!("        Message::ButtonPressed{} => pushfn(\"{}\"),\n", i, charlist[i]).as_bytes()).expect("write failed");
        } else if i >= charlist.len() {
            file.write_all(format!("        Message::ButtonPressed{} => pushfn(\"{}\"),\n", i, charlist2[i-charlist.len()]).as_bytes()).expect("write failed");
        }
        
    }
    file.write_all("      }\n".as_bytes()).expect("write failed");

    file.write_all("    }\n".as_bytes()).expect("write failed");

    // fn view
    file.write_all("\n  fn view(&mut self) -> Element<Message> {".as_bytes()).expect("write failed");
    file.write_all("\n     fn add_button<'a>(a: &'a mut button::State,b: &'a str,c: Message) -> Button<'a, Message> {\n".as_bytes()).expect("write failed");
    file.write_all("\n          return Button::new(a, Text::new(format!(\"{}\",b))).on_press(c);\n".as_bytes()).expect("write failed");
    file.write_all("\n      }\n".as_bytes()).expect("write failed");
    
    file.write_all("        let english = [\"hers\",\"yes\",\"can\",\"can not\"];\n".as_bytes()).expect("write failed");
    file.write_all("        for i in english {\n".as_bytes()).expect("write failed");
    file.write_all("            ENGLISH.lock_mut().unwrap().push(i)\n".as_bytes()).expect("write failed");
    file.write_all("        }\n".as_bytes()).expect("write failed");

    file.write_all("        let english = Text::new(format!(\"{}\",ENGLISH.lock_mut().unwrap()[N.lock_mut().unwrap()[0]] )).height(Length::Units(150)).size(80);\n".as_bytes()).expect("write failed");
    // text1
    // let text1 = Text::new(format!("{}", LETTERS.lock_mut().unwrap().concat())).height(Length::Units(250)).size(100);
    file.write_all("    let text1 = Text::new(format!(\"{}\", LETTERS.lock_mut().unwrap().concat())).height(Length::Units(150)).size(80);\n".as_bytes()).expect("write failed");

    // button1
    file.write_all("    let buttons1 = [\n".as_bytes()).expect("write failed");
    for i in 0..charlist.len() {
        file.write_all(format!("        add_button(&mut self.button_state{}, \"{}\", Message::ButtonPressed{}),\n",i,charlist[i], i).as_bytes()).expect("write failed");
    }
    file.write_all("    ];\n".as_bytes()).expect("write failed");

    // button2
    file.write_all("    let buttons2 = [\n".as_bytes()).expect("write failed");
    for i in 0..charlist2.len() {
        file.write_all(format!("        add_button(&mut self.button_state{}, \"{}\", Message::ButtonPressed{}),\n",i+charlist.len(),charlist2[i], i+charlist.len()).as_bytes()).expect("write failed");
    }
    file.write_all("    ];\n".as_bytes()).expect("write failed");

    // submit
    // let submit = add_button(&mut self.submit_state, "submit", Message::SubmitButton);
    for i in 0..customstates.len() {
        file.write_all(format!("    let {} = add_button(&mut self.{}_state, \"{}\", Message::{}Button);\n", customstates[i], customstates[i],customstates[i], custombuttons[i]).as_bytes()).expect("write failed");
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

    file.write_all("    let column1 = Column::new().push(english).push(text1).push(userrow).push(row1).push(row2).width(Length::Fill).align_items(iced::Alignment::Center);\n".as_bytes()).expect("write failed");
    file.write_all("    Container::new(column1)\n".as_bytes()).expect("write failed");
    file.write_all("    .padding(100)\n    .width(Length::Fill)\n    .height(Length::Fill)\n    .center_x()\n    .center_y()\n    .into()".as_bytes()).expect("write failed");

    file.write_all("\n    }\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");

    file.write_all("fn main() -> iced::Result {\n".as_bytes()).expect("write failed");
    file.write_all("    let rgba = vec![0, 0, 0, 255];\n".as_bytes()).expect("write failed");
    file.write_all("        N.lock_mut().unwrap().push(thread_rng().gen_range(0..4));\n".as_bytes()).expect("write failed");

    file.write_all("    let setting: iced::Settings<()> = Settings {\n".as_bytes()).expect("write failed");
    file.write_all("        window: window::Settings {\n".as_bytes()).expect("write failed");

    file.write_all("            size: (800, 600),resizable: true,decorations: true,min_size: Some((100 as u32,100 as u32)),max_size: Some((2000 as u32,2000 as u32)),transparent: false,always_on_top: true,icon: Some(Icon::from_rgba(rgba, 1, 1).unwrap()),position: Specific(0, 0),".as_bytes()).expect("write failed");
    file.write_all("        },default_font: Some(include_bytes!(\"../../resources/Arial Unicode MS Font.ttf\")),antialiasing: true,id: Some(\"buttons\".to_string()),flags: (),default_text_size: 20,text_multithreading: true,exit_on_close_request: true,try_opengles_first: false,\n".as_bytes()).expect("write failed");

    file.write_all("    };\n".as_bytes()).expect("write failed");
    file.write_all("    MyButton::run(setting)\n".as_bytes()).expect("write failed");
    file.write_all("}\n".as_bytes()).expect("write failed");
}