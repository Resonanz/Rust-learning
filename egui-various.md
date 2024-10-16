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

### Spinner widget

The simplest method foradding a spinner widget is super simple. According to the docs, "If the size isn't set explicitly, the active style's `interact_size` is used."


The second method is more complex and adds a size through the constructor.

The third method allocates space on the canvas I guess, then adds the widget.
```
ui.spinner();

ui.add(egui::Spinner::new().size(50.));

ui.add_sized([50., 50.], egui::Spinner::new().size(50.));
```
