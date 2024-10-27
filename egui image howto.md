# How to add images (PNG, SVG, etc.) into egui

Assuming that you are using **eframe_template** (https://github.com/emilk/eframe_template)...

### Step 1

Add the image "loaders" into ```cargo.toml``` under [dependencies]. This is needed to load the images.

```
[dependencies]
egui_extras = { version = "0.29.1", features = ["default", "all_loaders"] }
```

### Step 2

In ```main.rs``` install the image loaders. I should look something like this:

```
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);  <<<<<<<<<<<<< install image loaders
            Ok(Box::new(Texicons::TemplateApp::new(cc)))
        }),
    )
```

### Step 3

In ```app.rs``` add ```use egui::include_image``` to access the ```include_image``` macro.  You can then use the macro to load the image into (for example) a ```const``` as follow:

```
const MY_IMAGE_NAME: ImageSource<'_> = include_image!("../assets/icon-256.png");

```

### Step 4

In the following I use the image as a button that can be clicked or hovered. 

```
// Button 1
let b = ui.add(
    egui::Image::new(MY_IMAGE_NAME)
        .tint(self.data.button1.get_tint())
        .fit_to_exact_size(Vec2::new(32., 32.))
        .sense(Sense::click()),
);

if b.clicked() {
    self.data.button1.set_action(ButtonAction::Clicked);
} else if b.hovered() {
    self.data.button1.set_action(ButtonAction::Hover);
} else {
    self.data.button1.set_action(ButtonAction::NoAction);
}
```
