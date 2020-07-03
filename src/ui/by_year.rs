extern crate gio;
extern crate gtk;

use gtk::Box;

use crate::data::strings::get_string;
use crate::data::strings::StringId::{HomeLoading, StackYear};
use crate::widget::center_box::CenterBox;
use crate::widget::loading::Loading;

pub struct ByYear {
    pub widget: Box,
    pub tag: String,
    pub title: String,
}

pub fn build_by_year() -> ByYear {
    let loading = Loading::new(get_string(HomeLoading));
    let root = CenterBox::new(loading.widget);
    return ByYear {
        widget: root.widget,
        tag: String::from("StackYear"),
        title: get_string(StackYear),
    };
}
