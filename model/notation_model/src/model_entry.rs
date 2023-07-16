use std::sync::{Arc, Weak};

use crate::prelude::{Tab, Track};
use notation_proto::prelude::{
    Duration, Entry, EntryPassMode, FrettedEntry4, FrettedEntry6, ProtoEntry, TrackKind, Units,
};

#[derive(Copy, Clone, Debug)]
pub struct ModelEntryProps {
    pub index: usize,
    pub tied_units: Units,
}

#[derive(Debug)]
pub struct ModelEntry {
    pub track: Weak<Track>,
    pub proto: Arc<ProtoEntry>,
    pub props: ModelEntryProps,
}
impl ModelEntry {
    pub fn new(
        track: Weak<Track>,
        proto: Arc<ProtoEntry>,
        index: usize,
        tied_units: Units,
    ) -> Self {
        let props = ModelEntryProps { index, tied_units };
        Self {
            track,
            proto,
            props,
        }
    }
}
impl Entry for ModelEntry {
    fn duration(&self) -> notation_proto::prelude::Duration {
        self.proto.duration()
    }
    fn prev_is_tie(&self) -> bool {
        self.prev().map(|x| x.proto.is_core_tie()).unwrap_or(false)
    }
    fn next_is_tie(&self) -> bool {
        self.next().map(|x| x.proto.is_core_tie()).unwrap_or(false)
    }
    fn tied_units(&self) -> Units {
        self.props.tied_units
    }
    fn pass_mode(&self) -> EntryPassMode {
        self.proto.pass_mode()
    }
}
impl ModelEntry {
    pub fn track(&self) -> Option<Arc<Track>> {
        self.track.upgrade().map(|x| x.clone())
    }
    pub fn tab(&self) -> Option<Arc<Tab>> {
        self.track().and_then(|x| x.tab())
    }
    pub fn as_fretted6(&self) -> Option<&FrettedEntry6> {
        self.proto.as_fretted6()
    }
    pub fn as_fretted4(&self) -> Option<&FrettedEntry4> {
        self.proto.as_fretted4()
    }
    pub fn prev(&self) -> Option<Arc<ModelEntry>> {
        if self.props.index == 0 {
            None
        } else if let Some(track) = self.track.upgrade() {
            track.entries.get(self.props.index - 1).map(|x| x.clone())
        } else {
            None
        }
    }
    pub fn next(&self) -> Option<Arc<ModelEntry>> {
        if let Some(track) = self.track.upgrade() {
            track.entries.get(self.props.index + 1).map(|x| x.clone())
        } else {
            None
        }
    }
    pub fn prev_as_mark(&self) -> Option<String> {
        if let Some(entry) = self.prev() {
            entry.proto.as_mark().map(|x| x.clone())
        } else {
            None
        }
    }
    pub fn get_tied_prev(&self) -> Option<Arc<ModelEntry>> {
        if self.props.index <= 1 {
            return None;
        }
        if let Some(track) = self.track.upgrade() {
            if let Some(prev) = track.entries.get(self.props.index - 1) {
                if prev.proto.is_core_tie() {
                    for i in self.props.index - 2..=0 {
                        let entry = track.entries.get(i).unwrap();
                        if entry.duration() != Duration::Zero {
                            return Some(entry.clone());
                        }
                    }
                }
            }
        }
        None
    }
    pub fn get_tied_next(&self) -> Option<Arc<ModelEntry>> {
        if let Some(track) = self.track.upgrade() {
            if let Some(next) = track.entries.get(self.props.index + 1) {
                if next.proto.is_core_tie() {
                    for i in self.props.index + 2..track.entries.len() {
                        let entry = track.entries.get(i).unwrap();
                        if entry.duration() != Duration::Zero {
                            return Some(entry.clone());
                        }
                    }
                }
            }
        }
        None
    }
    pub fn track_id(&self) -> String {
        if let Some(track) = self.track.upgrade() {
            track.id.clone()
        } else {
            "".to_owned()
        }
    }
    pub fn track_kind(&self) -> TrackKind {
        if let Some(track) = self.track.upgrade() {
            track.kind.clone()
        } else {
            TrackKind::Unsupported
        }
    }
    pub fn track_index(&self) -> Option<usize> {
        if let Some(track) = self.track.upgrade() {
            Some(track.props.index)
        } else {
            None
        }
    }
    pub fn get_track_entry<T, F: Fn(&ModelEntry) -> Option<T>>(&self, predicate: &F) -> Option<T> {
        if let Some(track) = self.track.upgrade() {
            track.get_entry(predicate)
        } else {
            None
        }
    }
}
