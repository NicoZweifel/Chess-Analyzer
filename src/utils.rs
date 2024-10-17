use bevy::prelude::*;

pub(crate) fn get_texture(
    piece: Option<shakmaty::Piece>,
    asset_server: &Res<AssetServer>,
) -> Option<(Handle<Image>, shakmaty::Piece)> {
    match piece {
        Some(piece) => match piece.role {
            shakmaty::Role::Pawn => Some((
                asset_server.load(format!("Piece=Pawn, Side={}.png", piece.color)),
                piece,
            )),
            shakmaty::Role::Knight => Some((
                asset_server.load(format!("Piece=Knight, Side={}.png", piece.color)),
                piece,
            )),
            shakmaty::Role::Bishop => Some((
                asset_server.load(format!("Piece=Bishop, Side={}.png", piece.color)),
                piece,
            )),

            shakmaty::Role::Rook => Some((
                asset_server.load(format!("Piece=Rook, Side={}.png", piece.color)),
                piece,
            )),
            shakmaty::Role::Queen => Some((
                asset_server.load(format!("Piece=Queen, Side={}.png", piece.color)),
                piece,
            )),
            shakmaty::Role::King => Some((
                asset_server.load(format!("Piece=King, Side={}.png", piece.color)),
                piece,
            )),
        },
        None => None,
    }
}
