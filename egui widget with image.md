There is a "Button with Icon" widget in the Widgets demo panel on egui.rs.

Inside ```egui/crates/egui_demo_lib/src/demo/widget_gallery.rs``` on line 212 we find the image being laoded:

```let egui_icon = egui::include_image!("../../data/icon.png");```

Then on line 220 the image is used:

```
if ui.add(egui::Button::image_and_text(egui_icon, "Click me!")).clicked() {
  *boolean = !*boolean;
}
```

```image_and_text``` is defined in ```egui/crates/egui/src/widgets/button.rs```

On line 53:

```
/// Creates a button with an image to the left of the text. The size of the image as displayed is defined by the provided size.
#[allow(clippy::needless_pass_by_value)]
pub fn image_and_text(image: impl Into<Image<'a>>, text: impl Into<WidgetText>) -> Self {
    Self::opt_image_and_text(Some(image.into()), Some(text.into()))
}
```

which call on line 59:

```
pub fn opt_image_and_text(image: Option<Image<'a>>, text: Option<WidgetText>) -> Self {
    Self {
        text,
        image,
        shortcut_text: Default::default(),
        wrap_mode: None,
        fill: None,
        stroke: None,
        sense: Sense::click(),
        small: false,
        frame: None,
        min_size: Vec2::ZERO,
        rounding: None,
        selected: false,
    }
}
```

The Self refers to this struct, which includes image on line 25:

```
pub struct Button<'a> {
    image: Option<Image<'a>>,
    text: Option<WidgetText>,
    shortcut_text: WidgetText,
    wrap_mode: Option<TextWrapMode>,

    /// None means default for interact
    fill: Option<Color32>,
    stroke: Option<Stroke>,
    sense: Sense,
    small: bool,
    frame: Option<bool>,
    min_size: Vec2,
    rounding: Option<Rounding>,
    selected: bool,
}
```

This 
