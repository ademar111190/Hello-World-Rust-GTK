extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Orientation, StackTransitionType};
use gtk::prelude::*;
use gtk::WindowPosition::Center;

use crate::app::add_actions;
use crate::css::{apply_style, Style};
use crate::data::strings::get_string;
use crate::data::strings::StringId::{HomeLoading, HomeTitle, StackClub, StackYear};
use crate::widget::center_box::CenterBox;
use crate::widget::loading::Loading;

use self::gtk::{BoxBuilder, StackBuilder, StackSwitcherBuilder};

pub fn show_home(app: &Application) {
    app.connect_activate(|app| {
        let loading = Loading::new(get_string(HomeLoading));
        let root = CenterBox::new(loading.widget);

        let stack = StackBuilder::new().build();
        stack.add_titled(&root.widget, "StackClub", &get_string(StackClub));
        stack.add_titled(&Loading::new(String::from("XPTO")).widget, "StackYear", &get_string(StackYear));
        stack.set_transition_duration(200);
        stack.set_transition_type(StackTransitionType::Crossfade);

        let switcher = StackSwitcherBuilder::new()
            .halign(Align::Center)
            .build();
        switcher.set_stack(Some(&stack));

        let home = BoxBuilder::new()
            .orientation(Orientation::Vertical)
            .halign(Align::Fill)
            .valign(Align::Fill)
            .spacing(4)
            .hexpand(true)
            .vexpand(true)
            .vexpand_set(true)
            .vexpand_set(true)
            .build();

        home.pack_start(&switcher, false, false, 4);
        home.pack_start(&stack, true, true, 0);
        apply_style(&home, Style::Window);

        let window = ApplicationWindow::new(app);
        add_actions(&app, &window);
        window.set_decorated(true);
        window.set_title(&get_string(HomeTitle));
        window.set_default_size(400, 600);
        window.set_position(Center);
        window.add(&home);
        window.show_all();
    });
}
