## Display additional fonts

NOTE to self: check if this works: https://github.com/emilk/egui/issues/2188 

This was helpful for adding the BTreeMap:

* https://github.com/emilk/egui/discussions/4449
* https://github.com/markusdd/rusty_meter

Several steps are required:

1. First, insert the new font using ```fonts.font_data.insert(...)```
2. Second, create a new BTreeMap (std:collections::BTreeMap) and insert the new font into the FontFamily, providing font name and font bytes: ```let mut newfam = BTreeMap::new()...```. This data structure is then appended to ```fonts.families```.
3. Third, push my new font (NotoSans...) into the font list(?).
4. Fourth, use the new font on a label as shown below.

```
fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    ...

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert(
        "NotoSansSymbols2-Regular".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../fonts/NotoSansSymbols2-Regular.ttf"
        )),
    );

    // ============================================

    let mut newfam = BTreeMap::new();
    newfam.insert(
        FontFamily::Name("NotoSansSymbols2-Regular".into()),
        vec!["NotoSansSymbols2-Regular".to_owned()],
    );
    fonts.families.append(&mut newfam);

    // ============================================

    // Push my font somewhere:
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .push("NotoSansSymbols2-Regular".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

// ============================================

ui.label(RichText::new("🯰🯱🯲🯳🯴🯵🯶🯷🯸🯹")
    .color(Color32::LIGHT_GREEN)
    .font(FontId {
        size:self.value * 5.0,
        family: FontFamily::Name("NotoSansSymbols2-Regular".into()),
    })
);
```
## Overriding text inside a temporary [`Ui`] (scope)

```
// A `scope` creates a temporary [`Ui`] in which you can change settings:
ui.scope(|ui| {
    ui.visuals_mut().override_text_color = Some(Color32::RED);
    ui.style_mut().override_text_style = Some(TextStyle::Monospace);
    // Text wrapping or truncation...
    //ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);

    ui.label("This text will be red, monospace, and won't wrap to a new line");
}); // the temporary settings are reverted here
```

## Grouping some text (with outline)

```
ui.group(|ui| {
    ui.label("Within a frame");
    ui.label("Within a frame but longer");
    ui.set_min_height(20.0);
});
``` 
