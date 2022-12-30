use iced_native::{Executor, Command, Subscription, Color,renderer};
use iced::{window, Element, Settings, Result};
use iced_wgpu::Settings as WGPUSettings;
use iced_winit::application;

pub trait ApplicationModified: Sized {
    type Executor: Executor;

    /// The type of __messages__ your [`Application`] will produce.
    type Message: std::fmt::Debug + Send;

    /// The data needed to initialize your [`Application`].
    type Flags;
    fn new(&mut self, flags: Self::Flags) -> (Self, Command<Self::Message>);

    /// Returns the current title of the [`Application`].
    ///
    /// This title can be dynamic! The runtime will automatically update the
    /// title of your application when necessary.
    fn title(&self) -> String;
    fn update(&mut self, message: Self::Message) -> Command<Self::Message>;

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    /// Returns the widgets to display in the [`Application`].
    ///
    /// These widgets can produce __messages__ based on user interaction.
    fn view(&mut self) -> Element<'_, Self::Message>;

    fn mode(&self) -> window::Mode {
        window::Mode::Windowed
    }

    /// Returns the background color of the [`Application`].
    ///
    /// By default, it returns [`Color::WHITE`].
    fn background_color(&self) -> Color {
        Color::WHITE
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    /// Returns whether the [`Application`] should be terminated.
    ///
    /// By default, it returns `false`.
    fn should_exit(&self) -> bool {
        false
    }

    /// Runs the [`Application`].
    ///
    /// On native platforms, this method will take control of the current thread
    /// until the [`Application`] exits.
    ///
    /// On the web platform, this method __will NOT return__ unless there is an
    /// [`Error`] during startup.
    ///
    /// [`Error`]: crate::Error
    fn run(settings: Settings<Self::Flags>) -> Result
    where
        Self: 'static,
    {
        let renderer_settings = WGPUSettings {
            default_font: settings.default_font,
            default_text_size: settings.default_text_size,
            text_multithreading: settings.text_multithreading,
            antialiasing: if settings.antialiasing {
                Some(WGPUSettings::Antialiasing::MSAAx4)
            } else {
                None
            },
            ..WGPUSettings::from_env()
        };

        Ok(application::run::<
            Instance<Self>,
            Self::Executor,
            renderer::window::Compositor,
        >(settings.into(), renderer_settings)?)
    }
}