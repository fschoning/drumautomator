use bevy::prelude::*;
use bevy_easings::{Ease, EaseFunction, EasingComponent, EasingType};
use float_eq::float_ne;
use notation_bevy_utils::prelude::{GridData, LayoutData};

use notation_model::lane_kind::LaneKind;
use notation_model::prelude::Position;
use serde::{Deserialize, Serialize};

use crate::bar::bar_layout::BarLayoutData;
use crate::lane::lane_layout::LaneLayoutData;
use crate::play::pos_indicator::PosIndicatorData;
use crate::prelude::{NotationTheme, TabBars};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum LayoutMode {
    Grid,
    Line,
}
impl Default for LayoutMode {
    fn default() -> Self {
        Self::Grid
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum GridAlignMode {
    Center,
    ForceCenter,
    Top,
    ForceTop,
}
impl Default for GridAlignMode {
    fn default() -> Self {
        Self::Center
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LayoutSettings {
    pub mode: LayoutMode,
    pub grid_align_mode: GridAlignMode,
    pub focus_bar_ease_ms: u64,
    pub focusing_bar_ordinal: usize,
    pub video_recording_mode: bool,
    pub override_tab_width: Option<f32>,
    pub override_focus_offset_y: Option<f32>,
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            mode: LayoutMode::default(),
            grid_align_mode: GridAlignMode::default(),
            focus_bar_ease_ms: 250,
            focusing_bar_ordinal: usize::MAX,
            video_recording_mode: false,
            override_tab_width: None,
            override_focus_offset_y: None,
        }
    }
}

impl LayoutSettings {
    pub fn sort_lane_layouts(&self, lanes: &Vec<(LaneKind, LaneLayoutData)>) -> Vec<(LaneKind, LaneLayoutData)> {
        let mut sorted: Vec<(LaneKind, LaneLayoutData)> = lanes.clone();
        sorted.sort_by(|a, b| a.1.order().cmp(&b.1.order()));
        sorted
    }
    pub fn bar_layout_of_pos(
        &self,
        _pos: Position,
    ) -> Option<BarLayoutData> {
        //bar_layouts.get(pos.bar.bar_ordinal).map(|x| x.clone())
        None
    }
    pub fn pan_tab_bars(
        &self,
        theme: &NotationTheme,
        tab_bars_query: &mut Query<(
            Entity,
            &mut Transform,
            &TabBars,
            &LayoutData,
            &GridData,
        )>,
        delta_x: f32,
        delta_y: f32,
    ) {
        if let Ok((_, mut camera_transform, _bars, layout, grid_data)) = tab_bars_query.get_single_mut()
        {
            let trans = camera_transform.translation;
            let (x, y) = match self.mode {
                LayoutMode::Grid => {
                    let mut y = trans.y + delta_y;
                    let min_y =
                        layout.offset.y + grid_data.offset.y - theme.sizes.layout.page_margin;
                    if y < min_y {
                        y = min_y;
                    } else {
                        let max_y = layout.offset.y
                            + theme.sizes.layout.page_margin
                            + grid_data.content_size.height
                            - grid_data.grid_size.height;
                        if y > max_y {
                            y = max_y;
                        }
                    }
                    (trans.x, y)
                }
                LayoutMode::Line => {
                    let mut x = trans.x - delta_x;
                    let max_x = 0.0;
                    if x > max_x {
                        x = max_x
                    } else {
                        let min_x = grid_data
                            .calc_cell_size(grid_data.rows, grid_data.cols)
                            .width
                            - grid_data.content_size.width;
                        if x < min_x {
                            x = min_x
                        }
                    }
                    (x, trans.y)
                }
            };
            *camera_transform = Transform::from_xyz(x, y, trans.z);
        }
    }
    pub fn set_transform_xy(&self, transform: &mut Transform, x: Option<f32>, y: Option<f32>) {
        let trans = transform.translation;
        *transform = Transform::from_xyz(x.unwrap_or(trans.x), y.unwrap_or(trans.y), trans.z);
    }
    pub fn ease_transform_xy(
        &self,
        commands: &mut Commands,
        entity: Entity,
        transform: &mut Transform,
        x: Option<f32>,
        y: Option<f32>,
    ) {
        let mut entity_commands = commands.entity(entity);
        entity_commands.remove::<EasingComponent<Transform>>();
        let from = transform.translation;
        let to = Vec3::new(x.unwrap_or(from.x), y.unwrap_or(from.y), from.z);
        if float_ne!(from.x, to.x, abs <= 0.01) || float_ne!(from.y, to.y, abs <= 0.01) {
            println!(
                "layout_settings.ease_transform_xy(): {}, {} -> {}, {}",
                from.x, from.y, to.x, to.y
            );
            if self.focus_bar_ease_ms > 0 {
                let ease_function = EaseFunction::CubicOut;
                entity_commands.insert(transform.ease_to(
                    Transform::from_translation(to),
                    ease_function,
                    EasingType::Once {
                        duration: std::time::Duration::from_millis(self.focus_bar_ease_ms),
                    },
                ));
            } else {
                transform.translation = to;
            }
        }
    }
    fn calc_grid_focus_y(
        &self,
        theme: &NotationTheme,
        _bars: &TabBars,
        layout: &LayoutData,
        grid_data: &GridData,
        pos_data: &PosIndicatorData,
    ) -> f32 {
        let (_row, col) = grid_data.calc_row_col(pos_data.bar_position.bar_ordinal);
        let mut y = pos_data.bar_layout.offset.y;
        let grid_size = layout.size;
        let content_size = grid_data.content_size;
        if self.video_recording_mode || self.grid_align_mode == GridAlignMode::ForceTop {
            match self.override_focus_offset_y {
                Some(offset_y) => {
                    y += offset_y;
                },
                None => {
                    if grid_size.height > content_size.height {
                        y += (content_size.height - grid_size.height) / 2.0;
                    }
                }
            }
        } else {
            if grid_size.height > content_size.height {
                if self.grid_align_mode != GridAlignMode::ForceCenter {
                    y = -(grid_size.height - content_size.height);
                }
            } else {
                /* try to show as 2nd row
                let last_row_height = grid_data.calc_cell_size(row - 1, col).height;
                if last_row_height + pos_data.bar_layout.size.height <= grid_size.height / 2.0 {
                    y = grid_data.calc_cell_offset(row - 1, col).y;
                }
                 */
                if self.grid_align_mode == GridAlignMode::Center
                    || self.grid_align_mode == GridAlignMode::ForceCenter
                {
                    y += (grid_size.height
                        - grid_data.margin.height * 2.0
                        - pos_data.bar_layout.size.height)
                        / 2.0
                        + self.override_focus_offset_y.unwrap_or(0.0);
                }
                if self.grid_align_mode != GridAlignMode::ForceCenter {
                    let min_y = grid_size.height
                        - content_size.height
                        - theme.sizes.layout.page_margin * 2.0;
                    if y < min_y {
                        y = min_y;
                    } else {
                        let max_y = grid_data.calc_cell_offset(0, col).y;
                        if y > max_y {
                            y = max_y;
                        }
                    }
                }
            }
        }
        y - layout.offset.y - grid_data.offset.y + theme.sizes.layout.page_margin
    }
    fn calc_line_focus_xy(
        &self,
        theme: &NotationTheme,
        _bars: &TabBars,
        layout: &LayoutData,
        grid_data: &GridData,
        pos_data: &PosIndicatorData,
    ) -> (f32, f32) {
        let grid_size = layout.size;
        let bar_ordinal = pos_data.bar_position.bar_ordinal;
        let mut x = layout.offset.x + grid_data.offset.x;
        if bar_ordinal == 0 {
            if pos_data.bar_layout.size.width > grid_size.width / 3.0 {
                if pos_data.offset_x() > pos_data.bar_layout.size.width / 2.0 {
                    x = pos_data.offset_x() - pos_data.bar_layout.size.width / 2.0;
                }
            }
        } else {
            let last_cell_width = grid_data.calc_cell_size(0, bar_ordinal - 1).width;
            if last_cell_width + pos_data.bar_layout.size.width <= grid_size.width * 2.0 / 3.0 {
                x = pos_data.offset_x() - last_cell_width;
            } else {
                x = pos_data.offset_x() - last_cell_width / 2.0;
            }
        }
        let grid_size = layout.size;
        let content_size = grid_data.content_size;
        let y = pos_data.bar_layout.offset.y + grid_size.height
            - content_size.height
            - theme.sizes.layout.page_margin;
        (
            x - layout.offset.x - grid_data.offset.x,
            y - layout.offset.y,
        )
    }
    pub fn focus_bar(
        &mut self,
        commands: &mut Commands,
        theme: &NotationTheme,
        tab_bars_query: &mut Query<(
            Entity,
            &mut Transform,
            &TabBars,
            &LayoutData,
            &GridData,
        )>,
        pos_data: &PosIndicatorData,
    ) {
        if self.mode == LayoutMode::Grid
            && self.focusing_bar_ordinal == pos_data.bar_props.bar_ordinal
        {
            return;
        }
        if let Ok((bars_entity, mut bars_transform, bars, layout, grid_data)) =
            tab_bars_query.get_single_mut()
        {
            self.focusing_bar_ordinal = pos_data.bar_props.bar_ordinal;
            match self.mode {
                LayoutMode::Grid => {
                    let y = self.calc_grid_focus_y(theme, bars, layout, grid_data, pos_data);
                    self.ease_transform_xy(
                        commands,
                        bars_entity,
                        &mut bars_transform,
                        None,
                        Some(-y),
                    );
                }
                LayoutMode::Line => {
                    let (x, y) = self.calc_line_focus_xy(theme, bars, layout, grid_data, pos_data);
                    self.set_transform_xy(&mut bars_transform, Some(-x), Some(-y));
                }
            }
        } else {
            println!("layout_settings.focus_bar() Query Failed");
        }
    }
}
