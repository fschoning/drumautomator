pub mod midi_hub;
pub mod midi_message;
pub mod midi_plugin;
pub mod midi_settings;
pub mod midi_state;
pub mod midi_util;

pub mod play;

pub use notation_audio;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub mod prelude {
    #[doc(hidden)]
    pub use notation_audio::prelude::*;
    #[doc(hidden)]
    pub use crate::midi_hub::MidiHub;
    #[doc(hidden)]
    pub use crate::midi_message::MidiMessage;
    #[doc(hidden)]
    pub use crate::midi_plugin::MidiPlugin;
    #[doc(hidden)]
    pub use crate::midi_settings::MidiSettings;
    #[doc(hidden)]
    pub use crate::midi_state::{MidiChannel, MidiState};
    #[doc(hidden)]
    pub use crate::midi_util::MidiUtil;

    #[doc(hidden)]
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::native::midi_synth::MidiSynth;
    #[cfg(target_arch = "wasm32")]
    pub use crate::wasm::midi_synth::MidiSynth;

    #[doc(hidden)]
    pub use crate::play::play_clock::PlayClock;
    #[doc(hidden)]
    pub use crate::play::play_control::{PlayControl, PlaySpeed, TickResult};
    #[doc(hidden)]
    pub use crate::play::play_state::{PlayState, PlayingState};
    #[doc(hidden)]
    pub use crate::play::play_events::*;

}
