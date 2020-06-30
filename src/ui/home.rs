extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::prelude::*;
use gtk::WindowPosition::Center;

use crate::app::add_actions;
use crate::css::{apply_style, Style};
use crate::data::strings::get_string;
use crate::data::strings::StringId::HomeLoading;
use crate::data::strings::StringId::HomeTitle;
use crate::widget::center_box::CenterBox;
use crate::widget::loading::Loading;

pub fn show_home(app: &Application) {
    app.connect_activate(|app| {
        let loading = Loading::new(get_string(HomeLoading));
        let root = CenterBox::new(loading.widget);
        apply_style(&root.widget, Style::Window);

        let window = ApplicationWindow::new(app);
        add_actions(&app, &window);
        window.set_decorated(true);
        window.set_title(&get_string(HomeTitle));
        window.set_default_size(400, 600);
        window.set_position(Center);
        window.add(&root.widget);
        window.show_all();
    });
}
