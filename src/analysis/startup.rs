use std::fs;

use bevy::{prelude::*, utils::HashMap};

use super::{Engine, EngineTasks};

pub(crate) fn startup(mut commands: Commands) {
    let entries = fs::read_dir("./engines").unwrap();

    for entry in entries.flatten() {
        if std::fs::File::open(entry.path()).is_ok() && !entry.path().ends_with(".gitignore") {
            commands.spawn(Engine(entry.path()));
        }
    }

    commands.insert_resource(EngineTasks(HashMap::new()));
}
