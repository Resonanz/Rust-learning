There is a "Button with Icon" widget in the Widgets demo panel on egui.rs.

**This makes it clear that it is possible to load a PNG and place it on a widget.**

Inside ```egui/crates/egui_demo_lib/src/demo/widget_gallery.rs``` on line 212 we find the image being loaded:

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

This ```struct Button``` declared three functions:

1. ```new``` which creates a text only button: ```Self::opt_image_and_text(None, Some(text.into()))```
2. ```image``` which creates an image only button ```Self::opt_image_and_text(Some(image.into()), None)```
3. ```image_and_text``` which creates a text and image button: ```Self::opt_image_and_text(Some(image.into()), Some(text.into()))```

These three functions define the values used for the Self struct.

Now we get to the implementation on line 178: ```impl Widget for Button<'_>```

* On line 206 ```space_available_for_image``` is calculated
* On line 213 ```image_size``` is calculated
* On line 246 ```desired_size``` is assigned x and y image dimension sizes
* On line 268, now that the combined image + text galley dimensions have been calculated, we finally get to ```let (rect, mut response) = ui.allocate_at_least(desired_size, sense);```
* On line 277 we begin the drawing section of the widget: ```if ui.is_rect_visible(rect) {```
* On line 278 we have the interaction type ```let visuals = ui.style().interact(&response);``` which means "Check for clicks, drags and/or hover on a specific region of this Ui."
* On line 302 the rect is drawn: ```ui.painter().rect(```
* On line 309 the cursor is moved ready to draw the image: ```let mut cursor_x = rect.min.x + button_padding.x;```
* On line 312 we line up the image position: ```let mut image_pos = ui.layout()...```
* On line 319 the image rectangle is defined: ```let image_rect = Rect::from_min_size(image_pos, image_size);```
* On line 321 we "Load the image from its Image::source, returning the resulting SizedTexture." using ```let tlr = image.load_for_size(ui.ctx(), image_size);```
* On line 322 we call ```widgets::image::paint_texture_load_result``` passing in ```ui``` and the image and more
* On line 329 a response is set up but not used? ```response = widgets::image::texture_load_result_response```

  So on line 322 the ```image``` function ```paint_texture_load_result``` is called which in turn calls ```paint_texture_at(ui.painter(), rect, options, texture);```. paint_texture_at is an egui::widgets function which paints a texture inside a rectangle, and in this case takes a helper function ui.painter().

### Works !!!

The following code is my first attempt but does work to display a PNG as a texture.

```
// load_for_size returns enum TextureLoadResult
// enum variant Ok contains TexturePoll
// TexturePoll enum contains variant Ready
// Ready variant contains struct SizedTexture
// SizedTexture struct contains id: TextureId and size:Vec2
// TextureId enum contains variants Managed(u64) and Custom(u64)
//let i = &img;
let tlr = img.load_for_size(ui.ctx(), desired_size); // This stands for "texture load result"
match tlr {
    Ok(TexturePoll::Ready { texture }) => {
        //println!("TexturePoll::Ready !!! texture.id = {:?}", texture.id);
        // paint_texture_at(ui.painter(), rect, options, texture);
        ui.painter().add(RectShape {
            rect,
            rounding: Rounding::ZERO,
            fill: Color32::YELLOW,
            stroke: Stroke::NONE,
            blur_width: 0.0,
            fill_texture_id: texture.id,
            uv: Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)),
            // uv: Rect::ZERO,
        });
    }
    Ok(TexturePoll::Pending { .. }) => {
        //println!("TexturePoll::Pending");
    }
    Err(x) => {
        //println!("tlr Err() {x}");
        let font_id = TextStyle::Body.resolve(ui.style());
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            "⚠",
            font_id,
            ui.visuals().error_fg_color,
        );
    }
}
```
