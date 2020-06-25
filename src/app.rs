extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::Application;

pub fn build_app() -> Application {
    return Application::new(
        Some("ademar.hello.world.rust.gtk"),
        Default::default(),
    ).expect("failed to initialize GTK application");
}

pub fn run_app(app: &Application) {
    app.run(&[]);
}
