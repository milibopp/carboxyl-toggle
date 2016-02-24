use elmesque::Form;
use elmesque::text::Text;
use elmesque::color::{Color, black, light_blue, light_orange, blue, orange};
use elmesque::form::{text, rect, group};
use benzene::Component;

use app::Position;

pub use carboxyl_window::Event;

pub type Context = Position;

#[derive(Clone)]
pub enum Action { Toggle }

pub type State = bool;


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
    type View = Form;

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

    fn view(&self, context: Context, state: State) -> Form {
        let color = if self.hovers(context) {
            if state { light_blue() }
            else { light_orange() }
        } else {
            if state { blue() }
            else { orange() }
        };
        group(vec![self.button(color), self.hello()])
    }
}

impl Button {
    fn hovers(&self, position: Position) -> bool {
        let Button {width, height, ..} = *self;
        position.0 > -width / 2.0 && position.0 < self.width / 2.0 &&
        position.1 > -height / 2.0 && position.1 < self.height / 2.0
    }

    fn click(&self, event: Event) -> bool {
        use piston::input::Button::Mouse;
        use piston::input::MouseButton::Left;
        use carboxyl_window::Event::Press;

        event == Press(Mouse(Left))
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
