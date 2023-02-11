use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_bevy_utils::prelude::{BevyUtil, OutlineRectangle, ShapeOp};
use notation_model::prelude::{Note, PlayingState, Syllable};

use crate::prelude::{EntryData, NotationTheme};
use notation_model::prelude::TabBar;
use crate::theme::theme_sizes::NotesSizes;

use super::tone_mode::ToneMode;

pub type ToneNoteData = EntryData<ToneNoteValue>;

#[derive(Clone, Debug)]
pub struct ToneNoteValue {
    pub note: Note,
    pub mode: ToneMode,
    pub playing_state: PlayingState,
    pub bar_size: f32,
}

impl ToneNoteValue {
    pub fn new(tab_bar: &TabBar, note: Note, mode: ToneMode) -> Self {
        Self {
            note,
            mode,
            playing_state: PlayingState::Idle,
            bar_size: 0.0,
        }
    }
    pub fn syllable(&self) -> Syllable {
        self.note.syllable
    }
}

impl ShapeOp<NotationTheme, OutlineRectangle> for ToneNoteData {
    fn get_shape(&self, theme: &NotationTheme) -> OutlineRectangle {
        let (width, height) = self.calc_width_height(theme);
        let color = theme.colors.of_syllable(self.value.syllable());
        let outline_color = theme
            .colors
            .syllables
            .outline
            .of_state(&self.value.playing_state);
        let outline_width = self.calc_outline(theme);
        let offset = if self.value.bar_size <= 0.0 {
            BevyUtil::offscreen_offset()
        } else {
            let x =
                self.value.bar_size / self.bar_props.bar_units.0 * self.entry_props.in_bar_pos.0;
            let mut y = self.get_sizes(theme).calc_note_y(self.value.note);
            if self.value.playing_state.is_current() {
                let outline = self.calc_outline(theme);
                y -= outline / 2.0;
            }

            let (_width, height) = self.calc_width_height(theme);
            let extra_z = if self.value.playing_state.is_current() {
                1.0
            } else {
                0.0
            };
            Vec3::new(x, y + height / 2.0, theme.z.tone + extra_z)
        };
        OutlineRectangle {
            width,
            height,
            origin: shapes::RectangleOrigin::TopLeft,
            color,
            outline_width,
            outline_color,
            offset,
        }
    }
}

impl ToneNoteData {
    fn get_sizes(&self, theme: &NotationTheme) -> NotesSizes {
        if self.value.mode.is_melody() {
            theme.sizes.melody
        } else {
            theme.sizes.harmony
        }
    }
    fn calc_outline(&self, theme: &NotationTheme) -> f32 {
        self.get_sizes(theme).note_outline
            .of_state(&self.value.playing_state)
    }
    fn calc_width_height(&self, theme: &NotationTheme) -> (f32, f32) {
        let outline = self.calc_outline(theme);
        let width =
            self.value.bar_size / self.bar_props.bar_units.0 * self.entry_props.tied_units.0;
        let mut height = self.get_sizes(theme).note_height;
        if self.value.playing_state.is_current() {
            height += outline;
        }
        (width - outline * 2.0, height)
    }
}
