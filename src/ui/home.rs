extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Box, BoxBuilder, Orientation, Stack, StackBuilder,
          StackSwitcher, StackSwitcherBuilder, StackTransitionType};
use gtk::ApplicationWindowBuilder;
use gtk::prelude::*;
use gtk::WindowPosition::Center;

use crate::app::add_actions;
use crate::css::{apply_style, Style};
use crate::data::strings::get_string;
use crate::data::strings::StringId::HomeTitle;
use crate::ui::by_team::build_by_team;
use crate::ui::by_year::build_by_year;

pub fn show_home(app: &Application) {
    app.connect_activate(|app| {
        let by_team = build_by_team();
        let by_year = build_by_year();

        let stack = stack();
        stack.add_titled(&by_team.widget, &by_team.tag, &by_team.title);
        stack.add_titled(&by_year.widget, &by_year.tag, &by_year.title);

        let switcher = stack_switcher(&stack);
        let switcher_box = switcher_box();
        switcher_box.pack_start(&switcher, false, false, 16);

        let home = home_box();
        home.pack_start(&switcher_box, false, false, 8);
        home.pack_start(&stack, true, true, 0);
        apply_style(&home, Style::Window);

        let window = home_window(&app);
        add_actions(&app, &window);
        window.add(&home);
        window.show_all();
    });
}

fn home_window(app: &Application) -> ApplicationWindow {
    return ApplicationWindowBuilder::new()
        .application(app)
        .decorated(true)
        .title(&get_string(HomeTitle))
        .default_width(400)
        .default_height(600)
        .window_position(Center)
        .build();
}

fn home_box() -> Box {
    return BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .spacing(0)
        .hexpand(true)
        .vexpand(true)
        .vexpand_set(true)
        .vexpand_set(true)
        .build();
}

fn switcher_box() -> Box {
    return BoxBuilder::new()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .valign(Align::Center)
        .spacing(0)
        .hexpand(true)
        .vexpand(false)
        .vexpand_set(true)
        .vexpand_set(true)
        .build();
}

fn stack_switcher(stack: &Stack) -> StackSwitcher {
    return StackSwitcherBuilder::new()
        .halign(Align::Center)
        .stack(stack)
        .build();
}

fn stack() -> Stack {
    return StackBuilder::new()
        .transition_duration(200)
        .transition_type(StackTransitionType::Crossfade)
        .build();
}
