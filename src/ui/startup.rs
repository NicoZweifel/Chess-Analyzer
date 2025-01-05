use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_mod_picking::prelude::*;
use shakmaty::{Chess, Position};

use super::get_piece_texture;
use crate::{
    picking::{DragEndEvent, DragEvent, DropEvent, SelectEvent},
    Piece, Square,
};

#[derive(Bundle)]
struct Pickable {
    drop: On<Pointer<Drop>>,
    drag: On<Pointer<Drag>>,
    down: On<Pointer<Down>>,
    drag_end: On<Pointer<DragEnd>>,
}

impl Default for Pickable {
    fn default() -> Self {
        Self {
            drop: On::<Pointer<Drop>>::send_event::<DropEvent>(),
            down: On::<Pointer<Down>>::send_event::<SelectEvent>(),
            drag: On::<Pointer<Drag>>::send_event::<DragEvent>(),
            drag_end: On::<Pointer<DragEnd>>::send_event::<DragEndEvent>(),
        }
    }
}

#[derive(Bundle)]
struct SquarePickingBundle<M: Material2d> {
    material_mesh_2d: MaterialMesh2dBundle<M>,
    square: Square,
    pickable: Pickable,
}

impl<M: Material2d> Default for SquarePickingBundle<M> {
    fn default() -> Self {
        Self {
            pickable: default(),
            material_mesh_2d: default(),
            square: Square(shakmaty::Square::A1),
        }
    }
}

#[derive(Bundle)]
struct SquareBundle<M: Material2d> {
    material_mesh_2d: MaterialMesh2dBundle<M>,
}

impl<M: Material2d> Default for SquareBundle<M> {
    fn default() -> Self {
        Self {
            material_mesh_2d: default(),
        }
    }
}

#[derive(Bundle)]
struct PieceBundle {
    piece: Piece,
    sprite: SpriteBundle,
}

impl Default for PieceBundle {
    fn default() -> Self {
        Self {
            piece: Piece(shakmaty::Piece {
                color: shakmaty::Color::White,
                role: shakmaty::Role::King,
            }),
            sprite: default(),
        }
    }
}

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

    for (counter, square) in shakmaty::Square::ALL.iter().enumerate() {
        let mesh = Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0)));

        let j = (counter as f32 / 8.0).floor() as i32;
        let i = (counter % 8) as i32;

        let texture = get_piece_texture(start.board().piece_at(*square), &asset_server);

        commands.spawn(SquareBundle {
            material_mesh_2d: MaterialMesh2dBundle {
                mesh: mesh.clone(),
                // Change material according to position to get alternating pattern
                material: if (i + j + 1) % 2 == 0 {
                    white.clone()
                } else {
                    black.clone()
                },
                transform: Transform::from_xyz((i * 100 - 350) as f32, (j * 100 - 350) as f32, 0.1),
                ..default()
            },
        });

        commands
            .spawn(SquarePickingBundle {
                material_mesh_2d: MaterialMesh2dBundle {
                    mesh: mesh.clone(),
                    material: materials.add(Color::hsla(0.0, 0.0, 0.0, 0.0)),
                    transform: Transform::from_xyz(
                        (i * 100 - 350) as f32,
                        (j * 100 - 350) as f32,
                        0.2,
                    ),
                    ..default()
                },
                square: Square(*square),
                ..default()
            })
            .with_children(|parent| {
                if let Some((texture, piece)) = texture {
                    parent.spawn(PieceBundle {
                        piece: Piece(piece),
                        sprite: SpriteBundle {
                            texture,
                            ..default()
                        },
                    });
                }
            });
    }
}
