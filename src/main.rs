//! Simple interactive application

extern crate elmesque;
extern crate graphics;
extern crate glium;
extern crate glium_graphics;
extern crate shader_version;
extern crate input;
extern crate window;
extern crate glutin_window;
#[macro_use(lift)]
extern crate carboxyl;
extern crate carboxyl_window;

use window::WindowSettings;
use carboxyl::Signal;
use carboxyl_window::StreamingWindow;
use carboxyl_window::button::ButtonEvent;
use elmesque::{Element, Form};
use elmesque::text::Text;
use elmesque::color::{Color, black, white, green, red};
use elmesque::form::{collage, text, rect};

mod runner;


fn hello() -> Form {
    text(Text::from_string("Hello!".to_string())
        .color(black())
        .height(50.))
}

fn button(color: Color) -> Form {
    rect(200.0, 100.0).filled(color)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Model {
    Normal,
    Hover,
    Active
}

impl Model {
    fn color(self) -> Color {
        match self {
            Model::Normal => white(),
            Model::Hover => green(),
            Model::Active => red(),
        }
    }
}

fn hovers(position: (f64, f64)) -> bool {
    position.0 > 700.0
}

fn view((width, height): (u32, u32), model: Model) -> Element {
    collage(width as i32,
            height as i32,
            vec![button(model.color()), hello()])
        .clear(black())
}

fn init() -> Model {
    Model::Normal
}

type State = bool;
type Action = ButtonEvent;
type Input = (f64, f64);

fn update(previous: State, input: Input, _: Action) -> State {
    if hovers(input) {
        !previous
    } else {
        previous
    }
}

fn model(state: State, input: Input) -> Model {
    if state {
        Model::Active
    } else if hovers(input) {
        Model::Hover
    } else {
        Model::Normal
    }
}

fn app<W: StreamingWindow>(window: &W) -> Signal<Element> {
    let state = window.cursor()
        .snapshot(&window.buttons(), |x, y| (x, y))
        .fold(false, |st, (i, a)| update(st, i, a));
    lift!(view, &window.size(), &lift!(model, &state, &window.cursor()))
}

fn settings() -> WindowSettings {
    WindowSettings::new("carboxyl_window :: example/simple.rs", (640, 480))
}

fn main() {
    runner::run_glutin(settings(), app);
}
