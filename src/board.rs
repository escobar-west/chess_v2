use std::collections::HashMap;
use std::str::FromStr;
use crate::pieces::*;

pub struct Coordinate {
    pub rank: u8,
    pub file: u8,
}

#[derive(Debug)]
pub struct Board([[Option<Piece>; 8]; 8]);

impl Board {
    pub fn new() -> Board {
        Board([[None; 8]; 8])
    }

    pub fn insert_piece(&mut self, rank: u8, file: u8, piece: Piece) -> Option<Piece> {
        let rank_index = (rank - 1) as usize;
        let file_index = (file - 1) as usize;
        self.0[rank_index][file_index].replace(piece)
    }
}

#[derive(Debug)]
pub struct GameState {
    board: Board,
    active_color: Color,
    white_king_castle: bool,
    white_queen_castle: bool,
    black_king_castle: bool,
    black_queen_castle: bool,
    en_passant: String,
    halfmove_clock: u32,
    fullmove_number: u32,
}

impl FromStr for GameState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s.split(' ');
        let board_rows = fields.next().ok_or(())?;
        println!("{}", board_rows);

        let mut board = Board::new();
        let mut board_rows = board_rows.split('/');
        let mut file_idx: u8;
        for rank_idx in (1u8..=8u8).rev() {
            println!("{}", rank_idx);
            file_idx = 1;
            for char in board_rows.next().unwrap().chars() {
                println!("{}", char);
                if char.is_ascii_digit() {
                    file_idx += char.to_digit(9).unwrap() as u8;
                } else {
                    let piece: Piece = char.try_into()?;
                    board.insert_piece(rank_idx, file_idx, piece);
                    file_idx += 1;
                }
            }
        }

        let active_color = match fields.next().ok_or(())? {
            "w" => Color::White,
            _ => Color::Black,
        };

        let castle_field = fields.next().ok_or(())?;

        let white_king_castle = castle_field.contains('K');
        let white_queen_castle = castle_field.contains('Q');
        let black_king_castle = castle_field.contains('k');
        let black_queen_castle = castle_field.contains('q');

        let en_passant = fields.next().ok_or(())?.to_string();

        let halfmove_clock = fields.next().ok_or(())?.parse::<u32>().unwrap_or_default();
        let fullmove_number = fields.next().ok_or(())?.parse::<u32>().unwrap_or_default();

        Ok(GameState {
            board,
            active_color,
            white_king_castle,
            white_queen_castle,
            black_king_castle,
            black_queen_castle,
            en_passant,
            halfmove_clock,
            fullmove_number,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let from_fen: GameState = fen.parse().unwrap();
        println!("{:#?}", from_fen);
    }

    #[test]
    fn test_sicilian_fen() {
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
        let game: GameState = fen.parse().unwrap();
        println!("{:#?}", game);
    }
}
