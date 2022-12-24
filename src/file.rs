use crate::Mainstruct;
use std::io::Write;
pub fn savefn(selfx: &mut Mainstruct) {
    let mut file = std::fs::File::create("./settings.toml").expect("create failed");
    file.write_all(format!("[settings]\n").as_bytes()).expect("write failed");
    file.write_all(format!("seperate_check_synonyms = {} # 0\n", selfx.settings_bool[0]).as_bytes()).expect("write failed");
    file.write_all(format!("\n").as_bytes()).expect("write failed");
    file.write_all(format!("[settings.sound]\n").as_bytes()).expect("write failed");
    file.write_all(format!("sound =  {} # 1\n", selfx.settings_bool[1]).as_bytes()).expect("write failed");
    file.write_all(format!("volume = {} # 0\n", selfx.settings_usize[0]).as_bytes()).expect("write failed");
    file.write_all(format!("\n").as_bytes()).expect("write failed");

    file.write_all(format!("[settings.time]\n").as_bytes()).expect("write failed");
    file.write_all(format!("timed =  {} # 2\n", selfx.settings_bool[2]).as_bytes()).expect("write failed");
    file.write_all(format!("length = {} # 1\n", selfx.settings_usize[1]).as_bytes()).expect("write failed");
    file.write_all(format!("\n").as_bytes()).expect("write failed");

    file.write_all(format!("[settings.textsize]\n").as_bytes()).expect("write failed");
    file.write_all(format!("h1 = {} # 2\n", selfx.settings_usize[2]).as_bytes()).expect("write failed");
    file.write_all(format!("h2 = {} # 3\n", selfx.settings_usize[3]).as_bytes()).expect("write failed");
    file.write_all(format!("h3 = {} # 4\n", selfx.settings_usize[4]).as_bytes()).expect("write failed");
    file.write_all(format!("h4 = {} # 5\n", selfx.settings_usize[5]).as_bytes()).expect("write failed");
    file.write_all(format!("body = {} # 6\n", selfx.settings_usize[6]).as_bytes()).expect("write failed");
    file.write_all(format!("\n").as_bytes()).expect("write failed");

}