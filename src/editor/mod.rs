use std::sync::Arc;

use filter_curve::{FilterCurvePlot, PlotType};
use nih_plug::editor::Editor;
use nih_plug_egui::{create_egui_editor, egui::Vec2, resizable_window::ResizableWindow};

use crate::{PluginParams, filter::Lowpass};

pub mod filter_curve;
mod peak_meter;
mod toggle;

pub(crate) fn create(params: Arc<PluginParams>) -> Option<Box<dyn Editor>> {
    let egui_state = params.editor_state.clone();
    create_egui_editor(
        egui_state.clone(),
        (),
        |_, _| {},
        move |ctx, _setter, _state| {
            ResizableWindow::new("res-wind")
                .min_size(Vec2::new(128.0, 128.0))
                .show(ctx, egui_state.as_ref(), |ui| {
                    FilterCurvePlot::new(
                        PlotType::Magnitude,
                        Lowpass::new(48000.0, params.freq.value(), params.quality.value()).unwrap(),
                    )
                    .show(ui);
                });
        },
    )
}
