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

### Drawing lines

Both of the following go inside a cloure (inside a widget in my case):

```
ui.allocate_ui_with_layout(size, layout, |ui| {
```

#### Shape::line

Combining many ```Pos2``` points into a vector then calling ```Shape::line```
```
let locx = rect.min[0];
let locy = rect.min[1];

let mut v = Vec::new();
v.push(Pos2 {
    x: locx + 0.,
    y: locy + 0.,
});
v.push(Pos2 {
    x: locx + 70.,
    y: locy + 0.,
});
v.push(Pos2 {
    x: locx + 70.,
    y: locy + 70.,
});
v.push(Pos2 {
    x: locx + 0.,
    y: locy + 70.,
});
v.push(Pos2 {
    x: locx + 0.,
    y: locy + 0.,
});

ui.painter().add(Shape::line(v, (1., Color32::LIGHT_BLUE)));
```

#### line_segment (draws a single line rather than a series of lines from a vector)

```
let locx = rect.min[0];
let locy = rect.min[1];

let arr1 = [
    Pos2 {
        x: locx + 35.,
        y: locy + 0.,
    },
    Pos2 {
        x: locx + 0.,
        y: locy + 70.,
    },
];
let arr2 = [
    Pos2 {
        x: locx + 0.,
        y: locy + 70.,
    },
    Pos2 {
        x: locx + 70.,
        y: locy + 70.,
    },
];
let arr3 = [
    Pos2 {
        x: locx + 70.,
        y: locy + 70.,
    },
    Pos2 {
        x: locx + 35.,
        y: locy + 0.,
    },
];

ui.painter().line_segment(arr1, (1., Color32::LIGHT_GRAY));
ui.painter().line_segment(arr2, (1., Color32::LIGHT_GRAY));
ui.painter().line_segment(arr3, (1., Color32::LIGHT_GRAY));
```

## egui sub-window title font size

The following changes the heading font size that is used for the sub-window title.
```
let mut style: egui::Style = (*ctx.style()).clone();

style.text_styles.insert(
    egui::TextStyle::Heading,
    FontId::new(14.0, egui::FontFamily::Proportional), // Set the font size to 14.0
);

// Apply the new style
ctx.set_style(style);

egui::Window::new("THE TITLE").show(ctx, |ui| {
    ui.label("This window has a custom title font size.");
    ui.label(egui::RichText::new("I'm a heading").heading());
});
```
