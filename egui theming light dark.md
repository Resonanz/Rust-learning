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
if response.hovered() {
    if ui.visuals().dark_mode {
        fill = self.color_dark_hover;
    } else {
        fill = self.color_light_hover;
    }
} else {
    if true {
        fill = self.color_dark;
    } else {
        fill = self.color_light;
    }
}
```

### Text and background and foreground colors

```
Something like this to begin --> let visuals = ui.style().interact(&response);

then

visuals.text_color()
visuals.bg_fill
visuals.bg_stroke
etc...
```
Another example:
```
let painter = ui.painter();

painter.add(epaint::CircleShape {
    center: big_icon_rect.center(),
    radius: big_icon_rect.width() / 2.0 + visuals.expansion,
    fill: visuals.bg_fill,
    stroke: visuals.bg_stroke,
});
```

## Basic starter for theming

```
if ui.visuals().dark_mode {
    ctx.set_visuals_of(
        egui::Theme::Dark,
        egui::Visuals {
            panel_fill: egui::Color32::DARK_RED,
            ..Default::default()
        },
    );
} else {
    ctx.set_visuals_of(
        egui::Theme::Dark,
        egui::Visuals {
            panel_fill: egui::Color32::LIGHT_RED,
            ..Default::default()
        },
    );
}
```
