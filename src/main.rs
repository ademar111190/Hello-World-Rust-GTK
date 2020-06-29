use app::build_app;
use app::run_app;
use ui::home::show_home;

mod app;
mod data;
mod ui;

fn main() {
    let app = build_app();
    show_home(&app);
    run_app(&app);
}
