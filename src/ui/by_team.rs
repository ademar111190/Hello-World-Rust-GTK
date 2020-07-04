extern crate gio;
extern crate gtk;

use gtk::{Box, Orientation};

use crate::data::strings::get_string;
use crate::data::strings::StringId::{HomeLoading, StackClub};
use crate::widget::center_box::CenterBox;
use crate::widget::loading::Loading;

pub struct ByTeam {
    pub widget: Box,
    pub tag: String,
    pub title: String,
}

pub fn build_by_team() -> ByTeam {
    let loading = Loading::new(get_string(HomeLoading));
    let root = CenterBox::new(loading.widget, Orientation::Vertical);
    return ByTeam {
        widget: root.widget,
        tag: String::from("StackClub"),
        title: get_string(StackClub),
    };
}
