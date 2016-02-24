use benzene::Component;
use carboxyl_window::{Context, Event};
use elmesque::Element;
use elmesque::color::black;
use elmesque::form::collage;
use button::{self, Button};

#[derive(Clone)]
pub struct App {
    button: Button
}

impl App {
    pub fn new() -> App {
        App { button: button::Button {
            width: 200.0,
            height: 100.0,
            label: "Hello!".to_string()
        }}
    }
}

pub type Position = (f64, f64);

pub type Dimension = (u32, u32);

fn centered(size: Dimension, position: Position) -> Position {
    (position.0 - size.0 as f64 / 2.0,
     position.1 - size.1 as f64 / 2.0)
}

impl Component for App {
    type Context = Context;
    type Event = Event;
    type Action = button::Action;
    type State = button::State;
    type View = Element;

    fn intent(&self, context: Context, event: Event) -> Option<button::Action>
    {
        self.button.intent(
            centered(context.window.size, context.cursor.position),
            event)
    }

    fn init(&self) -> button::State {
        self.button.init()
    }

    fn update(&self, current: button::State, action: button::Action)
        -> button::State
    {
        self.button.update(current, action)
    }

    fn view(&self, context: Context, state: button::State) -> Element {
        let (width, height) = context.window.size;
        let button_view = self.button.view(
            centered(context.window.size, context.cursor.position),
            state);
        collage(width as i32, height as i32, vec![button_view])
            .clear(black())
    }
}
