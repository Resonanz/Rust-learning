```
// Loading images as follows bakes the raw image data into the binary file
// See: https://docs.rs/egui/latest/egui/macro.include_image.html
// Notes about include_image:
//   It produces an ImageSource which can be used directly in Ui::image or Image::new:
//   ImageSource is an enum with a Texture(SizedTexture) variant

const IMG_CAMERA: ImageSource<'_> = include_image!("../assets/icon-256.png");
const IMG_ICON: &[u8] = include_bytes!("../assets/icon-256.png");
```
