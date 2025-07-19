* https://docs.rs/arrayvec/latest/arrayvec/

* https://crates.io/crates/arrayvec

I did some testing, comparing std Vecs with the ArrayVec crate. See the getsetters folder on here.

The testing involved adding a struct containing 3 items: 2x u32s + 1x struct containing which also contained 2 x u32. So a struct contains another struct.

Vec/ArrayVec size                 ArrayVec faster by about
8                                 8x
16                                25x then 4-5x after that
64                                10-30x
256                               10-12x
1000                              5x
10000                             2x (quite consistently)

The cool thing about ArrayVec is that the methods are largely the same as for standard vectors. e.g. as_ptr, len, capacity
