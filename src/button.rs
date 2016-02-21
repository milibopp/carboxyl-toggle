use carboxyl_window::StreamingWindow;
use carboxyl_window::button::ButtonEvent;
use carboxyl::Stream;
use elmesque::Form;
use elmesque::text::Text;
use elmesque::color::{Color, black, light_blue, light_orange, blue, orange};
use elmesque::form::{text, rect};
use ::Position;

pub type Context = Position;

#[derive(Clone)]
pub enum Event { Click }

#[derive(Clone)]
pub enum Action { Toggle }

pub type State = bool;

pub type View = Vec<Form>;


#[derive(Clone)]
pub struct Button {
    pub width: f64,
    pub height: f64,
    pub label: String
}

impl Button {
    fn hovers(&self, position: Position) -> bool {
        let Button {width, height, ..} = *self;
        position.0 > -width / 2.0 && position.0 < self.width / 2.0 &&
        position.1 > -height / 2.0 && position.1 < self.height / 2.0
    }

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

    pub fn events<W: StreamingWindow>(&self, window: &W) -> Stream<Event> {
        window.buttons()
            .filter_map(Button::clicks)
    }

    pub fn intent(&self, context: Context, _: Event) -> Option<Action> {
        if self.hovers(context) { Some(Action::Toggle) } else { None }
    }

    pub fn init(&self) -> State {
        false
    }

    pub fn update(&self, current: State, _: Action) -> State {
        !current
    }

    fn hello(&self) -> Form {
        text(Text::from_string(self.label.clone())
            .color(black())
            .height(self.height / 2.0))
    }

    fn button(&self, color: Color) -> Form {
        rect(self.width, self.height).filled(color)
    }

    pub fn view(&self, context: Context, state: State) -> View {
        let color = if self.hovers(context) {
            if state { light_blue() }
            else { light_orange() }
        } else {
            if state { blue() }
            else { orange() }
        };
        vec![self.button(color), self.hello()]
    }
}
