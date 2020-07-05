extern crate gio;
extern crate gtk;

use std::fs::File;
use std::io::{ErrorKind, Read};
use std::thread;

use gtk::{Box, Orientation};

use crate::data::strings::get_string;
use crate::data::strings::StringId::{HomeLoading, StackYear};
use crate::data::team::{build_teams_payload, TeamsPayload};
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
        match read_json() {
            Ok(teams_payload) => show_teams_payload(teams_payload),
            Err(error) => show_error(error),
        }
    });

    return ByYear {
        widget: root.widget,
        tag: String::from("StackYear"),
        title: get_string(StackYear),
    };
}

fn show_error(error: String) {
    println!("Error > {}", error);
}

fn show_teams_payload(teams_payload: TeamsPayload) {
    println!("Success > {:?}", teams_payload);
}

fn read_json() -> Result<TeamsPayload, String> {
    return File::open("assets/byTeam.json")
        .map_err(|error| {
            match error.kind() {
                ErrorKind::NotFound => String::from("Aquivo de dados não encontrado"),
                _ => String::from("Falha desconhecida ao acessar aquivo de dados"),
            }
        })
        .and_then(|mut file: File| -> Result<String, String> {
            let mut json = String::new();
            return match file.read_to_string(&mut json) {
                Ok(_) => Ok(json),
                Err(error) => Err(format!("{:?}", error)),
            };
        })
        .and_then(|json: String| -> Result<TeamsPayload, String> {
            build_teams_payload(&json)
        });
}
