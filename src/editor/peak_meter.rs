use nih_plug_egui::egui::{
    self, Align2, Color32, FontId, Painter, Rect, Response, Sense, Stroke, StrokeKind, Ui, Widget,
    style::WidgetVisuals,
};
use std::ops::RangeInclusive;

pub struct PeakMeter {
    level_range: RangeInclusive<f32>,
    level_db: f32,
    draw_label: bool,
}

impl PeakMeter {
    pub fn new(level_range: RangeInclusive<f32>, level_db: f32) -> Self {
        Self {
            level_range,
            level_db,
            draw_label: false,
        }
    }

    pub fn show_label(mut self, show: bool) -> Self {
        self.draw_label = show;
        self
    }
}

impl Widget for PeakMeter {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(4.0, 1.0);
        let (rect, response) =
            ui.allocate_exact_size(desired_size, Sense::focusable_noninteractive());

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let rect = rect.expand(visuals.expansion);
            self.paint_level_meter(ui.painter(), rect, visuals);
        }

        response
    }
}

impl PeakMeter {
    fn paint_level_meter(&self, painter: &Painter, rect: Rect, visuals: &WidgetVisuals) {
        // Background
        painter.rect(
            rect,
            visuals.corner_radius,
            visuals.bg_fill,
            visuals.fg_stroke,
            StrokeKind::Outside,
        );

        let mut level_rect = rect;
        *level_rect.right_mut() = egui::remap(
            self.level_db,
            self.level_range.clone(), // Clone here as remap takes it by value
            rect.left()..=rect.right(),
        )
        .clamp(rect.left(), rect.right());

        let color = if self.level_db < 0.0 {
            Color32::GREEN
        } else {
            Color32::RED
        };
        painter.rect(
            level_rect,
            visuals.corner_radius,
            color,
            Stroke::NONE,
            StrokeKind::Outside,
        );

        if self.draw_label {
            painter.text(
                rect.left_center(),
                Align2::LEFT_CENTER,
                format!("{:.1} dB", self.level_db),
                FontId::default(),
                visuals.text_color(),
            );
        }
    }
}
