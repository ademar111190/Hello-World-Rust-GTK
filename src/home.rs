extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::{Application, ApplicationWindow, WindowPosition};
use gtk::prelude::*;

pub fn show_home(app: &Application) {
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Hello World Rust GTK");
        window.set_default_size(500, 800);
        window.set_position(WindowPosition::Center);
        window.show_all();
    });
}
