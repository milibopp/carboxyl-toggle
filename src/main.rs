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
use carboxyl::Signal;
use carboxyl_window::StreamingWindow;
use elmesque::{Element, Form};
use elmesque::color::black;
use elmesque::form::collage;

mod runner;
mod button;

pub type Position = (f64, f64);

type Dimension = (u32, u32);


fn centered(size: Dimension, position: Position) -> Position {
    (position.0 - size.0 as f64 / 2.0,
     position.1 - size.1 as f64 / 2.0)
}


fn view((width, height): (u32, u32), forms: Vec<Form>) -> Element {
    collage(width as i32, height as i32, forms)
        .clear(black())
}


fn app<W: StreamingWindow>(window: &W) -> Signal<Element> {
    let button = button::Button {
        width: 200.0,
        height: 100.0,
        label: "Hello!".to_string()
    };
    let context = lift!(centered, &window.size(), &window.cursor());
    let actions = context.snapshot(&button.events(window), {
            let button = button.clone();
            move |x, y| button.intent(x, y)
        })
        .filter_some();
    let state = actions.fold(button.init(), {
        let button = button.clone();
        move |x, y| button.update(x, y)
    });
    lift!(view,
        &window.size(),
        &lift!(move |x, y| button.view(x, y), &context, &state))
}


fn settings() -> WindowSettings {
    WindowSettings::new("carboxyl_window :: example/simple.rs", (640, 480))
}

fn main() {
    runner::run_glutin(settings(), app);
}
