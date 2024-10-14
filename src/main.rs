use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::{prelude::*, sprite::Anchor};
use bevy_mod_picking::prelude::*;
use shakmaty::{Chess, Position};

#[derive(Component, Clone, Copy, Debug)]
struct Piece(shakmaty::Piece);

#[derive(Component, Clone, Copy, Debug)]
struct Square(shakmaty::Square);

#[derive(Component, Clone, Copy, Debug)]
struct DragAndDropArea;

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    let white = materials.add(Color::WHITE);
    let black = materials.add(Color::BLACK);

    let pos = Chess::default();

    commands.spawn(Game { pos: Chess::new() });

    for i in 0..8 {
        for j in 0..8 {
            let square = match i {
                0 => match j {
                    0 => Some(Square(shakmaty::Square::A8)),
                    1 => Some(Square(shakmaty::Square::A7)),
                    2 => Some(Square(shakmaty::Square::A6)),
                    3 => Some(Square(shakmaty::Square::A5)),
                    4 => Some(Square(shakmaty::Square::A4)),
                    5 => Some(Square(shakmaty::Square::A3)),
                    6 => Some(Square(shakmaty::Square::A2)),
                    7 => Some(Square(shakmaty::Square::A1)),
                    _ => None,
                },
                1 => match j {
                    0 => Some(Square(shakmaty::Square::B8)),
                    1 => Some(Square(shakmaty::Square::B7)),
                    2 => Some(Square(shakmaty::Square::B6)),
                    3 => Some(Square(shakmaty::Square::B5)),
                    4 => Some(Square(shakmaty::Square::B4)),
                    5 => Some(Square(shakmaty::Square::B3)),
                    6 => Some(Square(shakmaty::Square::B2)),
                    7 => Some(Square(shakmaty::Square::B1)),
                    _ => None,
                },
                2 => match j {
                    0 => Some(Square(shakmaty::Square::C8)),
                    1 => Some(Square(shakmaty::Square::C7)),
                    2 => Some(Square(shakmaty::Square::C6)),
                    3 => Some(Square(shakmaty::Square::C5)),
                    4 => Some(Square(shakmaty::Square::C4)),
                    5 => Some(Square(shakmaty::Square::C3)),
                    6 => Some(Square(shakmaty::Square::C2)),
                    7 => Some(Square(shakmaty::Square::C1)),
                    _ => None,
                },
                3 => match j {
                    0 => Some(Square(shakmaty::Square::D8)),
                    1 => Some(Square(shakmaty::Square::D7)),
                    2 => Some(Square(shakmaty::Square::D6)),
                    3 => Some(Square(shakmaty::Square::D5)),
                    4 => Some(Square(shakmaty::Square::D4)),
                    5 => Some(Square(shakmaty::Square::D3)),
                    6 => Some(Square(shakmaty::Square::D2)),
                    7 => Some(Square(shakmaty::Square::D1)),
                    _ => None,
                },
                4 => match j {
                    0 => Some(Square(shakmaty::Square::E8)),
                    1 => Some(Square(shakmaty::Square::E7)),
                    2 => Some(Square(shakmaty::Square::E6)),
                    3 => Some(Square(shakmaty::Square::E5)),
                    4 => Some(Square(shakmaty::Square::E4)),
                    5 => Some(Square(shakmaty::Square::E3)),
                    6 => Some(Square(shakmaty::Square::E2)),
                    7 => Some(Square(shakmaty::Square::E1)),
                    _ => None,
                },
                5 => match j {
                    0 => Some(Square(shakmaty::Square::F8)),
                    1 => Some(Square(shakmaty::Square::F7)),
                    2 => Some(Square(shakmaty::Square::F6)),
                    3 => Some(Square(shakmaty::Square::F5)),
                    4 => Some(Square(shakmaty::Square::F4)),
                    5 => Some(Square(shakmaty::Square::F3)),
                    6 => Some(Square(shakmaty::Square::F2)),
                    7 => Some(Square(shakmaty::Square::F1)),
                    _ => None,
                },
                6 => match j {
                    0 => Some(Square(shakmaty::Square::G8)),
                    1 => Some(Square(shakmaty::Square::G7)),
                    2 => Some(Square(shakmaty::Square::G6)),
                    3 => Some(Square(shakmaty::Square::G5)),
                    4 => Some(Square(shakmaty::Square::G4)),
                    5 => Some(Square(shakmaty::Square::G3)),
                    6 => Some(Square(shakmaty::Square::G2)),
                    7 => Some(Square(shakmaty::Square::G1)),
                    _ => None,
                },
                7 => match j {
                    0 => Some(Square(shakmaty::Square::H8)),
                    1 => Some(Square(shakmaty::Square::H7)),
                    2 => Some(Square(shakmaty::Square::H6)),
                    3 => Some(Square(shakmaty::Square::H5)),
                    4 => Some(Square(shakmaty::Square::H4)),
                    5 => Some(Square(shakmaty::Square::H3)),
                    6 => Some(Square(shakmaty::Square::H2)),
                    7 => Some(Square(shakmaty::Square::H1)),
                    _ => None,
                },
                _ => None,
            };

            let mesh = Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0)));

            if let Some(s) = square {
                let m = MaterialMesh2dBundle {
                    mesh: mesh.clone(),
                    // Change material according to position to get alternating pattern
                    material: if (i + j + 1) % 2 == 0 {
                        white.clone()
                    } else {
                        black.clone()
                    },
                    transform: Transform::from_xyz(
                        (i * 100 - 350) as f32,
                        (j * 100 - 350) as f32,
                        0.1,
                    ),

                    ..Default::default()
                };

                commands.spawn((
                    m,
                    On::<Pointer<Drop>>::target_component_mut::<Transform>(|drag, transform| {
                        println!("dropped");
                    }),
                ));
                let texture: Option<(Handle<Image>, Piece)> = match pos.board().piece_at(s.0) {
                    Some(piece) => match piece.role {
                        shakmaty::Role::Pawn => Some((
                            asset_server.load(format!("Piece=Pawn, Side={}.png", piece.color)),
                            Piece(piece),
                        )),
                        shakmaty::Role::Knight => Some((
                            asset_server.load(format!("Piece=Knight, Side={}.png", piece.color)),
                            Piece(piece),
                        )),
                        shakmaty::Role::Bishop => Some((
                            asset_server.load(format!("Piece=Bishop, Side={}.png", piece.color)),
                            Piece(piece),
                        )),
                        shakmaty::Role::Rook => Some((
                            asset_server.load(format!("Piece=Rook, Side={}.png", piece.color)),
                            Piece(piece),
                        )),
                        shakmaty::Role::Queen => Some((
                            asset_server.load(format!("Piece=Queen, Side={}.png", piece.color)),
                            Piece(piece),
                        )),
                        shakmaty::Role::King => Some((
                            asset_server.load(format!("Piece=King, Side={}.png", piece.color)),
                            Piece(piece),
                        )),
                    },
                    None => None,
                };

                if let Some((t, p)) = texture {
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                (i * 100 - 350) as f32,
                                (j * 100 - 350) as f32,
                                0.3,
                            ),
                            texture: t,
                            ..default()
                        },
                        p,
                        DragAndDropArea,
                        On::<Pointer<Drag>>::target_component_mut::<Transform>(
                            |drag, transform| {
                                transform.translation.y -= drag.delta.y;
                                transform.translation.x += drag.delta.x;
                            },
                        ),
                        On::<Pointer<Drop>>::target_component_mut::<Transform>(
                            |drag, transform| {
                                println!("dropped");
                            },
                        ),
                    ));
                }

                commands.spawn((
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
                    s,
                    DragAndDropArea,
                    On::<Pointer<Drop>>::target_component_mut::<Transform>(|drag, transform| {
                        println!("dropped");
                    }),
                ));
            }
        }
    }

    let legals = pos.legal_moves();
    assert_eq!(legals.len(), 20);

    pos.board();
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Chess"),
                        present_mode: bevy_window::PresentMode::AutoVsync,
                        resolution: (800., 800.).into(),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            DefaultPickingPlugins,
        ))
        .add_systems(Startup, startup)
        .add_systems(Update, chess)
        .run();
}

#[derive(Component)]
struct Game {
    pos: Chess,
}

fn chess(mut commands: Commands, game: Query<&mut Game>, pieces: Query<&Transform, With<Piece>>) {}
