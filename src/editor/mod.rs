use std::sync::Arc;

use filter_curve::filter_curve;
use nih_plug::editor::Editor;
use nih_plug_egui::{create_egui_editor, egui::Vec2, resizable_window::ResizableWindow};

use crate::{
    PluginParams,
    filter::{FilterParams, FilterType},
};

mod filter_curve;

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
                    filter_curve(
                        ui,
                        FilterType::Lowpass,
                        FilterParams {
                            frequency: params.frequency.value(),
                            quality: params.quality.value(),
                            gain: params.gain.value(),
                        },
                    );
                });
        },
    )
}
