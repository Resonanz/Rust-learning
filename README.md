# Rust-learning code snippets

# Print macros

## Formatters
If the trait std::fmt::Display is not implemented then {:?} or {:#?} for pretty print may be required

```println!("This is some text with a formatter {:#?}, i);```

## Delay example
```
use std::{thread, time};
thread::sleep(time::Duration::from_millis(500));
```
## Zeroing an array of bytes
These all generate identical code in [GodBolt](https://rust.godbolt.org) (Rust 1.75.0):
```
pub fn clear(array: &mut [u8]) {
    array.iter_mut().for_each(|m| *m = 0)
}
```
```
pub fn clear(array: &mut [u8]) {
        for i in 0..array.len() { array[i] = 0; }
}
```
```
pub fn clear(array: &mut [u8]) {
        for i in array { *i = 0; }
}
```
Output:
```
 example::clear:
        test    rsi, rsi
        je      .LBB0_1
        mov     rdx, rsi
        xor     esi, esi
        jmp     qword ptr [rip + memset@GOTPCREL]
.LBB0_1:
        ret
```

# egui

## Make a UI button that is a small as the text it contains
```
if ui.small_button("mmmmmmmm").clicked() {
    self.value += 0.0;
};
```

## Add extra space before the next widget
```
ui.add_space(10.0);
```

# Enums

## A simple enum

```
#[derive(Debug)]  // Add Debug trait for Animals
enum Animals {
    Cat,
    Dog,
}
```
The Debug trait is required for printing the enum:
```    println!("Animal = {:#?}", i);```
