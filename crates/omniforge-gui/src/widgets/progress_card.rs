//! `Adw::Clamp` + `ProgressBar` pattern for long-running jobs.

use relm4::gtk;
use gtk::prelude::*;

/// Groups title, description, and a progress bar for multi-step workflows.
pub struct ProgressCard {
    root: gtk::Box,
    bar: gtk::ProgressBar,
}

impl ProgressCard {
    /// Builds a titled progress card.
    pub fn new(title: &str) -> Self {
        let root = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(8)
            .margin_top(8)
            .margin_bottom(8)
            .css_classes(["card"])
            .build();

        let label = gtk::Label::builder().label(title).xalign(0.0).build();
        let bar = gtk::ProgressBar::new();
        bar.set_show_text(true);

        root.append(&label);
        root.append(&bar);

        Self { root, bar }
    }

    /// Root container for layout managers.
    pub fn widget(&self) -> &gtk::Box {
        &self.root
    }

    /// Updates the fractional progress (`0.0` … `1.0`).
    pub fn set_fraction(&self, value: f64) {
        self.bar.set_fraction(value.clamp(0.0, 1.0));
    }
}
