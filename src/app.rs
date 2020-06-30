extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use glib::clone;
use gtk::Application;
use gtk::prelude::*;

use crate::data::strings::get_string;
use crate::data::strings::StringId::Quit;

use self::gio::SimpleAction;
use self::gtk::ApplicationWindow;

pub fn build_app() -> Application {
    return Application::new(
        Some("ademar.hello.world.rust.gtk"),
        Default::default(),
    ).expect("failed to initialize GTK application");
}

pub fn add_menus(application: &Application) {
    application.connect_activate(|app| {
        let menu = gio::Menu::new();
        menu.append(Some(&get_string(Quit)), Some("app.quit"));
        app.set_app_menu(Some(&menu));
        app.set_accels_for_action("app.quit", &["<Primary>Q"]);
    });
}

pub fn add_actions(application: &Application, window: &ApplicationWindow) {
    let quit = SimpleAction::new("quit", None);
    quit.connect_activate(clone!(@weak window => move |_, _| {
        window.destroy();
    }));
    application.add_action(&quit);
}

pub fn run_app(app: &Application) {
    app.run(&[]);
}
