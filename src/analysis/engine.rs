use std::{path::PathBuf, str::FromStr};

use bevy::{
    prelude::*,
    tasks::futures_lite::future,
    tasks::{block_on, AsyncComputeTaskPool, Task},
};

use bevy::utils::HashMap;
use shakmaty::{fen::Fen, Chess, FromSetup};

use crate::{ui::SpawnEngineIndicator, Game, Square};

#[derive(Component, Clone, Debug)]
pub(crate) struct Engine(pub(crate) PathBuf);

#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct EngineIndicator;

#[derive(Event)]
pub(crate) struct EngineEvent;

#[derive(Resource)]
pub(crate) struct EngineTasks(pub(crate) HashMap<String, Task<String>>);

pub(crate) fn send_to_engines(
    mut rm_tasks: ResMut<EngineTasks>,
    q_engines: Query<&Engine>,
    q_games: Query<&Game>,
    mut ev_engine: EventReader<EngineEvent>,
) {
    let game = q_games.single().clone();

    for _ in ev_engine.read() {
        for engine in q_engines.iter() {
            let task_pool = AsyncComputeTaskPool::get();
            let e = engine.clone();

            let task = task_pool.spawn(async move {
                let e = uci::Engine::new(e.0.to_str().unwrap()).unwrap();
                let chess = Chess::from_setup(
                    shakmaty::Setup {
                        board: shakmaty::Board::from_bitboards(
                            game.board.by_role,
                            game.board.by_color,
                        ),
                        turn: game.turn,
                        castling_rights: game.castling_rights,
                        ..default()
                    },
                    shakmaty::CastlingMode::Standard,
                )
                .expect("Chess could not load!");

                if e.set_position(
                    Fen::from_position(chess, shakmaty::EnPassantMode::Always)
                        .to_string()
                        .as_str(),
                )
                .is_ok()
                {
                    if let Ok(best_move) = e.bestmove() {
                        return best_move;
                    }
                }
                "".to_string()
            });

            rm_tasks.0.insert("stockfish".to_string(), task);
        }
    }
}

pub(crate) fn check_engines(
    mut my_tasks: ResMut<EngineTasks>,
    mut commands: Commands,
    q_games: Query<&Game>,
    q_squares: Query<(Entity, &Square)>,
    asset_server: Res<AssetServer>,
) {
    my_tasks.0.retain(|_, task| {
        let status = block_on(future::poll_once(task));

        let retain = status.is_none();

        if let Some(data) = status {
            println!("{:?}", data);

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

            if let Ok(m) = shakmaty::uci::UciMove::from_str(&data) {
                let m = m.to_move(&chess);
                let square_entities: HashMap<shakmaty::Square, Entity> = q_squares
                    .into_iter()
                    .map(|x| {
                        let square = x.1 .0;
                        let entity = x.0;
                        (square, entity)
                    })
                    .collect();

                if let Ok(m) = m {
                    let to_square = square_entities.get(&m.to()).unwrap();
                    // let from_square = square_entities.get(&m.from().unwrap()).unwrap();
                    commands.spawn_engine_indicator(to_square, &asset_server);
                }
            }
        }

        retain
    });
}
