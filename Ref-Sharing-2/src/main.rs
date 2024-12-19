//
// Passing and updating through references
//
// We have main() + a "texicon" module.
//
// The struct containing the texicon
// data to be shared across modules
// is defined in the texicon module.
//
// This struct is instantiated in
// main() as "states" and populated
// with initial values.
//
// The struct is then passed as a
// &mut back into the texicon widget.
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
    // Create struct from struct inside widget
    // states holds the populated struct
    let mut states = texicon::State { mouse: 789 };
    // Pass the populated struct back into the
    // widget as &mut type.
    let mut portal = texicon::StateRefs::new(&mut states);
    // Now change the referenced value inside the widget.
    portal.change_a_val();
    println!("main mouse = {}", portal.mouse);
    portal.print_mouse_val();
    *portal.mouse = 800;
    portal.print_mouse_val();
}

mod texicon {
    pub struct State {
        pub mouse: u32,
    }

    pub struct StateRefs<'a> {
        pub mouse: &'a mut u32,
    }

    impl<'a> StateRefs<'a> {
        pub fn new(texi_struct: &'a mut State) -> Self {
            StateRefs {
                mouse: &mut texi_struct.mouse,
            }
        }
        pub fn change_a_val(&mut self) {
            *self.mouse = 42;
        }

        pub fn print_mouse_val(&self) {
            println!("texicon mouse = {}", self.mouse)
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
