use super::app;
use super::blar;
use super::usb_comms::pkt;

pub struct TemplateBlar {
    pub item: u32,
}

pub fn blarfn() {
    let _ = app::TemplateApp { item: 345 };
    let _ = blar::TemplateBlar { item: 345 };
    let _ = pkt::PktStruct { item: 345 };
}
