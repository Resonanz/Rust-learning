This is a nicely licensed font source (made by SIL I think, https://openfontlicense.org/ofl-fonts/):

https://fontlibrary.org/en

### Get font size

```
// Get current font details
let style = ctx.style();

let font_size = style.text_styles[&TextStyle::Body].size;
println!("font_size = {font_size}");

Alternatively use the BTreeMap K,V pairs to access:

// Access the BTreeMap
let font = default_text_styles();
// Pull out the K,V value
let font_style = font.get(&TextStyle::Body);

let font_family = font_style.unwrap().family.clone();
let font_size = font_style.unwrap().size;
```
