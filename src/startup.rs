use bevy::prelude::*;

use crate::Game;

pub(crate) fn startup(mut commands: Commands) {
    commands.spawn(Game::default());
}
