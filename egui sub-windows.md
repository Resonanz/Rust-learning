## egui sub-windows titles

Sub-window titles can be defined using ```egui::RichText``` which changes the size of the title area:

```
egui::Window::new(egui::RichText::from("Cool window1").size(16.)).show(ctx, |ui| {});
egui::Window::new(egui::RichText::from("Cool window2").size(14.)).show(ctx, |ui| {});
egui::Window::new(egui::RichText::from("Cool window3").size(12.)).show(ctx, |ui| {});
```
