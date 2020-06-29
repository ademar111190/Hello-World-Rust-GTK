use gtk::Box;
use gtk::Orientation::Vertical;
use gtk::prelude::*;

pub struct CenterBox {
    pub widget: Box,
}

impl CenterBox {
    pub fn new(widget: Box) -> CenterBox {
        let root = Box::new(Vertical, 0);
        add_vertical_spacing(&root);
        root.pack_start(&widget, false, false, 0);
        add_vertical_spacing(&root);
        return CenterBox {
            widget: root
        };
    }
}

fn add_vertical_spacing(root: &Box) {
    root.pack_start(&Box::new(Vertical, 0), true, false, 0);
}
