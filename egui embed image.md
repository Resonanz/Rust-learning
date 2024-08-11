# How to embed an image into a egui frame

Note that to show images, it is required to load the loaders from egui_extras:

```
egui_extras = { version = "0.28", features = ["default", "all_loaders", "serde"] }
# serde should not be here -- it fixes a bug in the .0 release
```

and add egui_extras into main.rs

```
eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);   <<< add here
            Ok(Box::new(eframe_template::TemplateApp::new(cc)))
        }),
    )
```
## Add image from memory

Now that the image loaders are present, let's insert an image from memory.

First we create a Vec of length 40*40 and fill it with decimal 100 (a grayscale value between 0 and 255):

```
/* ======================================================================================= */

// WORKS !!!
// Add a slider to set the grayscale
ui.add(egui::Slider::new(&mut self.slider_val, 0..=255));

/* ======================================================================================= */
```
```
let v: Vec<u8> = vec![self.slider_val; 40 * 40];
let ci = ColorImage::from_gray([40,40], &v);
```
Then we use the vector data to "load" the texture data, give it a TextureId and size it, then display the image:
```
let tex: TextureHandle = ctx.load_texture("tex name", ci, Default::default());
let tex_id: TextureId = TextureId::from(&tex);
let st: SizedTexture = SizedTexture {
    id: tex_id,
    size: Vec2::new(40., 40.),
};
let b: ImageSource = ImageSource::Texture(st);
ui.add(egui::Image::new(b));
```
Alternatives ColorImages could be:
```
let ci = egui::ColorImage::example();  // Created in egui in HSVA format (128x64 pixels))
```
and
```
let ci = ColorImage::new([40,40], Color32::RED);
```
