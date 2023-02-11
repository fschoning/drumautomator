use bevy::prelude::*;

use notation_bevy_utils::prelude::{DoLayoutEvent, ShapeOp};

use crate::prelude::{BarPlaying, NotationAssetsStates, NotationTheme};
use crate::prelude::NotationLayout;

use super::mini_bar::{MiniBar, MiniBarData};
use super::mini_map::MiniMap;

pub type MiniMapDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, MiniMap>;

pub struct MiniPlugin;

impl Plugin for MiniPlugin {
    fn build(&self, app: &mut App) {
        MiniMapDoLayoutEvent::setup(app);
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(on_bar_playing_changed)
                .with_system(MiniMap::do_layout)
                .with_system(MiniMap::update_debug_str)
                .with_system(MiniBar::on_layout_changed),
        );
    }
}

fn on_bar_playing_changed(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    mut query: Query<(Entity, &BarPlaying, &mut MiniBarData), Changed<BarPlaying>>,
) {
    for (entity, playing, mut data) in query.iter_mut() {
        //println!("{:?} -> {:?} -> {:?}", name, data, playing)
        data.value.playing_state = playing.value;
        data.update(&mut commands, &theme, entity);
    }
}
