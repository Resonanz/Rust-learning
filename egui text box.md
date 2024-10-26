## Capture focus

```
ui.horizontal(|ui| {
    ui.add(Label::new("Write somefing: ").selectable(false));
    ui.text_edit_singleline(&mut self.label).request_focus();
});
```

## This is cool for capturing loss of focus

```pub fn lost_focus(&self) -> bool```

The widget had keyboard focus and lost it, either because the user pressed tab or clicked somewhere else, or (in case of a crate::TextEdit) because the user pressed enter.

```
let response = ui.text_edit_singleline(&mut my_text);
if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
    do_request(&my_text);
}
```
