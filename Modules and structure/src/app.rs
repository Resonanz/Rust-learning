pub use super::blar;
use super::usb_comms;

pub struct TemplateApp {
    pub item: u32,
}

pub fn appblarblar() {
    let _ = blar::TemplateBlar { item: 345 };
    let _ = usb_comms::pkt::PktStruct { item: 345 };
}
