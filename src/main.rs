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
use elmesque::color::{Color, black, light_blue, light_orange, blue, orange};
use elmesque::form::{collage, text, rect};

mod runner;


type Position = (f64, f64);

type Dimension = (u32, u32);


#[derive(Clone)]
enum Context {
    Hover,
    Free
}

fn centered(size: Dimension, position: Position) -> Position {
    (position.0 - size.0 as f64 / 2.0,
     position.1 - size.1 as f64 / 2.0)
}

fn hovers(position: Position) -> bool {
    position.0 > -100.0 && position.0 < 100.0 &&
    position.1 > -50.0 && position.1 < 50.0
}

fn context<W: StreamingWindow>(window: &W) -> Signal<Context> {
    lift!(
        |size, cursor|
            if hovers(centered(size, cursor)) {
                Context::Hover
            } else {
                Context::Free
            },
        &window.size(),
        &window.cursor()
    )
}


#[derive(Clone)]
enum Event { Click }

fn clicks(event: ButtonEvent) -> Option<Event> {
    use piston::input::Button::Mouse;
    use piston::input::MouseButton::Left;
    use carboxyl_window::button::ButtonState::Pressed;

    if event.button == Mouse(Left) && event.state == Pressed {
        Some(Event::Click)
    } else {
        None
    }
}

fn events<W: StreamingWindow>(window: &W) -> Stream<Event> {
    window.buttons()
        .filter_map(clicks)
}

#[derive(Clone)]
enum Action { Toggle }

fn intent(context: Context, _: Event) -> Option<Action> {
    match context {
        Context::Hover => Some(Action::Toggle),
        Context::Free => None
    }
}


type State = bool;

fn init() -> bool {
    false
}

fn update(current: State, _: Action) -> State {
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

fn view(context: Context, state: State) -> View {
    let color = match context {
        Context::Hover =>
            if state { light_blue() }
            else { light_orange() },
        Context::Free =>
            if state { blue() }
            else { orange() }
    };
    vec![button(color), hello()]
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
