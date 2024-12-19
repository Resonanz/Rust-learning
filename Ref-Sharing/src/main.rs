//
// Passing and updating through references
//
// We have main() + two modules.
// A reference to a variable "value"
// is defined in main().
//
// Value is then passed from main
// to module to module and updated
// in each. At the same time the
// value in value is printed to
// show that updating the value
// in reference can be used to
// share the value between the
// various modules.
//
// But... what if the want value
// to be defined in module 1 ?

fn main() {
    let mut value: u32 = 456;
    let mut result = abc::MyStruct::new(&mut value);
    println!("1. main_var = {}", result.main_var);
    *result.main_var = 789;
    result.update();
    println!("6. main_var = {}", result.main_var);
}

mod abc {
    use crate::xyz;

    pub struct MyStruct<'a> {
        pub main_var: &'a mut u32,
    }

    impl<'a> MyStruct<'a> {
        pub fn new(main_var: &'a mut u32) -> Self {
            Self { main_var }
        }

        pub fn update(&mut self) {
            println!("2. main_var = {}", self.main_var);
            *self.main_var = 234;
            println!("3. main_var = {}", self.main_var);
            let mut rem = xyz::Remote::new(self.main_var);
            println!("4. xyz var = {}", rem.var);
            rem.rem_update();
            println!("5. xyz var = {}", rem.var);
        }
    }
}

pub mod xyz {
    pub struct Remote<'a> {
        pub var: &'a mut u32,
    }

    impl<'a> Remote<'a> {
        pub fn new(v: &'a mut u32) -> Self {
            Self { var: v }
        }

        pub fn rem_update(&mut self) {
            *self.var = 246;
        }
    }
}
