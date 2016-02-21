pub trait Component {
    type Context;
    type Event;
    type Action;
    type State;
    type View;

    fn intent(&self, Self::Context, Self::Event) -> Self::Action;
    fn init(&self) -> Self::State;
    fn update(&self, Self::State, Self::Action) -> Self::Action;
    fn view(&self, Self::Context, Self::State) -> Self::View;
}
