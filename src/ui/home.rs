extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::{Align, Application, ApplicationWindow, HeaderBar, HeaderBarBuilder, Stack, StackBuilder,
          StackSwitcher, StackSwitcherBuilder, StackTransitionType};
use gtk::ApplicationWindowBuilder;
use gtk::prelude::*;
use gtk::WindowPosition::Center;

use crate::app::add_actions;
use crate::data::strings::get_string;
use crate::data::strings::StringId::HomeTitle;
use crate::ui::by_team::{build_by_team, ByTeam};
use crate::ui::by_year::{build_by_year, ByYear};

pub struct Home {
    pub by_team: ByTeam,
    pub by_year: ByYear,
}

impl Home {
    pub fn will_run(&self, app: &Application) {
        let stack = stack();
        stack.add_titled(&self.by_team.root, &self.by_team.tag, &self.by_team.title);
        stack.add_titled(&self.by_year.widget, &self.by_year.tag, &self.by_year.title);

        let window = home_window(&app);
        add_actions(&app, &window);
        window.set_titlebar(Some(&header_bar(&stack_switcher(&stack))));
        window.add(&stack);
        window.show_all();
    }

    pub fn did_run(&self) {
        println!("A");
        self.by_team.run().iter().for_each(|state| {
            println!("B {:?}", &state);
            self.by_team.render(&state);
        });
        println!("C");
    }
}

pub fn build_home() -> Home {
    return Home {
        by_team: build_by_team(),
        by_year: build_by_year(),
    };
}

fn header_bar(switcher: &StackSwitcher) -> HeaderBar {
    return HeaderBarBuilder::new()
        .show_close_button(true)
        .custom_title(switcher)
        .halign(Align::Fill)
        .build();
}

fn home_window(app: &Application) -> ApplicationWindow {
    return ApplicationWindowBuilder::new()
        .application(app)
        .decorated(true)
        .title(&get_string(HomeTitle))
        .default_width(600)
        .default_height(400)
        .window_position(Center)
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
