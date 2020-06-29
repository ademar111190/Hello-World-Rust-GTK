use gtk::{Box, Label, Spinner};
use gtk::Orientation::Vertical;
use gtk::prelude::*;

pub struct Loading {
    pub widget: Box,
}

impl Loading {
    pub fn new(text: String) -> Loading {
        let spinner = Spinner::new();
        spinner.start();

        let label = Label::new(Some(&text));

        let root = Box::new(Vertical, 4);
        root.pack_start(&spinner, false, false, 0);
        root.pack_start(&label, false, false, 0);

        return Loading {
            widget: root,
        };
    }
}
