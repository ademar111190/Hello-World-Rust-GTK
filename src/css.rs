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
}

pub fn apply_style<T: WidgetExt>(widget: &T, style: Style) {
    let class_name = match style {
        Style::LoadingLabel => "loadingLabel",
        Style::LoadingSpinner => "loadingSpinner",
    };
    widget.get_style_context().add_class(class_name);
}

const STYLE: &str = "
.loadingLabel {
    font-size: 22px;
}

.loadingSpinner {
    min-width: 32px;
    min-height: 32px;
    margin: 4px;
}

button {
    font-size: 14px;
}
";
