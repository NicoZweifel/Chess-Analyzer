use bevy::prelude::*;

use crate::picking::SelectIndicator;

#[derive(Bundle)]
struct SelectIndicatorBundle {
    indicator: SelectIndicator,
    sprite: SpriteBundle,
}

impl SelectIndicatorBundle {
    fn new(texture: Handle<Image>) -> Self {
        Self {
            indicator: SelectIndicator,
            sprite: SpriteBundle {
                texture,
                transform: Transform::from_xyz(0.0, 0.0, 0.3),
                ..default()
            },
        }
    }
}

pub trait SpawnSelectIndicator {
    fn spawn_select_indicator(&mut self, square: &Entity, asset_server: &Res<AssetServer>);
}

impl SpawnSelectIndicator for Commands<'_, '_> {
    fn spawn_select_indicator(&mut self, square: &Entity, asset_server: &Res<AssetServer>) {
        let texture: Handle<Image> = asset_server.load("Name=Off, Hint=On.png");

        let child = self.spawn(SelectIndicatorBundle::new(texture)).id();

        self.entity(*square).push_children(&[child]);
    }
}
