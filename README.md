# Rust-learning code snippets

## Keeping Rust up-to-date

To check rustup and rustc versions: ```rustup --version```

To update to latest: ```rustup update```






## Timing code using timeit

Add to cargo.toml:

```
[dependencies]
timeit = "0.1.2"
```
Then add the timing code where the first macro parameter is the number of loops to run:
```
// TIMING LOOP
let sec = timeit::timeit_loops!(1, { /* CODE TO TEST HERE */ });
println!("Time taken was: {} s", sec);
```







## Print macros

### Formatters
If the trait std::fmt::Display is not implemented then {:?} or {:#?} for pretty print may be required

```println!("This is some text with a formatter {:#?}, i);```








## Adding a delay
```
use std::{thread, time};
thread::sleep(time::Duration::from_millis(500));
```






## const TUPLES

```
const SUPLE: [u32; 2] = [1, 2]; 

fn suple() {
    let s = SUPLE[0];  // Legit
}

const TUPLE: [(u32, u32); 2] = [(1,2), (3,4)];

fn tuple() {
    let t = TUPLE[0].1;  // Legit
}

const TUPLE: [(&str, &str); 2] = [("abc", "def"), ("ghi", "jkl")];

pub fn tuple() {
    let t = TUPLE[0].1;
    print!("{t}");
}

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














## egui

### Make a UI button that is a small as the text it contains
```
if ui.small_button("mmmmmmmm").clicked() {
    self.value += 0.0;
};
```

### Add extra space before the next widget
```
ui.add_space(10.0);
```



### Draw a circle on a window

```
let center1 = pos2(100.0,100.0);  // pos2 = a struct
let center2 = pos2(100.0,200.0);
let stroke = Stroke::new(3.0, Color32::LIGHT_GREEN);

let circle1 = CircleShape::filled(center1, self.value, Color32::RED);
ui.painter().add(circle1);

let circle2 = CircleShape::stroke(center2, self.value, stroke);
ui.painter().add(circle2);

```






## Display an SVG image

It is essential to add the image loader to cargo.toml as follows:

```
impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        ...
        egui_extras::install_image_loaders(&cc.egui_ctx);  // <--- Install image loaders
        Default::default()
    }
}
```

It is also essential to update cargo.toml dependencies with egui_extras as follows:

```
[dependencies]
egui_extras = { features = ["default", "all_loaders"] }
```

With this in place, SVG images can be displayed using one of these two:

```
ui.image(egui::include_image!("../assets/test-tube.svg"));
ui.add(egui::Image::new(include_image!("../assets/test-tube.svg"))
    .fit_to_exact_size(Vec2::new(100.0, 100.0)));

```

These could be wrapped in a ScrollArea:

```
egui::ScrollArea::both().max_height(400.0).show(ui, |ui| {
    ui.add(egui::Image::new(egui::include_image!("../assets/test-tube.svg"))
        .fit_to_exact_size(Vec2::new(200.0, 100.0)));
    ui.add(egui::Image::new(egui::include_image!("../assets/ferris.svg"))
        .fit_to_exact_size(Vec2::new(200.0, 100.0)));
    ui.add(egui::Image::new(egui::include_image!("../assets/ferris.svg"))
        .fit_to_exact_size(Vec2::new(200.0, 100.0)));
});
```





















## Looping

* https://doc.rust-lang.org/core/ops/struct.Range.html#method.step_by
* This is a core/std library iterator method

```
for x in (1..10).step_by(2) {
    println!("{}", x);
}
```
