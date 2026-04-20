//! Landing dashboard cards (recent projects, hardware summary placeholders).

use gtk::prelude::*;
use relm4::gtk;

/// Static dashboard layout until navigation model lands.
pub struct DashboardWidgets {
    root: gtk::Box,
}

impl DashboardWidgets {
    /// Builds the default dashboard with placeholder copy aligned to the OmniForge vision.
    pub fn new() -> Self {
        let root = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(8)
            .build();

        let title = gtk::Label::builder()
            .label("Willkommen bei OmniForge")
            .xalign(0.0)
            .build();
        title.add_css_class("title-1");

        let subtitle = gtk::Label::builder()
            .label(
                "Lokales Fine-Tuning, RAG und Export — ohne Cloud. \
                 Die Modul-Ansichten (Data Forge, Training, Packager) werden hier als Stack eingehängt.",
            )
            .wrap(true)
            .xalign(0.0)
            .build();

        root.append(&title);
        root.append(&subtitle);

        Self { root }
    }

    /// Returns the vertical box used as the dashboard page root.
    pub fn root_widget(&self) -> &gtk::Box {
        &self.root
    }
}
