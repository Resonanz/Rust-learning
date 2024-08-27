### Read 4 u8 bytes from a slice in little endian format and convert to u32 big endian format

The 4 bytes could ORed or added.

```
fn convert_le_to_be(array: &[u8]) -> u32 {
    ((array[3] as u32) << 24) | 
    ((array[2] as u32) << 16) | 
    ((array[1] as u32) << 8)  | 
    ((array[0] as u32) << 0)
}
```

### Convert from u32 to 4 bytes
```
fn u32_to_byte_array(&self, val: u32) -> [u8; 4] {
    let res7to0 = (val & 0x000000FF) as u8;
    let res15to8 = (val & 0x000000FF) as u8;
    let res23to16 = (val & 0x000000FF) as u8;
    let res31to24 = (val & 0x000000FF) as u8;
    [res31to24, res23to16, res15to8, res7to0]
    //
    // Is this nay better than:
    //   let m: u32 = 0x12345678;
    //   let m = m.to_be_bytes();
    //   println!("m = {:?}", m);
    //
    // Could also use a rotate of val_u32
    // u32::rotate_right(val, 8);
}
```
