use carboxyl_window::button::ButtonEvent;
use elmesque::Form;
use elmesque::text::Text;
use elmesque::color::{Color, black, light_blue, light_orange, blue, orange};
use elmesque::form::{text, rect};
use ::Position;
use ::component::Component;

pub type Context = Position;

pub type Event = ButtonEvent;

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

impl Component for Button {
    type Context = Position;
    type Event = Event;
    type Action = Action;
    type State = bool;
    type View = Vec<Form>;

    fn intent(&self, context: Context, event: Event) -> Option<Action> {
        if self.click(event) && self.hovers(context) {
            Some(Action::Toggle)
        } else {
            None
        }
    }

    fn init(&self) -> State {
        false
    }

    fn update(&self, current: State, _: Action) -> State {
        !current
    }

    fn view(&self, context: Context, state: State) -> View {
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

impl Button {
    fn hovers(&self, position: Position) -> bool {
        let Button {width, height, ..} = *self;
        position.0 > -width / 2.0 && position.0 < self.width / 2.0 &&
        position.1 > -height / 2.0 && position.1 < self.height / 2.0
    }

    fn click(&self, event: ButtonEvent) -> bool {
        use piston::input::Button::Mouse;
        use piston::input::MouseButton::Left;
        use carboxyl_window::button::ButtonState::Pressed;

        event.button == Mouse(Left) && event.state == Pressed
    }

    fn hello(&self) -> Form {
        text(Text::from_string(self.label.clone())
            .color(black())
            .height(self.height / 2.0))
    }

    fn button(&self, color: Color) -> Form {
        rect(self.width, self.height).filled(color)
    }
}
