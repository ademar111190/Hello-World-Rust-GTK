extern crate gio;
extern crate gtk;

use std::fs::File;
use std::io::Read;
use std::thread;

use gtk::{Box, Orientation};

use crate::data::strings::get_string;
use crate::data::strings::StringId::{HomeLoading, StackYear};
use crate::data::team::build_teams_payload;
use crate::widget::center_box::CenterBox;
use crate::widget::loading::Loading;

pub struct ByYear {
    pub widget: Box,
    pub tag: String,
    pub title: String,
}

pub fn build_by_year() -> ByYear {
    let loading = Loading::new(get_string(HomeLoading));
    let root = CenterBox::new(loading.widget, Orientation::Vertical);

    thread::spawn(|| {
        read_json();
    });

    return ByYear {
        widget: root.widget,
        tag: String::from("StackYear"),
        title: get_string(StackYear),
    };
}

fn read_json() {
    let mut file = File::open("assets/byTeam.json")
        .expect("Failed to open the byTeam.json file");
    let mut json = String::new();
    file.read_to_string(&mut json)
        .expect("Failed to read the byTeam.json content");
    let teams_payload = build_teams_payload(&json);
    println!("> {:?}", teams_payload);
}
