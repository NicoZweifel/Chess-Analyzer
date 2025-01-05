use bevy::prelude::*;

use crate::analysis::EngineIndicator;

#[derive(Bundle)]
struct EngineIndicatorBundle {
    indicator: EngineIndicator,
    sprite: SpriteBundle,
}

impl Default for EngineIndicatorBundle {
    fn default() -> Self {
        Self {
            indicator: EngineIndicator,
            sprite: SpriteBundle { ..default() },
        }
    }
}

pub trait SpawnEngineIndicator {
    fn spawn_engine_indicator(&mut self, square: &Entity, asset_server: &Res<AssetServer>);
}

impl SpawnEngineIndicator for Commands<'_, '_> {
    fn spawn_engine_indicator(&mut self, square: &Entity, asset_server: &Res<AssetServer>) {
        let texture: Handle<Image> = asset_server.load("Engine_Move.png");

        let child_to = self
            .spawn(EngineIndicatorBundle {
                sprite: SpriteBundle {
                    texture,
                    transform: Transform::from_xyz(0.0, 0.0, 0.3),
                    ..default()
                },
                ..default()
            })
            .id();

        self.entity(*square).push_children(&[child_to]);
    }
}
