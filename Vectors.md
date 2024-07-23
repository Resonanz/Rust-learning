# Vectors

The Primegen reviewed a lecture by Bjarne Stroustrup on data structures, specifically vectors versus linked lists.

The takeaway was that vectors are super fast while linked lists are slow, even for e.g. insertions at the beginning of a vector.

This is because CPUs are typically optimized for contiguous operations like moving vector contents, but are not good at following lots of pointers (= cache misses). 

So how fast are vector operation in Rust? Let's timeit.

The following times (per loop) are for VEC_LENGTHs from 2 up to 2_000_000_000.

- VEC_LENGTH = 2: 4.9 us
- VEC_LENGTH = 20: 5.9 us
- VEC_LENGTH = 200: 4.3 us
- VEC_LENGTH = 2_000: 4.4 us
- VEC_LENGTH = 20_000: 7.4 us
- VEC_LENGTH = 200_000: 73 us
- VEC_LENGTH = 2_000_000: 602 us
- VEC_LENGTH = 20_000_000: 8090 us (= 8 ms)
- VEC_LENGTH = 200_000_000: 110_000 us (= 110 ms)
- VEC_LENGTH = 2_000_000_000: 1_060_000 us (= 1.06 s)

Notes
- Dropping the vector takes essentially zero time


use timeit::timeit_loops;

fn main() {
    const VEC_LENGTH: usize = 2_000_000_000;  // 2 MP acA1920-155um camera 8-bit mode: 1920 x 1080 x 1000 frames = 2_073_600_000 bytes = ca. 2 GB

    let sec = timeit_loops!(5, {
        let mut w: Vec<u8> = vec![31; VEC_LENGTH];
        for i in 0..VEC_LENGTH - 1 {
            w[i] = 7;
        }
        println!("{}", w[VEC_LENGTH - 1]);  // Need print otherwise compiler optimizes away the vector creation code

        drop(w);  // Makes no difference to timeit
    });

    println!("Elapsed time = {}", sec);
}
