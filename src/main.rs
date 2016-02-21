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
extern crate benzene;
extern crate benzene_2d;

use piston::window::WindowSettings;
use carboxyl_window::{Context, Event};
use benzene::{Driver, Component};
use benzene_2d::Driver2d;
use elmesque::Element;
use elmesque::color::black;
use elmesque::form::collage;

use button::Button;

mod button;

pub type Position = (f64, f64);

pub type Dimension = (u32, u32);

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
    type Event = Event;
    type Action = button::Action;
    type State = button::State;
    type View = Element;

    fn intent(&self, context: Context, event: Event)
        -> Option<button::Action>
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
    let mut driver2d = Driver2d::new(settings());
    let output = benzene::start(app(), driver2d.output());
    driver2d.run(output);
}
