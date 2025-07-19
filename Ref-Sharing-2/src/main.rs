use module3::States;

fn main() {
    let texistate1 = States { mouse: false }; // concrete states for each texicon
    let texistate2 = States { mouse: false };
    let texistate3 = States { mouse: false };
    let m = module1::TexiStates::new(texistate1, texistate2, texistate3);
    println!("vec = {m:?}");
}

pub mod module1 {
    use crate::module3::States;

    #[derive(Debug)]
    pub struct TexiStates {
        pub v: Vec<States>,
    }

    // impl Default for MyStruct {
    //     fn default() -> Self {
    //         Self { vec: Vec::new() }
    //     }
    // }
    impl TexiStates {
        pub fn new(s1: States, s2: States, s3: States) -> Self {
            let mut w: Vec<States> = Vec::new();
            w.push(s1);
            w.push(s2);
            w.push(s3);
            Self { v: w }
        }
    }
}

pub mod module2 {
    use crate::module3::{self, Config, States, Widget};

    pub fn config(vec_of_states: Vec<States>) {
        let conf = Config { config: 1243 };
        let mut states = States { mouse: false };
        let state_ref = Widget::new(conf, &mut states);
        // vec_of_states.push(states_ref);
    }
}

pub mod module3 {
    pub struct Config {
        pub config: u32,
    }

    #[derive(Debug)]
    pub struct States {
        pub mouse: bool,
    }

    pub struct Widget<'a> {
        pub mouse: &'a mut bool,
    }

    impl<'a> Widget<'a> {
        pub fn new(config: Config, state: &'a mut States) -> Self {
            Self {
                mouse: &mut state.mouse,
            }
        }

        pub fn change_a_val(&mut self) {
            *self.mouse = true;
        }

        pub fn print_mouse_val(&self) {
            println!("mod_widget mouse = {}", self.mouse);
        }
    }
}
