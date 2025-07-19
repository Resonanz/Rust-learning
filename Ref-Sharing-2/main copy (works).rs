/*
This code WORKS !!!  Thanks ChatGPT for helping me sort this !

It uses references to enable passing
of values via pointers between modules.

The difference between this code and
previous code I was struggling with
is that the main function creates
the Victor and passes it into the
builder function/module. This fixes
the problem with messy lifetimes and
references all over the place.

This was used to develop code for the
Texicon widget, however I may resort
to using getters/setters for simplicity.
*/

fn main() {
    use mod_widget::Widget;

    // Vector in central codebase
    let mut v: Vec<Widget> = Vec::new();

    // Create some shared vars
    let int_conf = mod_widget::InternalConfig {
        internal_config1: 123,
        internal_config2: 456,
    };

    // Call builder to generate Config
    let mut shared_states = mod_widget::SharedStates {
        mouse: false,
        rat: 44,
        cat: 55,
    };

    // build returns a Vec<Widget>
    // having passed Vec<Widget> v
    // into build()
    v = builder::build(v, &mut shared_states, int_conf);

    *v[0].mouse = false;

    // Example usage
    if let Some(widget) = v.first_mut() {
        widget.change_a_val();
        widget.print_mouse_val();
        *widget.rat = 43;
        println!("main: mouse = {}", widget.rat);
        *widget.rat = 44;
        widget.print_mouse_val();
    }
}

pub mod builder {
    use crate::mod_widget::{InternalConfig, SharedStates, Widget};

    pub fn build<'a>(
        mut v: Vec<Widget<'a>>,
        shared_states: &'a mut SharedStates,
        int_conf: InternalConfig,
    ) -> Vec<Widget<'a>> {
        // Now pass both structs into the widget
        let widget = Widget::new(shared_states, int_conf);
        v.push(widget);
        v
    }
}

mod mod_widget {
    #[derive(Default, Debug)]
    pub struct InternalConfig {
        // Pretend ConfigBuilder struct
        pub internal_config1: u32,
        pub internal_config2: u32,
    }

    pub struct SharedStates {
        pub mouse: bool,
        pub rat: u32,
        pub cat: u32,
    }

    #[derive(Debug)]
    pub struct Widget<'a> {
        pub mouse: &'a mut bool,
        pub rat: &'a mut u32,
        pub cat: &'a mut u32,
        _internal_conf: InternalConfig,
    }

    impl<'a> Widget<'a> {
        pub fn new(texi_struct: &'a mut SharedStates, int_conf: InternalConfig) -> Self {
            Widget {
                mouse: &mut texi_struct.mouse,
                rat: &mut texi_struct.rat,
                cat: &mut texi_struct.cat,
                _internal_conf: int_conf,
            }
        }

        pub fn change_a_val(&mut self) {
            *self.mouse = true;
            *self.rat = 42;
            *self.cat = 42;
        }

        pub fn print_mouse_val(&self) {
            println!("mod_widget mouse = {}", self.mouse);
            println!("mod_widget rat = {}", self.rat);
        }
    }
}
