## Display additional fonts

This was helpful for adding the BTreeMap:

* https://github.com/emilk/egui/discussions/4449

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
