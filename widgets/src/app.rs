use mix_platform::Cx;
use mix_platform::event::Event;
use mix_draw::Cx2d;

pub trait AppMain: 'static {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event);
}

pub struct App<T: AppMain> {
    pub app: T,
}

impl<T: AppMain> App<T> {
    pub fn new(app: T) -> Self {
        Self { app }
    }

    pub fn run(mut self) {
        let mut cx = Cx::new();

        // Create event handler
        let event_handler = |cx: &mut Cx, event: Event| {
            self.app.handle_event(cx, &event);

            // Handle draw events
            if let Event::Draw = event {
                // Draw is handled by the app's handle_event
                let _cx2d = Cx2d::new(cx);
            }
        };

        // Run the event loop
        cx.run(event_handler);
    }
}

#[macro_export]
macro_rules! app_main {
    ($app_type:ty) => {
        fn main() {
            let app = <$app_type>::new();
            let app_runner = $crate::app::App::new(app);
            app_runner.run();
        }
    };
}
