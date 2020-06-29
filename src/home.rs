extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::{Application, ApplicationWindow, WindowPosition};
use gtk::prelude::*;

use crate::data::strings::get_string;
use crate::data::strings::StringId::HomeTitle;

use self::gtk::Label;

pub fn show_home(app: &Application) {
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(&get_string(HomeTitle));
        window.set_default_size(500, 800);
        window.set_position(WindowPosition::Center);
        window.add(&Label::new(Some("Hello World!")));
        window.show_all();
    });
}
