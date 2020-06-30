extern crate gdk;
extern crate gio;
extern crate gtk;

use gdk::Screen;
use gtk::prelude::*;
use gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;
use gtk::StyleContext;

pub fn load_css() {
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(STYLE.as_bytes())
        .expect("Failed to load CSS");
    let screen = Screen::get_default()
        .expect("Error initializing gtk css provider.");
    let priority = STYLE_PROVIDER_PRIORITY_APPLICATION;
    StyleContext::add_provider_for_screen(&screen, &provider, priority);
}

pub enum Style {
    LoadingLabel,
    LoadingSpinner,
    Window,
}

pub fn apply_style<T: WidgetExt>(widget: &T, style: Style) {
    let class_name = match style {
        Style::LoadingLabel => "loadingLabel",
        Style::LoadingSpinner => "loadingSpinner",
        Style::Window => "window",
    };
    widget.get_style_context().add_class(class_name);
}

const STYLE: &str = "
.window {
    background-color: #212121;
}

.loadingLabel {
    font-size: 22px;
    color: #C5E1A5;
}

.loadingSpinner {
    min-width: 32px;
    min-height: 32px;
    margin: 4px;
    color: #C5E1A5;
}
";
