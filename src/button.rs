use carboxyl_window::StreamingWindow;
use carboxyl_window::button::ButtonEvent;
use carboxyl::Stream;
use elmesque::Form;
use elmesque::text::Text;
use elmesque::color::{Color, black, light_blue, light_orange, blue, orange};
use elmesque::form::{text, rect};
use ::Position;

#[derive(Clone)]
pub enum Context {
    Hover,
    Free
}

fn hovers(position: Position) -> bool {
    position.0 > -100.0 && position.0 < 100.0 &&
    position.1 > -50.0 && position.1 < 50.0
}

pub fn context(cursor: Position) -> Context {
    if hovers(cursor) {
        Context::Hover
    } else {
        Context::Free
    }
}


#[derive(Clone)]
pub enum Event { Click }

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

pub fn events<W: StreamingWindow>(window: &W) -> Stream<Event> {
    window.buttons()
        .filter_map(clicks)
}

#[derive(Clone)]
pub enum Action { Toggle }

pub fn intent(context: Context, _: Event) -> Option<Action> {
    match context {
        Context::Hover => Some(Action::Toggle),
        Context::Free => None
    }
}


pub type State = bool;

pub fn init() -> State {
    false
}

pub fn update(current: State, _: Action) -> State {
    !current
}


pub type View = Vec<Form>;

fn hello() -> Form {
    text(Text::from_string("Hello!".to_string())
        .color(black())
        .height(50.))
}

fn button(color: Color) -> Form {
    rect(200.0, 100.0).filled(color)
}

pub fn view(context: Context, state: State) -> View {
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

