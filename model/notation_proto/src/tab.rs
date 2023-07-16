use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::fmt::Display;

use crate::prelude::{Form, Section, Track};
use notation_core::prelude::{
    Key, Note, Pitch, Scale, Signature, Syllable, Tempo, Units, Octave,
};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct TabMeta {
    pub key: Key,
    pub scale: Scale,
    pub signature: Signature,
    pub tempo: Tempo,
}
impl TabMeta {
    pub fn bar_units(&self) -> Units {
        Units::from(self.signature)
    }
    pub fn calc_syllable(&self, pitch: &Pitch) -> Syllable {
        self.scale.calc_syllable(&self.key, pitch)
    }
    pub fn calc_note_from_pitch(&self, pitch: &Pitch, octave: &Octave) -> Note {
        self.scale.calc_note_from_pitch(&self.key, pitch, octave)
    }
    pub fn calc_note_from_syllable(&self, syllable: &Syllable, octave: &Octave) -> Note {
        self.scale.calc_note_from_syllable(&self.key, syllable, octave)
    }
}
impl Display for TabMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {:?}",
            self.key, self.scale, self.signature, self.tempo,
        )
    }
}
impl TabMeta {
    pub fn new(key: Key, scale: Scale, signature: Signature, tempo: Tempo) -> Self {
        Self {
            key,
            scale,
            signature,
            tempo,
        }
    }
}
impl Default for TabMeta {
    fn default() -> Self {
        Self {
            key: Key::C,
            scale: Scale::Major,
            signature: Signature::_4_4,
            tempo: Tempo::Moderato,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Tab {
    pub uuid: Uuid,
    pub meta: TabMeta,
    pub tracks: Vec<Track>,
    pub sections: Vec<Section>,
    pub form: Form,
}
impl Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Tab>({} T:{} S:{} F:{})",
            self.meta,
            self.tracks.len(),
            self.sections.len(),
            self.form.sections.len(),
        )
    }
}
impl Tab {
    pub fn new_uuid() -> String {
        Uuid::new_v4().to_string()
    }
    pub fn new(
        uuid: &str,
        meta: TabMeta,
        tracks: Vec<Track>,
        sections: Vec<Section>,
        form: Form,
    ) -> Self {
        let uuid = Uuid::parse_str(uuid).unwrap();
        Self {
            uuid,
            meta,
            tracks,
            sections,
            form,
        }
    }
    pub fn new_empty() -> Self {
        Self::new(
            Self::new_uuid().as_str(),
            TabMeta::default(),
            vec![],
            vec![],
            Form { sections: vec![] },
        )
    }
}
