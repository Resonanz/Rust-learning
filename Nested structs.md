I searched for structs with structs and seemed to find numerous results that said it was not possible. 

Nevertheless I persisted. How about this (partially stolen from somewhere:

```
#[derive(Default)]
pub struct State {    // Create struct
    i: u32,
}

pub struct WrapApp {    //Create another struct
    state: State,       // and add the first struct as a field 
}

impl WrapApp {
    pub fn new() {
        let mut slf = Self { state: State::default() };    // Set i to default using Default trait

        slf.state.i = 123;    // Now set i to 123. It worked !!!

    }
}
```
