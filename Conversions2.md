## Convert an array of u8 ascii numbers to a single u32

This code was for serialport-rs for reading a u32 number that had been converted to ascii. I needed to convert it back to u32.

First the port is read into a buffer:
```
let mut buf = [0u8; 1000];
let n = port.read(&mut buf[..])?;
```
The LF character (dec 10) terminates the string of data so we trim it off the end and convert to vec for ease of use:
```
let buf_trimmed = buf[..n - 1].to_vec();
```
Using an iterator that is reversed and thus begins at the end, and an enumerator, we use map to multiply each character (minus 48 to convert from ascii to utf8) times 10 to the power of the enumerator and summing.
```
let r: usize = buf[..n - 1]
    .iter()
    .rev()
    .enumerate()
    .map(|(x, y)| usize::pow(10, x.try_into().unwrap()) * ((*y - 48) as usize))
    .sum();
```


let the_string = str::from_utf8(&buf_trimmed).expect("Not UTF-8.");
let the_number: u32 = the_string.parse().expect("Not a number.");
