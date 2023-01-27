use crate::basic::body;
use crate::Message;
use iced::{button, Button};
#[macro_export]
macro_rules! makeslider {
    ($a:expr,$b:expr,$c:expr,$d:expr) => {{
        iced::Slider::new($a, 0..=$b, $c as i32, $d)
    }};
    ($a:expr,$b:expr,$c:expr,$d:expr, $e:expr) => {{
        iced::Slider::new($a, $b..=$c, $d as i32, $e)
    }};
}

pub fn add_button(
    state: &'_ mut button::State,
    content: String,
    message: Message,
    usize: usize,
) -> Button<'_, Message> {
    return Button::new(state, body(usize, content)).on_press(message);
}
