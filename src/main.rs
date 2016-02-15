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
use carboxyl::{Signal, Stream};
use carboxyl_window::StreamingWindow;
use carboxyl_window::button::ButtonEvent;
use elmesque::{Element, Form};
use elmesque::text::Text;
use elmesque::color::{Color, black, green, red};
use elmesque::form::{collage, text, rect};

mod runner;



type Context = (f64, f64);

fn centered(size: (u32, u32), position: (f64, f64)) -> (f64, f64) {
    (position.0 - size.0 as f64 / 2.0,
     position.1 - size.1 as f64 / 2.0)
}

fn context<W: StreamingWindow>(window: &W) -> Signal<Context> {
    lift!(centered, &window.size(), &window.cursor())
}


type Event = ();

fn clicks(event: ButtonEvent) -> Option<Event> {
    use piston::input::Button::Mouse;
    use piston::input::MouseButton::Left;
    use carboxyl_window::button::ButtonState::Pressed;

    if event.button == Mouse(Left) && event.state == Pressed {
        Some(())
    } else {
        None
    }
}

fn events<W: StreamingWindow>(window: &W) -> Stream<Event> {
    window.buttons()
        .filter_map(clicks)
}


type Action = ();

fn hovers(position: (f64, f64)) -> bool {
    position.0 > 0.0
}

fn intent(cursor_position: Context, (): Event) -> Option<Action> {
    if hovers(cursor_position) {
        Some(())
    } else {
        None
    }
}


type State = bool;

fn init() -> bool {
    false
}

fn update(current: State, (): Action) -> State {
    !current
}


type View = Vec<Form>;

fn hello() -> Form {
    text(Text::from_string("Hello!".to_string())
        .color(black())
        .height(50.))
}

fn button(color: Color) -> Form {
    rect(200.0, 100.0).filled(color)
}

fn view(_cursor: Context, state: State) -> View {
    vec![button(if state { red() } else { green() }), hello()]
}

fn display((width, height): (u32, u32), forms: Vec<Form>) -> Element {
    collage(width as i32, height as i32, forms)
        .clear(black())
}


fn app<W: StreamingWindow>(window: &W) -> Signal<Element> {
    let context = context(window);
    let actions = context.snapshot(&events(window), intent)
        .filter_some();
    let state = actions.fold(init(), update);
    let view = lift!(view, &context, &state);
    lift!(display, &window.size(), &view)
}


fn settings() -> WindowSettings {
    WindowSettings::new("carboxyl_window :: example/simple.rs", (640, 480))
}

fn main() {
    runner::run_glutin(settings(), app);
}
