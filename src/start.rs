use carboxyl_window::StreamingWindow;
use carboxyl_window::button::ButtonEvent;
use carboxyl::{Signal, Stream};
use ::component::Component;

pub type Position = (f64, f64);

pub type Dimension = (u32, u32);

#[derive(Clone)]
pub struct Context {
    pub position: Position,
    pub size: Dimension
}

fn window_context<W: StreamingWindow>(window: &W) -> Signal<Context> {
    lift!(|p, s| Context { position: (p.0 as f64, p.1 as f64), size: s },
        &window.cursor(),
        &window.size()
    )
}

fn window_events<W: StreamingWindow>(window: &W) -> Stream<ButtonEvent> {
    window.buttons()
}

pub fn window_communication<W: StreamingWindow>(window: &W)
    -> Communication<Context, ButtonEvent>
{
    Communication {
        context: window_context(window),
        events: window_events(window)
    }
}

pub struct Communication<C, E> {
    context: Signal<C>,
    events: Stream<E>
}

pub fn start<C>(app: C, inputs: Communication<C::Context, C::Event>)
    -> Signal<C::View>
    where C: Component + Clone + Send + Sync + 'static,
          C::Action: Clone + Send + Sync + 'static,
          C::State: Clone + Send + Sync + 'static,
          C::Context: Clone + Send + Sync + 'static,
          C::Event: Clone + Send + Sync + 'static,
          C::View: Clone + Send + Sync + 'static
{
    let actions = inputs.context.snapshot(&inputs.events, {
            let app = app.clone();
            move |x, y| app.intent(x, y)
        })
        .filter_some();
    let state = actions.fold(app.init(), {
        let app = app.clone();
        move |x, y| app.update(x, y)
    });
    lift!(move |x, y| app.view(x, y), &inputs.context, &state)
}
