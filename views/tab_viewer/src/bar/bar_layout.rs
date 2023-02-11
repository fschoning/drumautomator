use std::fmt::Display;
use std::sync::Arc;
use bevy::prelude::*;

use notation_model::prelude::TabBarProps;

use crate::prelude::LaneLayoutData;

#[derive(Clone, Debug, Component)]
pub struct BarLayoutData {
    pub min_height: f32,
    pub bar_props: TabBarProps,
    pub lane_layouts: Vec<Arc<LaneLayoutData>>,
}
impl Display for BarLayoutData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BarLayoutData>([{}])", self.lane_layouts.len())
    }
}
impl BarLayoutData {
    pub fn new(
        min_height: f32,
        bar_props: TabBarProps,
        lane_layouts: Vec<Arc<LaneLayoutData>>,
    ) -> Self {
        Self {
            min_height,
            bar_props,
            lane_layouts: lane_layouts,
        }
    }
    pub fn height(&self) -> f32 {
        Self::calc_height(self.min_height, &self.lane_layouts)
    }
    pub fn calc_height(min_height: f32, lane_layouts: &Vec<Arc<LaneLayoutData>>) -> f32 {
        let mut height = 0.0;
        let len = lane_layouts.len();
        for (index, lane_layout) in lane_layouts.iter().enumerate() {
            if lane_layout.visible() {
                height += lane_layout.height;
                if index < len - 1 {
                    height += lane_layout.margin;
                }
            }
        }
        if height > min_height {
            height
        } else {
            min_height
        }
    }
}
