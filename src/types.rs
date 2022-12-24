use iced::executor;
#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(iced_native::Event),
    ButtonPressed(String),
    LangChange(usize),
    IndexSent(usize),
    LetterPressed(String),
    SeperateCheckBox(bool),
    SoundBox(bool),
    VolumeSlider(i32),
    TimedBox(bool),
    LengthSlider(i32),
    H1Slider(i32),
    H2Slider(i32),
    H3Slider(i32),
    H4Slider(i32),
    BodySlider(i32),
    LoadButton,
}
pub type Executor = executor::Default;
pub type Flags = ();