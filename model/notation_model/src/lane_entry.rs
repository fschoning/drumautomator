use std::fmt::Display;
use std::sync::{Arc, Weak};

use crate::prelude::{BarLane, BarLaneProps, ModelEntry, Tab, TabBar, TabBarProps, Track};
use notation_proto::prelude::{
    BarPosition, Duration, Entry, EntryPassMode, ProtoEntry, TrackKind, Units, Slice,
};

#[derive(Clone, Debug, Default)]
pub struct LaneEntryProps {
    pub slice: Slice,
    pub slice_index: usize,
    pub index: usize,
    pub in_bar_pos: Units,
    pub tied_units: Units,
    pub duration: Duration,
}

#[derive(Clone, Debug)]
pub struct LaneEntry {
    pub lane: Weak<BarLane>,
    pub model: Arc<ModelEntry>,
    pub props: LaneEntryProps,
}
impl Display for LaneEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<LaneEntry>({}: {})", self.props.index, self.model.proto)
    }
}
impl LaneEntry {
    pub fn new(
        lane: Weak<BarLane>,
        slice: Slice,
        slice_index: usize,
        index: usize,
        model: Arc<ModelEntry>,
        in_bar_pos: Units,
    ) -> Self {
        let props = LaneEntryProps {
            slice,
            slice_index,
            index,
            in_bar_pos,
            tied_units: model.tied_units(),
            duration: model.duration(),
        };
        Self { lane, model, props }
    }
}
impl Entry for LaneEntry {
    fn duration(&self) -> notation_proto::prelude::Duration {
        self.model.duration()
    }
    fn prev_is_tie(&self) -> bool {
        self.model.prev_is_tie()
    }
    fn next_is_tie(&self) -> bool {
        self.model.next_is_tie()
    }
    fn tied_units(&self) -> Units {
        self.model.tied_units()
    }
    fn pass_mode(&self) -> EntryPassMode {
        self.model.pass_mode()
    }
}
impl LaneEntry {
    pub fn in_bar_pos(&self) -> Units {
        self.props.in_bar_pos
    }
    pub fn bar_position(&self) -> BarPosition {
        BarPosition::new(
            self.bar_props().bar_units,
            self.bar_props().bar_ordinal,
            self.props.in_bar_pos,
        )
    }
}
impl LaneEntry {
    pub fn lane(&self) -> Option<Arc<BarLane>> {
        self.lane.upgrade().map(|x| x.clone())
    }
    pub fn track(&self) -> Option<Arc<Track>> {
        self.lane().map(|x| x.track.clone())
    }
    pub fn bar(&self) -> Option<Arc<TabBar>> {
        self.lane().and_then(|x| x.bar())
    }
    pub fn tab(&self) -> Option<Arc<Tab>> {
        self.bar().and_then(|x| x.tab())
    }
    pub fn lane_props(&self) -> BarLaneProps {
        self.lane().map(|x| x.props).unwrap_or_default()
    }
    pub fn bar_props(&self) -> TabBarProps {
        self.bar().map(|x| x.props).unwrap_or_default()
    }
}
impl LaneEntry {
    pub fn model(&self) -> &ModelEntry {
        self.model.as_ref()
    }
    pub fn proto(&self) -> &ProtoEntry {
        self.model.proto.as_ref()
    }
    pub fn prev(&self) -> Option<Arc<LaneEntry>> {
        if self.props.index == 0 {
            None
        } else if let Some(lane) = self.lane.upgrade() {
            lane.entries.get(self.props.index - 1).map(|x| x.clone())
        } else {
            None
        }
    }
    pub fn next(&self) -> Option<Arc<LaneEntry>> {
        if let Some(lane) = self.lane.upgrade() {
            lane.entries.get(self.props.index + 1).map(|x| x.clone())
        } else {
            None
        }
    }
    pub fn prev_as_mark(&self) -> Option<String> {
        if let Some(entry) = self.prev() {
            entry.model.proto.as_mark().map(|x| x.clone())
        } else {
            None
        }
    }
    pub fn track_id(&self) -> String {
        self.model.track_id()
    }
    pub fn track_kind(&self) -> TrackKind {
        self.model.track_kind()
    }
    pub fn track_index(&self) -> Option<usize> {
        self.model.track_index()
    }
    pub fn get_lane_entry<T, F: Fn(&LaneEntry) -> Option<T>>(&self, predicate: &F) -> Option<T> {
        if let Some(lane) = self.lane.upgrade() {
            lane.get_entry(predicate)
        } else {
            None
        }
    }
    pub fn get_track_entry<T, F: Fn(&ModelEntry) -> Option<T>>(&self, predicate: &F) -> Option<T> {
        if let Some(lane) = self.lane.upgrade() {
            lane.track.get_entry(predicate)
        } else {
            None
        }
    }
}
