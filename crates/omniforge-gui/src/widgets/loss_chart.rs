//! Loss curve rendering (Plotters + Cairo hooks behind the `charts` feature).

use gtk::prelude::*;
use relm4::gtk;

/// Drawing area placeholder; enable `charts` + integrate `plotters-cairo` for live metrics.
pub struct LossChart {
    area: gtk::DrawingArea,
}

impl LossChart {
    /// Allocates a chart surface with an explicit minimum height for HIG-compliant spacing.
    pub fn new() -> Self {
        let area = gtk::DrawingArea::builder()
            .height_request(220)
            .css_classes(["card"])
            .build();
        Self { area }
    }

    /// Access the underlying `DrawingArea` for signal wiring.
    pub fn widget(&self) -> &gtk::DrawingArea {
        &self.area
    }
}
