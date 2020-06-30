use app::add_menus;
use app::build_app;
use app::run_app;
use css::load_css;
use ui::home::show_home;

mod app;
mod css;
mod data;
mod ui;
mod widget;

fn main() {
    let app = build_app();
    load_css();
    add_menus(&app);
    show_home(&app);
    run_app(&app);
}
