use bevy::prelude::*;
use shakmaty::{Chess, FromSetup, Position};

use crate::{utils::get_texture, Game, Piece, Square};

pub(crate) fn update(
    q_games: Query<&mut Game>,
    mut commands: Commands,
    q_squares: Query<(Entity, &Square, Option<&Children>)>,
    q_pieces: Query<(Entity, &Piece, &Parent)>,
    asset_server: Res<AssetServer>,
) {
    let game = q_games.get_single().expect("Game not found!");
    let chess = Chess::from_setup(
        shakmaty::Setup {
            board: shakmaty::Board::from_bitboards(game.board.by_role, game.board.by_color),
            turn: game.turn,
            castling_rights: game.castling_rights,
            ..default()
        },
        shakmaty::CastlingMode::Standard,
    )
    .expect("Chess could not load!");
    for square in q_squares.iter() {
        let mut piece_component: Option<(Entity, &Piece)> = None;

        if let Some(children) = square.2 {
            for &child in children.iter() {
                if let Ok(piece) = q_pieces.get(child) {
                    piece_component = Some((child, piece.1));
                    break;
                }
            }
        }

        let piece = chess.board().piece_at(square.1.square);
        let res = get_texture(piece, &asset_server);
        if let Some((texture, piece)) = res {
            if let Some(piece_component) = piece_component {
                if piece_component.1 .0 != piece {
                    commands
                        .entity(square.0)
                        .remove_children(&[piece_component.0]);
                    commands.entity(piece_component.0).despawn();

                    let child = commands
                        .spawn((
                            Piece(piece),
                            SpriteBundle {
                                texture: texture.clone(),
                                ..default()
                            },
                        ))
                        .id();

                    commands.entity(square.0).push_children(&[child]);
                }
            } else {
                let child = commands
                    .spawn((
                        Piece(piece),
                        SpriteBundle {
                            texture: texture.clone(),
                            ..default()
                        },
                    ))
                    .id();

                commands.entity(square.0).push_children(&[child]);
            }
        } else if let Some(piece_component) = piece_component {
            commands
                .entity(square.0)
                .remove_children(&[piece_component.0]);
            commands.entity(piece_component.0).despawn();
        }
    }
}
