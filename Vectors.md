# Vectors

The Primegen reviewed a lecture by Bjarne Stroustrup on data structures, specifically vectors versus linked lists.

The takeaway was that vectors are super fast while linked lists are slow, even for e.g. insertions at the beginning of a vector.

This is because CPUs are typically optimized for contiguous operations like moving vector contents, but are not good at following lots of pointers (= cache misses). 

So how fast are vector operation in Rust? Let's timeit.

In the following times (per loop) the VEC_LENGTHs range from 2 up to 2_000_000_000.

Notes
- Dropping and for-loop filling the vector takes essentially zero time

## Code for creating a vector and for-loop filling

```
use timeit::timeit_loops;

fn main() {
    const VEC_LENGTH: usize = 2_000_000_000;  // 2 MP acA1920-155um camera 8-bit mode: 1920 x 1080 x 1000 frames = 2_073_600_000 bytes = ca. 2 GB

    let sec = timeit_loops!(50, {
        let mut w: Vec<u8> = vec![31; VEC_LENGTH];
        for i in 0..VEC_LENGTH {  // This loop appears to take a tiny fraction of the 
            w[i] = 7;             // the time that is required to create the vec![]
        }
        println!("{}", w[VEC_LENGTH - 1]);  // Need print otherwise compiler optimizes away the vector creation code
        drop(w);  // Makes no obvious difference to timeit
    });
    println!("Elapsed time = {}", sec);
}
```

## Code for creating a vector and for-loop filling then inserting and deleting an element at the start of the vector

```
use timeit::timeit_loops;

fn main() {
    const VEC_LENGTH: usize = 2_000_000_000;  // 2 MP acA1920-155um camera 8-bit mode: 1920 x 1080 x 1000 frames = 2_073_600_000 bytes = ca. 2 GB

    let mut r = 0;

    let sec = timeit_loops!(50, {
        let mut w: Vec<u8> = vec![31; VEC_LENGTH];
        for i in 0..VEC_LENGTH {  // This loop appears to take a tiny fraction of the 
            w[i] = 7;             // the time that is required to create the vec![]
        }
        println!("{}", w[VEC_LENGTH - 1]);  // Need print otherwise compiler optimizes away the vector creation code
        w.insert(3, 123);
        r = w.remove(3);
        drop(w);  // Makes no obvious difference to timeit
    });
    println!("Elapsed time = {}, r = {}", sec, r);
}

```
## Plots of vector size versus creation / insertion / deletion

The two plots are using the same data sets. The first is linear-linear, the second is log-log. The blue trace illustrates vector creation/filling, then re-filling using a for loop. The red trace includes the times requires to insert and remove a single element at position 3 in the vector.

![Vector length (lin)](https://github.com/user-attachments/assets/2a9281a1-fc05-4a31-bae5-2ad98f9114e9)
![Vector length (log)](https://github.com/user-attachments/assets/e770ff67-cbf3-44d4-91ee-f1898cafe1cf)
