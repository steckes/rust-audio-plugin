use filter::{Filter, FilterParams, FilterType};
use nih_plug::prelude::*;
use nih_plug_egui::EguiState;
use std::sync::Arc;

mod editor;
mod filter;

pub struct MyPlugin {
    params: Arc<PluginParams>,
    filter: Vec<Filter>, // Filter per Channel
}

#[derive(Params)]
struct PluginParams {
    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,

    #[id = "frequency"]
    pub frequency: FloatParam,

    #[id = "quality"]
    pub quality: FloatParam,

    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "type"]
    pub filter_type: EnumParam<FilterType>,
}

impl Default for MyPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(PluginParams::default()),
            filter: Vec::new(),
        }
    }
}

impl Default for PluginParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(400, 300),
            frequency: FloatParam::new(
                "Frequency",
                500.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20_000.0,
                    factor: 0.25,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" Hz"),
            quality: FloatParam::new(
                "Quality",
                0.71,
                FloatRange::Linear {
                    min: 0.1,
                    max: 12.0,
                },
            )
            .with_step_size(0.01)
            .with_smoother(SmoothingStyle::Linear(50.0)),
            gain: FloatParam::new(
                "Gain",
                0.0,
                FloatRange::Linear {
                    min: -30.0,
                    max: 30.0,
                },
            )
            .with_step_size(0.1)
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB"),
            filter_type: EnumParam::new("Filter Type", FilterType::Lowpass),
        }
    }
}

impl Plugin for MyPlugin {
    const NAME: &'static str = "My Plugin";
    const VENDOR: &'static str = "SteckTech";
    const URL: &'static str = "https://steck.tech";
    const EMAIL: &'static str = "info@steck.tech";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // determine number of input channels
        let num_channels = if let Some(ch) = audio_io_layout.main_input_channels {
            ch.get() as usize
        } else {
            0
        };

        // ensure there is one filter per channel
        self.filter
            .resize(num_channels, Filter::new(FilterType::Lowpass));

        // set the sample_rate for each filter
        for filter in self.filter.iter_mut() {
            if filter.set_sample_rate(buffer_config.sample_rate).is_err() {
                return false;
            }
        }
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let filter_type = self.params.filter_type.value();
        for filter in self.filter.iter_mut() {
            if filter.set_filter_type(filter_type).is_err() {
                return ProcessStatus::Error("Failed to set filter parameters");
            }
        }

        for mut frame in buffer.iter_samples() {
            let frequency = self.params.frequency.smoothed.next();
            let quality = self.params.quality.smoothed.next();
            let gain = self.params.gain.smoothed.next();

            for filter in self.filter.iter_mut() {
                if filter
                    .set_params(FilterParams {
                        frequency,
                        quality,
                        gain,
                    })
                    .is_err()
                {
                    return ProcessStatus::Error("Failed to set filter parameters");
                }
            }

            for (sample, filter) in frame.iter_mut().zip(self.filter.iter_mut()) {
                *sample = filter.tick(*sample);
            }
        }

        ProcessStatus::Normal
    }

    fn reset(&mut self) {
        for filter in self.filter.iter_mut() {
            // clear filter states
            filter.reset();
        }
    }

    // This can be used for cleaning up special resources like socket connections whenever the
    // plugin is deactivated. Most plugins won't need to do anything here.
    fn deactivate(&mut self) {}

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone())
    }
}

impl ClapPlugin for MyPlugin {
    const CLAP_ID: &'static str = "com.stecktech.myplug";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("An example plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for MyPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"MStecktechPlugin";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(MyPlugin);
nih_export_vst3!(MyPlugin);
