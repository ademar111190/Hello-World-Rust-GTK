use gtk::{Label, LabelBuilder};

pub fn build_label(text: String) -> Label {
    return LabelBuilder::new()
        .label(&text)
        .build();
}
