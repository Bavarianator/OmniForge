//! GTK4 `DropTarget` wrapper for file:// and plain path payloads.

use gtk::prelude::*;
use relm4::gtk;

/// Visual drop region; connect `DropTarget` signals in the Relm4 controller.
pub struct FileDropZone {
    frame: gtk::Frame,
}

impl FileDropZone {
    /// Creates a labeled frame suitable for CSS styling via `omniforge-drop-zone`.
    pub fn new() -> Self {
        let frame = gtk::Frame::new(Some("Dateien hier ablegen"));
        frame.set_label(Some("Dateien hier ablegen"));
        frame.add_css_class("card");
        frame.add_css_class("omniforge-drop-zone");
        Self { frame }
    }

    /// Underlying widget for packing into parent containers.
    pub fn widget(&self) -> &gtk::Frame {
        &self.frame
    }
}
