use iced::{executor, Application, Command, Settings, Text, Length, Color, Button, button, Renderer, Element};
use iced::widget::{Container, Column, Row};
use iced_wgpu::Backend;

pub fn main() -> iced::Result {
    Html::run(Settings::default())
}
fn rgbconvert(rgb: [i32; 3]) -> Color {
    let r = rgb[0] as f32 / 255.0;
    let g = rgb[1] as f32/ 255.0;
    let b = rgb[2] as f32 / 255.0;
    let a: f32 = 1.0;
    //println!("{},{},{},{}", r,g,b,a);
    return Color {r,g,b,a};
}

struct Html {
    buttonclicked: Message
}

#[derive(Clone,Debug)]
pub enum Message {
    ButtonClicked,
}

impl Application for Html {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Html, Command<Message>) {
        (Html { buttonclicked: Message::ButtonClicked }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Bresenham's line algorithm")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }
    //90.0,159.0,184.0
    //0.35, 0.62, 0.72
    fn view(&mut self) -> Element<Message> {
        let text1 = Text::new("Bresenham's line algorithm")
        .size(30)
        .color(rgbconvert([90,159,184]))
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .width(Length::Fill);

        let text2 = Text::new("About the code")
        .size(20)
        .color(rgbconvert([90,159,184]));

        let text3 = Text::new(
        "The code in this article was written using Code::Blocks and SDL 2.
        You can read here a guide to install this sofware.
        Although it is based on SDL, I don't use its functions directly. I have written a small library with a few basic
        functions to ease the understanding and the portability to another language.
        You can read more about this lib here")
        .size(15);
        
        /*
        let mut state1 = button::State::new();
        let mut state2 = button::State::new();
        let mut state3 = button::State::new();
        let menu1 = Button::new(&mut state1, Text::new("Home")).on_press(Message::ButtonClicked);
        let menu2 = Button::new(&mut state2, Text::new("Curves")).on_press(Message::ButtonClicked);
        let menu3 = Button::new(&mut state3, Text::new("Contact")).on_press(Message::ButtonClicked);
        */
        let menu1 = Text::new("A").horizontal_alignment(iced::alignment::Horizontal::Left).width(Length::Fill);
        let menu2 = Text::new("B").horizontal_alignment(iced::alignment::Horizontal::Center).width(Length::Fill);
        let menu3 = Text::new("C").horizontal_alignment(iced::alignment::Horizontal::Right).width(Length::Fill);
        let banner = Row::new().push(menu1).push(menu2).push(menu3).width(Length::Fill);
        //let content = Column::new().push(text1).push(text2).push(text3);
        
        let content = Column::new().push(banner).push(text1).push(text2).push(text3);

        Container::new(content)
            .padding(100)
            .width(Length::Fill)
            .height(Length::Fill)
            //.center_x()
            //.center_y()
            .into()
    }
}