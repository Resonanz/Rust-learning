The Primegen reviewed a lecture by Bjarne Stroustrup on data structures, specifically vectors versus linked lists.

The takeaway was that vectors are super fast while linked lists are slow, even for e.g. insertions at the beginning of a vector.

This is because CPUs are typically optimized for contiguous operations like moving vector contents, but are not good at following lots of pointers (= cache misses). 

So how fast are vector operation in Rust? Let's timeit.

The following times are for variations of VEC_LENGTH from 2 up to 2_000_000_000.

- asdf

use timeit::timeit_loops;

fn main() {
    const VEC_LENGTH: usize = 2_000_000_000;

    let sec = timeit_loops!(5, {
        let mut w: Vec<u8> = vec![31; VEC_LENGTH];
        for i in 0..VEC_LENGTH - 1 {
            w[i] = 7;
        }
        println!("{}", w[VEC_LENGTH - 1]);

        drop(w);  // Makes no difference to timeit
    });

    println!("Elapsed time = {}", sec);
}
