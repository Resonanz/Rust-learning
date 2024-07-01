# Rust-learning code snippets

## Keeping Rust up-to-date

To check rustup and rustc versions: ```rustup --version```

To update to latest: ```rustup update```


## Compiling

### For size

* https://github.com/johnthagen/min-sized-rust
* https://stackoverflow.com/questions/29008127/why-are-rust-executables-so-huge

Rust binaries even for "Hello, world!" can be large. Quoting: 

    Rust uses static linking to compile its programs, meaning that all libraries required by even the simplest Hello world! program will be compiled into your executable. This also includes the Rust runtime.

I created a Linux executable Rust release using the following in ```cargo.toml```:

```
[profile.release]
opt-level = 'z'     # Optimize for size
codegen-units = 1   # Reduce number of codegen units to increase optimizations
panic = 'abort'     # Abort on panic
strip = true        # Strip symbols from binary*
```

The resultant binary was 342 kB.

### What about compression of the executable

UPX (https://github.com/upx/upx) is recommended to compress the final binary by 50-70%. UPX releases on Github are themselves about 670 kB and cross platform (Win + Linux, no Mac it seems).

I downloaded the Linux version "upx-4.2.4-amd64_linux.tar.xz" (UPX - Linux version, statically linked) and dragged the single binary file "upx" into the root folder of my Rust project. I then used UPX to compress the 342 kB binary down to 139 kB as follows:


```
./upx --best --lzma target/release/my_release_binary
```

### Automating compression

Cargo has no post precessing capability, so... create a bash script: ```touch compile_compress_run.sh``` and make executable ```chmod +x compile_compress_run.sh```. Add the following to the script:

```
#!/bin/bash
cargo build --release
./upx --best --lzma target/release/my_release_binary
target/release/my_release_binary
```

To run the script use ```./compile_compress_run.sh```.





## Cargo automatic re-compile

* https://github.com/watchexec/cargo-watch
* https://stackoverflow.com/questions/29461693/how-can-i-get-cargo-to-recompile-changed-files-automatically

Execute the following in VSCode Terminal to automatically recompile and run upon save:

```
cargo watch -q -c -w src -x run
```

For --release code, use the following:

```
cargo watch -q -c -w src -x 'run -- --release'
```






## Timing code using timeit

Add to cargo.toml:

```
[dependencies]
timeit = "0.1.2"
```
Then add the timing code where the first macro parameter is the nunber of loops to run:
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




## Display additional fonts

This was helpful for adding the BTreeMap:

* https://github.com/emilk/egui/discussions/4449

Several steps are required:

1. First, insert the new font using ```fonts.font_data.insert(...)```
2. Second, create a new BTreeMap (std:collections::BTreeMap) and insert the new font into the FontFamily, providing font name and font bytes: ```let mut newfam = BTreeMap::new()...```. This data structure is then appended to ```fonts.families```.
3. Third, push my new font (NotoSans...) into the font list(?).
4. Fourth, use the new font on a label as shown below.

```
fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    ...

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert(
        "NotoSansSymbols2-Regular".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../fonts/NotoSansSymbols2-Regular.ttf"
        )),
    );

    // ============================================

    let mut newfam = BTreeMap::new();
    newfam.insert(
        FontFamily::Name("NotoSansSymbols2-Regular".into()),
        vec!["NotoSansSymbols2-Regular".to_owned()],
    );
    fonts.families.append(&mut newfam);

    // ============================================

    // Push my font somewhere:
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .push("NotoSansSymbols2-Regular".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

// ============================================

ui.label(RichText::new("🯰🯱🯲🯳🯴🯵🯶🯷🯸🯹")
    .color(Color32::LIGHT_GREEN)
    .font(FontId {
        size:self.value * 5.0,
        family: FontFamily::Name("NotoSansSymbols2-Regular".into()),
    })
);
```
















## Looping

* https://doc.rust-lang.org/core/ops/struct.Range.html#method.step_by
* This is a core/std library iterator method

```
for x in (1..10).step_by(2) {
    println!("{}", x);
}
```














## Enums

### A simple enum:

```
#[derive(Debug)]  // Add Debug trait for Animals
enum Animals {
    Cat,
    Dog,
}
```
The Debug trait is required for printing the enum:
```
println!("Animal = {:#?}", i);
```


### Pattern matching a struct tuple
```
struct MyStruct {
    a: u8,
    b: u8,
}

let p = MyStruct { a: 22, b: 33 };

match (p.a, p.b) {
    (22, 33) => println!("Found the tuple!!!"),
    _ => println!("Tuple not found :-("),
}
```
### Pattern matching against enum data
Find the match... extract the enum data !
```
enum EnumWithData {
    A(u8, u32),
    B(u8, u32),
}

fn main() {
    let e = EnumWithData::A(55, 67);
    match e {
        EnumWithData::A(x,y) => println!("Wow, found {}, {}", x, y),
        _ => println!("No match found"),
    }
```

Apparently Rust 1.65.0 brings a new ```let-else``` method for accessing a single enum variant as follows:

(https://stackoverflow.com/questions/9109872/how-do-you-access-enum-values-in-rust)

```
#[derive(Debug)]
enum Pet {
    Cat(u8),
    Dog(u8),
}

fn main() {   
    let pc = Pet::Cat(4);

    let Pet::Cat(x) = pc else {return};  // Access the variant. 
                                         // If pc cannot be destructured
                                         // into Cat then return
    println!("The cat has {} legs.", x)
}
```

A very nice example of pattern matching enums:

(https://dev.to/aniket_botre/day-13-rust-enums-unleashing-the-power-of-variants-jli)

```
enum IpAddr {
    V4(String),  // The variants carry additional data !!!
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

// Match the enum variant and extract the additional data
match home {
    IpAddr::V4(addr) => println!("IPv4: {}", addr),
    IpAddr::V6(addr) => println!("IPv6: {}", addr),
}
```

Adding methods to enums:

(https://dev.to/aniket_botre/day-13-rust-enums-unleashing-the-power-of-variants-jli)

```
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 30,
        }
    }
}

let red = TrafficLight::Red;
println!("Red light for {} seconds", red.time());
// Output: Red light for 60 seconds
```
