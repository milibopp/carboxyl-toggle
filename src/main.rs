//! Simple interactive application

extern crate elmesque;
extern crate graphics;
extern crate glium;
extern crate glium_graphics;
extern crate shader_version;
extern crate piston;
extern crate glutin_window;
#[macro_use(lift)]
extern crate carboxyl;
extern crate carboxyl_window;

use piston::window::WindowSettings;
use carboxyl_window::StreamingWindow;
use carboxyl_window::button::ButtonEvent;
use elmesque::Element;
use elmesque::color::black;
use elmesque::form::collage;

use component::Component;
use button::Button;
use start::{start, Context, Position, Dimension};

mod runner;
mod button;
pub mod start;
pub mod component;

fn centered(size: Dimension, position: Position) -> Position {
    (position.0 - size.0 as f64 / 2.0,
     position.1 - size.1 as f64 / 2.0)
}

#[derive(Clone)]
struct App {
    button: Button
}

impl Component for App {
    type Context = Context;
    type Event = ButtonEvent;
    type Action = button::Action;
    type State = button::State;
    type View = Element;

    fn intent(&self, context: Context, event: ButtonEvent)
        -> Option<button::Action>
    {
        let Context {position, size, ..} = context;
        self.button.intent(centered(size, position), event)
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
        let (width, height) = context.size;
        let Context {position, size, ..} = context;
        let button_view = self.button.view(centered(size, position), state);
        collage(width as i32, height as i32, button_view)
            .clear(black())
    }
}

fn app() -> App {
    App { button: button::Button {
        width: 200.0,
        height: 100.0,
        label: "Hello!".to_string()
    }}
}

fn settings() -> WindowSettings {
    WindowSettings::new("carboxyl_window :: example/simple.rs", (640, 480))
}

fn main() {
    runner::run_glutin(settings(), |win| start(app(), win));
}
