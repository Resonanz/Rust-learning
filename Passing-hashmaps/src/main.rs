use std::collections::HashMap;

use texicon::{TexiStruct, Texicon};
// use texicon::TexiStruct;

fn main() {
    let texistruct = TexiStruct {
        shared: &mut texicon::Enum::Off,
        vec: vec![4, 5, 6],
        s: "me is a string".to_string(),
    };

    let mut hm: HashMap<String, TexiStruct> = HashMap::new();
    hm.insert("a".to_string(), texistruct);

    let result = Texicon::new(&mut hm);
    println!("result from new() = {:?}", result.texiref);

    texicon::Texicon::aggregate(hm);
    println!("result from aggregate(hm) = {:?}", result.texiref);
}

//
// ======================================================================
// ======================================================================
// ======================================================================
//

mod texicon {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub enum Enum {
        On,
        Off,
    }

    pub struct TexiStruct<'a> {
        pub shared: &'a mut Enum,
        pub vec: Vec<u32>,
        pub s: String,
    }

    // impl<'a> TexiStruct<'a> {
    //     pub fn new(val: &'a mut u32) -> Self {
    //         Self {
    //             shared: val,
    //             vec: vec![1, 2, 3],
    //             s: "Hi there".to_owned(),
    //         }
    //     }
    // }
    //
    // ======================================================================
    // ======================================================================
    // ======================================================================
    //

    pub struct Texicon<'a> {
        pub texiref: &'a mut Enum,
    }

    impl<'a> Texicon<'a> {
        pub fn new(hm: &'a mut HashMap<String, TexiStruct>) -> Self {
            Self {
                texiref: hm.get_mut("a").unwrap().shared,
            }
        }

        pub fn aggregate(mut hm: HashMap<String, TexiStruct>) {
            let temp = hm.get_mut("a").unwrap();
            println!("temp.s = {}", temp.s);
            *temp.shared = Enum::On;
        }
    }
}
