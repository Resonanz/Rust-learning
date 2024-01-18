# Rust-learning code snippets

## Delay example
```
use std::{thread, time};
thread::sleep(time::Duration::from_millis(500));
```
## Zeroing a array of bytes
Running this code in [GodBolt](https://rust.godbolt.org) Rust 1.75.0 leads to code shown.
```
pub fn clear(array: &mut [u8]) {
    array.iter_mut().for_each(|m| *m = 0)
}

 example::clear:
        test    rsi, rsi
        je      .LBB0_1
        mov     rdx, rsi
        xor     esi, esi
        jmp     qword ptr [rip + memset@GOTPCREL]
.LBB0_1:
        ret
```
Compare this to the more traditional for-loop:
```
pub fn clear(array: &mut [u8]) {
        for i in 0..array.len() { array[i] = 0; }
}

  example::clear:
        test    rsi, rsi
        je      .LBB0_1
        mov     rdx, rsi
        xor     esi, esi
        jmp     qword ptr [rip + memset@GOTPCREL]
.LBB0_1:
        ret
```
