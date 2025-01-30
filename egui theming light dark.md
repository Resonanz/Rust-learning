### These provide access to both light/dark system-setting and light/dark egui setting
```
// Get the system-set theme
// Returns Some(Light) or Some(Dark)
let preference = ctx.system_theme();
println!("preference = {:?}", preference);

// Get the egui-set theme
// Returns Light or Dark
let theme = ctx.theme();
println!("theme = {:?}", theme);
```

### Getting theming to work in egui with the light/dark/system buttons
The cool thing about egui theming is that you can define a couple of functions with all the theme definitions in them (one for light and one for dark mode theme) then simply use the built-in light/dark/system triplet and it's job done!

This is how to set the themes in egui...
```
impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Set custom themes
        cc.egui_ctx.style_mut_of(egui::Theme::Light, use_latte);
        cc.egui_ctx.style_mut_of(egui::Theme::Dark, use_mocha);
```
The "use_latte" is a closure call I think to the ```use_latte``` function: ```style_mut_of(egui::Theme::Light, use_latte)```

And the theme functions look like this...

```pub fn use_latte(style: &mut Style) {
    // Background colors
    style.visuals.panel_fill = Color32::from_rgb(239, 241, 245); // Soft cream-white
    style.visuals.window_fill = Color32::from_rgb(239, 241, 245); // Slightly lighter than panel
    style.visuals.extreme_bg_color = Color32::from_rgb(230, 233, 239); // Softest background
...snip
```


### Set global
```
ctx.set_visuals(Visuals::dark());
```
### Set using buttons
```
menu::bar(ui, |ui| {
    widgets::global_theme_preference_buttons(ui);  // Three buttons
    widgets::global_theme_preference_switch(ui);  // Single icon
});
```

### Test for light or dark mode
```
match ctx.theme() {
    egui::Theme::Dark => todo!(),
    egui::Theme::Light => todo!(),
}),
```
