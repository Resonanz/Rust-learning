### How to align text within a selectable label?

https://github.com/emilk/egui/discussions/3868

### https://docs.rs/egui/0.24.1/egui/struct.Ui.html#method.horizontal

```
ui.horizontal(|ui| {
    ui.label("Same");
    ui.label("row");
});
```
See also ```horizontal_centered``` and ```horizontal_top``` and ```horizontal_wrapped```.

### https://docs.rs/egui/0.24.1/egui/struct.Ui.html#method.vertical

Start a ui with vertical layout. Widgets will be left-justified.

```
ui.vertical(|ui| {
    ui.label("over");
    ui.label("under");
});
```
See also ```ui.vertical_centered``` and ```ui.vertical_centered_justified```
