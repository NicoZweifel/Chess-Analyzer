use super::get_piece_texture;
use crate::{audio::SoundEvent, Game, Piece, Square};
use bevy::prelude::*;
use shakmaty::{Chess, FromSetup, Position};

pub(crate) fn update(
    q_games: Query<&mut Game>,
    mut commands: Commands,
    q_squares: Query<(Entity, &Square, Option<&Children>)>,
    q_pieces: Query<(Entity, &Piece, &Parent)>,
    asset_server: Res<AssetServer>,
    mut evr_sounds: EventWriter<SoundEvent>,
) {
    let game = q_games.single();
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
    for (sq_entity, sq, sq_children) in q_squares.iter() {
        let mut piece_component: Option<(Entity, &Piece)> = None;

        if let Some(children) = sq_children {
            for &child in children.iter() {
                if let Ok(piece) = q_pieces.get(child) {
                    piece_component = Some((child, piece.1));
                    break;
                }
            }
        }

        let piece = chess.board().piece_at(sq.square);
        let res = get_piece_texture(piece, &asset_server);

        if let Some((texture, piece)) = res {
            if let Some((p_entity, p_component)) = piece_component {
                if p_component.0 != piece {
                    commands.entity(sq_entity).remove_children(&[p_entity]);
                    commands.entity(p_entity).despawn();

                    let child = commands
                        .spawn((
                            Piece(piece),
                            SpriteBundle {
                                texture: texture.clone(),
                                ..default()
                            },
                        ))
                        .id();

                    commands.entity(sq_entity).push_children(&[child]);

                    evr_sounds.send(SoundEvent {
                        sound: "piece-capture.mp3".to_string(),
                    });
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

                commands.entity(sq_entity).push_children(&[child]);

                evr_sounds.send(SoundEvent {
                    sound: "piece-placement.mp3".to_string(),
                });
            }
        } else if let Some((p_entity, _)) = piece_component {
            commands.entity(sq_entity).remove_children(&[p_entity]);
            commands.entity(p_entity).despawn();
        }
    }
}
