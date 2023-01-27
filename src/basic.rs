use crate::Mainstruct;
use iced::Text;

pub fn h1(usize: usize, text: String) -> Text {
    Text::new(text).size(usize as u16)
}
pub fn h2(usize: usize, text: String) -> Text {
    Text::new(text).size(usize as u16)
}
pub fn h3(usize: usize, text: String) -> Text {
    Text::new(text).size(usize as u16)
}
pub fn h4(usize: usize, text: String) -> Text {
    Text::new(text).size(usize as u16)
}
pub fn body(usize: usize, text: String) -> Text {
    Text::new(text).size(usize as u16)
}
pub fn get_time(selfx: &mut Mainstruct) -> String {
    // Borrow values from RwLock instead of cloning them
    let settings = selfx.settings_usize[1];
    let time = selfx.time;

    // Cache elapsed time value in a local variable
    let elapsed = time.elapsed().as_secs_f32();

    // Calculate and return result
    let new_time = settings as f32 - elapsed;
    format!("{new_time:.2}")
}
