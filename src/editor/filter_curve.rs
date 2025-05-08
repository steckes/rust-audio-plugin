use egui_plot::{
    AxisHints, CoordinatesFormatter, Corner, GridInput, GridMark, Legend, Line, Plot, PlotBounds,
    PlotPoint,
};
use nih_plug_egui::egui::{Ui, Vec2b};
use num::{Float, complex::ComplexFloat};
use std::ops::RangeInclusive;

use crate::filter::{FilterParams, FilterType, filter_response};

const X_BOUNDS: (f64, f64) = (20.0, 20_0000.0);

pub fn filter_curve(ui: &mut Ui, filter_type: FilterType, params: FilterParams) {
    let freq_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| {
        const FREQ_LABELS: [u32; 5] = [50, 100, 500, 2000, 10_000];
        let freq = scale_to_log10(mark.value, X_BOUNDS.0, X_BOUNDS.1);
        if FREQ_LABELS.contains(&(freq.round() as u32)) {
            format!("{:.0} Hz", freq)
        } else {
            String::new()
        }
    };

    let spl_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| format!("{} dB", mark.value);

    let label_fmt = |_s: &str, val: &PlotPoint| {
        format!(
            "{x:.0} Hz\n{y:.2} dB",
            x = scale_to_log10(val.x, X_BOUNDS.0, X_BOUNDS.1),
            y = val.y,
        )
    };
    let formatter = CoordinatesFormatter::new(|val, _bounds| {
        format!(
            "{x:.0} Hz\n{y:.2} dB",
            x = scale_to_log10(val.x, X_BOUNDS.0, X_BOUNDS.1),
            y = val.y,
        )
    });

    let x_axes = vec![
        AxisHints::new_x()
            .label("Frequency")
            .formatter(freq_formatter),
    ];

    let y_axes = vec![AxisHints::new_y().label("SPL").formatter(spl_formatter)];

    const NUM_PLOT_POINTS: usize = 100;
    let response = (0..NUM_PLOT_POINTS)
        .map(|i| i as f64 / (NUM_PLOT_POINTS as f64 + 30.0))
        .map(|x_pos| {
            (
                x_pos,
                filter_response(
                    scale_to_log10(x_pos, X_BOUNDS.0, X_BOUNDS.1),
                    filter_type,
                    48000.0,
                    params,
                ),
            )
        })
        .collect::<Vec<_>>();

    Plot::new("Filter Response")
        .legend(Legend::default())
        .x_grid_spacer(x_grid)
        .custom_x_axes(x_axes)
        .custom_y_axes(y_axes)
        .label_formatter(label_fmt)
        .coordinates_formatter(Corner::LeftBottom, formatter)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false)
        .auto_bounds(Vec2b::FALSE)
        .show(ui, |plot_ui| {
            plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0, -30.0], [0.8, 30.0]));
            plot_ui.line(Line::new(
                response
                    .iter()
                    .map(|(x_pos, response)| [*x_pos, response.norm().log10() as f64 * 20.0])
                    .collect::<Vec<_>>(),
            ));
            plot_ui.line(Line::new(
                response
                    .iter()
                    .map(|(x_pos, response)| [*x_pos, response.norm().arg() as f64])
                    .collect::<Vec<_>>(),
            ));
        });
}

fn x_grid(input: GridInput) -> Vec<GridMark> {
    const FREQ_LINES: [u32; 29] = [
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000,
        2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000, 20000,
    ];
    let (min, max) = input.bounds;
    let mut marks = Vec::new();
    let last_value = 0.0;
    for f in FREQ_LINES {
        let value = scale_from_log10(f as f64, X_BOUNDS.0, X_BOUNDS.1);
        if (min..max).contains(&value) {
            marks.push(GridMark {
                value,
                step_size: value - last_value,
            });
        }
    }

    marks
}

/// Scales a normalized value (between 0 and 1) to a logarithmic target range.
/// The entire target range must be greater than zero.
#[inline(always)]
pub fn scale_to_log10<F: Float>(source: F, log_min: F, log_max: F) -> F {
    if log_min <= F::zero() || log_max <= F::zero() {
        return F::zero();
    }

    let log_min = log_min.log10();
    let log_max = log_max.log10();

    let target = F::powf(
        F::from(10.0).unwrap(),
        log_min + source * (log_max - log_min),
    );
    debug_assert!(!target.is_nan() && target.is_finite());
    target
}

/// Remaps a logarithmic value in a target range to a normalized value (between 0 and 1).
/// The entire target range must be greater than zero.
#[inline(always)]
pub fn scale_from_log10<F: Float>(source_log: F, log_min: F, log_max: F) -> F {
    if log_min <= F::zero() || log_max <= F::zero() {
        return F::zero();
    }

    let log_min = log_min.log10();
    let log_max = log_max.log10();

    let target = (F::log10(source_log) - log_min) / (log_max - log_min);
    target
}
