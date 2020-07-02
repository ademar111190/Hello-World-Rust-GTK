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
.loadingLabel {
    font-size: 22px;
    color: #90A4AE;
}

.loadingSpinner {
    min-width: 32px;
    min-height: 32px;
    margin: 4px;
    color: #90A4AE;
}

button {
    color: #78909C;
    background: #263238;
    border: 1px solid #424242;
    box-shadow: none;
    font-size: 14px;
    text-shadow: 0 1px rgba(97, 97, 97, 0.9);
}
button:active {
    color: #B0BEC5;
    background: #37474F;
}
button:checked {
    box-shadow: inset 0 1px rgba(97, 97, 97, 0.9);
    text-shadow: none;
}

.window {
    background-color: #212121;
}
";
