Assuming that you are using **eframe_template**...

### Step 1

Add the image "loaders" into ```cargo.toml``` under [dependencies]. 

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

In ```app.rs``` add ```use egui::include_image``` to import the crate.  You can then load the image into (for example) a ```const``` as follow:

```
const MY_IMAGE_NAME: ImageSource<'_> = include_image!("../assets/icon-256.png");

```

