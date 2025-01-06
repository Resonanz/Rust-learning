//
// Passing and updating through references
//
// We have main() + a "mod_widget" module.
//
// The struct containing the mod_widget
// data to be shared across modules
// is defined in the mod_widget module.
//
// This struct is instantiated in
// main() as "states" and populated
// with initial values.
//
// The struct is then passed as a
// &mut back into the mod_widget widget.
//
// The &mut struct values are then
// used to update a second identical
// -ish struct which contains &'a mut
// variables rather than concrete
// values. Viola !!!
//
// Now either main() or widget can
// update the values inside the
// references.

fn main() {
    // Let's say we want to pass a struct
    // populated using a ConfigBuilder into the
    // mod_widget module...(pretend ConfigBuilder)
    let animals = mod_widget::Animals { cat: 123, dog: 456 };

    // Also create a struct using a struct
    // inside the widget to hold shared data
    let mut states = mod_widget::State {
        mouse: false,
        rat: 44,
        cat: 55,
    };

    // Now pass both structs
    // back into the widget
    let mut portal = mod_widget::Widget::new(&mut states, animals);

    // Now change the referenced value inside the widget.
    portal.change_a_val();
    println!("main mouse = {}", portal.mouse);
    portal.print_mouse_val();
    *portal.mouse = false;
    portal.print_mouse_val();
}

mod mod_widget {
    #[derive(Default)]
    pub struct Animals {
        // Pretend ConfigBuilder struct
        pub cat: u32,
        pub dog: u32,
    }

    pub struct State {
        pub mouse: bool,
        pub rat: u32,
        pub cat: u32,
    }

    pub struct Widget<'a> {
        pub mouse: &'a mut bool,
        pub rat: &'a mut u32,
        pub cat: &'a mut u32,
        _animals: Animals,
    }

    impl<'a> Widget<'a> {
        pub fn new(texi_struct: &'a mut State, animals: Animals) -> Self {
            Widget {
                mouse: &mut texi_struct.mouse,
                rat: &mut texi_struct.rat,
                cat: &mut texi_struct.cat,
                _animals: animals,
            }
        }

        pub fn change_a_val(&mut self) {
            *self.mouse = true;
            *self.rat = 42;
            *self.cat = 42;
        }

        pub fn print_mouse_val(&self) {
            println!("mod_widget mouse = {}", self.mouse)
            // println!("anim")
        }
    }
}

// pub mod xyz {
//     pub struct Remote<'a> {
//         pub var: &'a mut u32,
//     }

//     impl<'a> Remote<'a> {
//         pub fn new() -> Self {}

//         pub fn rem_update(&mut self) {}
//     }
// }
