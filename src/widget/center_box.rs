use gtk::{Box, Orientation};
use gtk::prelude::*;

pub struct CenterBox {
    pub widget: Box,
}

impl CenterBox {
    pub fn new(widget: Box, orientation: Orientation) -> CenterBox {
        let root = Box::new(orientation, 0);
        add_vertical_spacing(&root, orientation);
        root.pack_start(&widget, false, false, 0);
        add_vertical_spacing(&root, orientation);
        return CenterBox {
            widget: root
        };
    }
}

fn add_vertical_spacing(root: &Box, orientation: Orientation) {
    root.pack_start(&Box::new(orientation, 0), true, false, 0);
}
