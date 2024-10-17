use std::fs;

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::HashMap;
use bevy_inspector_egui::egui::debug_text::print;
use bevy_mod_picking::prelude::*;
use shakmaty::{Chess, Position};

use crate::drag::DragEvent;
use crate::drag_end::DragEndEvent;
use crate::engine::EngineTasks;
use crate::select::SelectEvent;
use crate::utils::get_texture;
use crate::{Board, DropEvent, Engine, Game, History, Piece, Square};

pub(crate) fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    let white = materials.add(Color::WHITE);
    let black = materials.add(Color::BLACK);

    let start = Chess::default();
    let board = start.board().clone();
    let (by_role, by_color) = board.into_bitboards();
    let game = Game {
        board: Board { by_role, by_color },
        castling_rights: start.castles().castling_rights(),
        turn: start.turn(),
    };

    for (counter, square) in shakmaty::Square::ALL.iter().enumerate() {
        let mesh = Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0)));

        let j = (counter as f32 / 8.0).floor() as i32;
        let i = (counter % 8) as i32;

        commands.spawn((MaterialMesh2dBundle {
            mesh: mesh.clone(),
            // Change material according to position to get alternating pattern
            material: if (i + j + 1) % 2 == 0 {
                white.clone()
            } else {
                black.clone()
            },
            transform: Transform::from_xyz((i * 100 - 350) as f32, (j * 100 - 350) as f32, 0.1),
            ..Default::default()
        },));

        let texture = get_texture(start.board().piece_at(*square), &asset_server);

        commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: mesh.clone(),
                    material: materials.add(Color::hsla(0.0, 0.0, 0.0, 0.0)),
                    transform: Transform::from_xyz(
                        (i * 100 - 350) as f32,
                        (j * 100 - 350) as f32,
                        0.2,
                    ),
                    ..Default::default()
                },
                Square::new(*square),
                On::<Pointer<Drop>>::send_event::<DropEvent>(),
                On::<Pointer<Down>>::send_event::<SelectEvent>(),
                On::<Pointer<Drag>>::send_event::<DragEvent>(),
                On::<Pointer<DragEnd>>::send_event::<DragEndEvent>(),
            ))
            .with_children(|parent| {
                if let Some((texture, piece)) = texture {
                    parent.spawn((
                        Piece(piece),
                        SpriteBundle {
                            texture,
                            ..default()
                        },
                    ));
                }
            });
    }

    let entries = fs::read_dir("./engines").unwrap();

    for entry in entries.flatten() {
        if std::fs::File::open(entry.path()).is_ok() && !entry.path().ends_with(".gitignore") {
            commands.spawn(Engine(entry.path()));
        }
    }

    commands.insert_resource(EngineTasks(HashMap::new()));
    commands.spawn(History {
        entries: Vec::new(),
        current: 0,
    });
    commands.spawn(game);
}
