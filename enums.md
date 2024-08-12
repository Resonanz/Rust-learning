## Enums

### A simple enum:

```
#[derive(Debug)]  // Add Debug trait for Animals
enum Animals {
    Cat,
    Dog,
}
```
The Debug trait is required for printing the enum:
```
println!("Animal = {:#?}", i);
```


### Pattern matching a struct tuple
```
struct MyStruct {
    a: u8,
    b: u8,
}

let p = MyStruct { a: 22, b: 33 };

match (p.a, p.b) {
    (22, 33) => println!("Found the tuple!!!"),
    _ => println!("Tuple not found :-("),
}
```
### Pattern matching against enum data
Find the match... extract the enum data !
```
enum EnumWithData {
    A(u8, u32),
    B(u8, u32),
}

fn main() {
    let e = EnumWithData::A(55, 67);
    match e {
        EnumWithData::A(x,y) => println!("Wow, found {}, {}", x, y),
        _ => println!("No match found"),
    }
```

Apparently Rust 1.65.0 brings a new ```let-else``` method for accessing a single enum variant as follows:

(https://stackoverflow.com/questions/9109872/how-do-you-access-enum-values-in-rust)

```
#[derive(Debug)]
enum Pet {
    Cat(u8),
    Dog(u8),
}

fn main() {   
    let pc = Pet::Cat(4);

    let Pet::Cat(x) = pc else {return};  // Access the variant. 
                                         // If pc cannot be destructured
                                         // into Cat then return
    println!("The cat has {} legs.", x)
}
```

A very nice example of pattern matching enums:

(https://dev.to/aniket_botre/day-13-rust-enums-unleashing-the-power-of-variants-jli)

```
enum IpAddr {
    V4(String),  // The variants carry additional data !!!
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

// Match the enum variant and extract the additional data
match home {
    IpAddr::V4(addr) => println!("IPv4: {}", addr),
    IpAddr::V6(addr) => println!("IPv6: {}", addr),
}
```

Adding methods to enums:

(https://dev.to/aniket_botre/day-13-rust-enums-unleashing-the-power-of-variants-jli)

```
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 30,
        }
    }
}

let red = TrafficLight::Red;
println!("Red light for {} seconds", red.time());
// Output: Red light for 60 seconds
```
