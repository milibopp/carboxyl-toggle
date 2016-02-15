Pseudo-code of component architecture:

```
type Context
type Event
type Input = {
    context: Signal Context,
    events: Stream Event
}

type Action
intent : Context -> Event -> Option Action

type State
update : Action -> State -> State
init : State

type Output
view : Context -> State -> Output


program : Input -> Signal Output
program {context, events} =
    let actions: Stream Action = snapshot intent events context |> filter_some
    let state: Signal State = fold init update actions
    let output: Signal Output = lift2 view context state
    output
```
