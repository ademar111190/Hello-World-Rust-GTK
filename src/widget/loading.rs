use gtk::{Box, Label, Spinner};
use gtk::Orientation::Vertical;
use gtk::prelude::*;

use crate::css::apply_style;
use crate::css::Style;

pub struct Loading {
    pub widget: Box,
}

impl Loading {
    pub fn new(text: String) -> Loading {
        let spinner = Spinner::new();
        apply_style(&spinner, Style::LoadingSpinner);
        spinner.start();

        let label = Label::new(Some(&text));
        apply_style(&label, Style::LoadingLabel);

        let root = Box::new(Vertical, 4);
        root.pack_start(&spinner, false, false, 0);
        root.pack_start(&label, false, false, 0);

        return Loading {
            widget: root,
        };
    }
}
