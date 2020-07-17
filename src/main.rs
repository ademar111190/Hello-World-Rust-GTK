use std::sync::Arc;

use gio::prelude::*;

use app::add_menus;
use app::build_app;
use app::run_app;
use css::load_css;

use crate::ui::home::build_home;

mod app;
mod css;
mod data;
mod ui;
mod widget;

fn main() {
    gtk::init().expect("Failed to init GTK");
    let app = build_app();
    load_css();
    add_menus(&app);
    let home = Arc::new(build_home());
    let home_will_run = home.clone();
    let home_did_run = home.clone();
    app.connect_activate(move |app| home_will_run.will_run(&app));
    app.connect_activate(move |app| home_did_run.did_run());
    run_app(&app);
}
