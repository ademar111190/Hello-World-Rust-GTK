use gtk::{Box, BoxBuilder, Orientation, Spinner, SpinnerBuilder};
use gtk::prelude::*;

use crate::css::apply_style;
use crate::css::Style;
use crate::widget::label::build_label;

pub struct Loading {
    pub widget: Box,
}

impl Loading {
    pub fn new(text: String) -> Loading {
        let spinner = spinner();
        apply_style(&spinner, Style::LoadingSpinner);
        spinner.start();

        let label = build_label(text);
        apply_style(&label, Style::LoadingLabel);

        let root = root();
        root.pack_start(&spinner, false, false, 0);
        root.pack_start(&label, false, false, 0);

        return Loading {
            widget: root,
        };
    }
}

fn spinner() -> Spinner {
    return SpinnerBuilder::new()
        .build();
}

fn root() -> Box {
    return BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .spacing(4)
        .build();
}
