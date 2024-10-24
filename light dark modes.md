### Set global
```
ctx.set_visuals(Visuals::dark());
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
