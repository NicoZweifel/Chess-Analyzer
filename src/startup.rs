use bevy::prelude::*;

use crate::Game;

pub(crate) fn startup(mut commands: Commands) {
    let game = Game::default();
    commands.spawn(game);
}
