use nih_plug_egui::egui;

pub fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    // Set the toggle switch size to be twice as wide as it is tall
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    // Allocate space for the toggle and get the rectangle and response
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    // Toggle the state when clicked
    if response.clicked() {
        *on = !*on;
        response.mark_changed(); // Notify the UI that a value has changed
    }

    // Only draw the toggle if it's visible in the current view
    if ui.is_rect_visible(rect) {
        // Get animation progress for smooth transition (0.0 to 1.0)
        let how_on = ui.ctx().animate_bool_responsive(response.id, *on);
        // Get the visual style based on interaction state and toggle value
        let visuals = ui.style().interact_selectable(&response, *on);
        // Expand the rectangle according to the visual style
        let rect = rect.expand(visuals.expansion);
        // Calculate the radius for rounded corners (half the height)
        let radius = 0.5 * rect.height();

        // Draw the background rounded rectangle
        ui.painter().rect(
            rect,
            radius,
            visuals.bg_fill,
            visuals.bg_stroke,
            egui::StrokeKind::Inside, // Stroke is drawn inside the rectangle
        );

        // Calculate the x-position for the circle, interpolated based on toggle state
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        // Create a position vector for the circle's center
        let center = egui::pos2(circle_x, rect.center().y);
        // Draw the toggle circle
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    // Return the response for this widget
    response
}
