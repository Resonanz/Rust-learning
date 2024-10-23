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
