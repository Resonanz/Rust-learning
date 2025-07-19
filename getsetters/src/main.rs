const CAPACITY: usize = 10000;
fn main() {
    use arrayvec::ArrayVec;
    use widget::State;

    // Vec
    let mut v: Vec<State> = Vec::new(); // Create vector of widgets
    builder::vec_builder(&mut v); // Populate vector
    dbg!(v.len(), v.capacity(), v.as_ptr());

    // ArrayVec
    let mut av = ArrayVec::<State, CAPACITY>::new();
    builder::avec_builder(&mut av); // Populate vector
    dbg!(av.len(), av.capacity(), av.as_ptr());

    // println!("Getting widget state s: {}", State::get_state_s(&self));
    // println!("Getting widget vectors: {:?}", v);
    println!("Getting widget v[0] s state: {:?}", v[0].get_state_s());

    // Vec
    let mut vec: Vec<usize> = Vec::new();
    let start = std::time::Instant::now();
    (0..CAPACITY).for_each(|f| {
        v[f].set_state_s(f);
        vec.push(f);
    });
    let duration = start.elapsed();

    println!("vec = {:?}", vec[CAPACITY - 1]);
    println!("vec Elapsed time = {:?}", duration);

    // ArrayVec
    let mut avec = ArrayVec::<usize, CAPACITY>::new();
    let start = std::time::Instant::now();
    (0..CAPACITY).for_each(|f| {
        av[f].set_state_s(f);
        avec.push(f);
    });
    let duration = start.elapsed();

    println!("avec = {:?}", avec[CAPACITY - 1]);
    println!("avec Elapsed time = {:?}", duration);

    println!("Getting widget v[0] s state: {:?}", v[0].get_state_s());
    println!("Getting widget v[0] s state: {:?}", av[0].get_state_s());
}

mod builder {
    use arrayvec::ArrayVec;

    use crate::widget::{Config, State};

    pub fn vec_builder(v: &mut Vec<State>) {
        (0..crate::CAPACITY).for_each(|_| {
            let w = Config { a: 123, b: 456 };
            let state = State::new(w);
            v.push(state);
        });
    }

    pub fn avec_builder(av: &mut ArrayVec<State, { crate::CAPACITY }>) {
        (0..crate::CAPACITY).for_each(|_| {
            let w = Config { a: 123, b: 456 };
            let state = State::new(w);
            av.push(state);
        });
    }
}

mod widget {

    #[derive(Default, Debug)]
    pub struct Config {
        pub a: u32,
        pub b: u32,
    }

    #[derive(Default, Debug)]
    pub struct State {
        c: Config,
        s: usize,
        t: u32,
    }

    impl State {
        pub fn new(c: Config) -> Self {
            Self {
                c,
                s: Default::default(),
                t: Default::default(),
            }
        }

        pub fn do_stuff(&mut self) {}
    }

    impl State {
        // Getters and setters
        #[inline]
        pub fn get_state_s(&self) -> usize {
            self.s
        }

        #[inline]
        pub fn set_state_s(&mut self, s: usize) {
            self.s = s;
        }
    }
}
