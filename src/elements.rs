use iced::{button, Button};
use crate::Message;
use crate::basic::body;
#[macro_export]
macro_rules! makeslider {
    ($a:expr,$b:expr,$c:expr,$d:expr)=>{
        {
            iced::Slider::new($a,0..=$b,$c as i32,$d)
        }
    };
    ($a:expr,$b:expr,$c:expr,$d:expr, $e:expr)=>{
        {
            iced::Slider::new($a,$b..=$c,$d as i32,$e)
        }
    }
}

pub async fn add_button<'a>(state: &'a mut button::State,content: String,message: Message, usize: usize) -> Button<'a, Message> {
    return Button::new(state, body(usize,content)).on_press(message);
}

