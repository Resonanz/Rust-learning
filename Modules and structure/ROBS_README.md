# Rust egui eframe project structure

src
src/main.rs
src/lib.rs
src/app.rs
src/blar.rs
src/usb_comms
src/usb_comms/mod.rs
src/usb_comms/comms.rs
src/usb_comms/pkt.rs

## BEWARE

This will allow main.rs to access app.rs:

pub mod app;

but this won't:

mod app;

## Do we need main.rs or lib.rs

Yes: according to Rust, "either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present"

## main.rs

main CANNOT access lib.rs using

use paths::lib;

but CAN access other modules such as app.rs and blar.rs (at the same level) and usb_comms.rs which is a folder.

These are accessed using "paths::etc" which is the Cargo.toml 'name = "paths"' entry.

use paths::app::blarblar;
use paths::blar;
use paths::usb_comms;

TODO Can main access via super?

## lib.rs

Exposes modules that we can thereafter access in modules using "use".

pub mod app;
pub mod blar;
pub mod usb_comms;

## app.rs

app.rs is on the same level as blar.rs, but cannot access blar.rs using:

use blar                    // fails
pub use blar                // fails
use super::blar;            // works (super:: goes up a level)
pub use super::blar;        // works
use path::blar;             // fails
pub use path::blar;         // fails

use super::usb_comms;       // works to access the folder containing mod.rs and other files

Access to the folder also allows access into module pkt via mod.rs e.g.

let _ = usb_comms::pkt::PktStruct { item: 345 };

## mod.rs

Provides access into these modules within the usb_comms folder:

pub mod comms;
pub mod pkt;
