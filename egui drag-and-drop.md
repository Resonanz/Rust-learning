Relies on winit which may not work under wayland: https://github.com/emilk/egui/issues/4105

* egui example: https://github.com/emilk/egui/blob/master/examples/file_dialog/src/main.rs. To install, see also https://github.com/emilk/egui/issues/5205#issuecomment-2387694303.
* https://rardiol.gitlab.io/BroaNetCo/eframe/struct.NativeOptions.html#structfield.drag_and_drop_support
* https://docs.rs/egui/latest/egui/struct.RawInput.html#structfield.dropped_files
* https://docs.rs/egui/latest/src/egui/data/input.rs.html#358-373

```
let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_drag_and_drop(true) <-------------------
            ...etc
```
The following is similar to the egui example. The egui example would be preferable to follow.
```
// Capture the files dropped this frame
if !&ctx.input(|i| i.raw.dropped_files.is_empty()) {
    println!("File dropped");
};
```
