extern crate gio;
extern crate gtk;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use gtk::{Align, Box, BoxBuilder, Orientation};
use gtk::prelude::*;

use crate::data::strings::get_string;
use crate::data::strings::StringId::{HomeLoading, StackClub};
use crate::data::team::Team;
use crate::widget::loading::Loading;

#[derive(Debug)]
pub enum State {
    Loading { message: String },
    Error { message: String },
    Success { teams: Vec<Team> },
}

pub struct ByTeam {
    pub root: Box,
    pub loading: Box,
    pub error: Box,
    pub success: Box,
    pub tag: String,
    pub title: String,
}

impl ByTeam {
    pub fn render(&self, state: &State) {
        match state {
            State::Loading { message } => println!("Loading {}", message),
            State::Error { message } => println!("Error {}", message),
            State::Success { teams } => println!("Success {:?}", teams),
        }
    }

    pub fn run(&self) -> Receiver<State> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            tx.send(State::Loading {
                message: String::from("A loading message")
            }).unwrap();
            thread::sleep(Duration::from_secs(1));

            tx.send(State::Error {
                message: String::from("An error message")
            }).unwrap();
            thread::sleep(Duration::from_secs(1));

            tx.send(State::Success {
                teams: vec![]
            }).unwrap();
        });
        return rx;
    }
}

pub fn build_by_team() -> ByTeam {
    let root_box = build_box();

    let loading_box = build_box();
    root_box.pack_start(&loading_box, false, false, 0);
    let loading = Loading::new(get_string(HomeLoading));
    loading_box.pack_start(&loading.widget, false, false, 0);

    let error_box = build_box();
    root_box.pack_start(&error_box, false, false, 0);

    let success_box = build_box();
    root_box.pack_start(&success_box, false, false, 0);

    return ByTeam {
        root: root_box,
        loading: loading_box,
        error: error_box,
        success: success_box,
        tag: String::from("StackClub"),
        title: get_string(StackClub),
    };
}

fn build_box() -> Box {
    return BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .valign(Align::Center)
        .spacing(0)
        .build();
}
