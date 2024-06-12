use super::board::{Board, Slot};
use super::player::Player;

#[repr(u8)]
#[derive(PartialEq, Eq)]
pub enum State {
    Playing,
    Draw,
    Won(Player),
}

impl State {
    pub(super) fn check_horizontal(board: &Board) -> Self {
        for row in &board.rows {
            if row[0] == row[1] && row[1] == row[2] {
                return match row[0] {
                    Slot::Occupied(player) => Self::Won(player),
                    Slot::Empty => Self::Playing,
                };
            };
        }

        Self::Playing
    }

    pub(super) fn check_diagonal(board: &Board) -> Self {
        let rows = &board.rows[..];
        let pattern_found = (rows[0][0] == rows[1][1] && rows[1][1] == rows[2][2])
            || (rows[0][2] == rows[1][1] && rows[1][1] == rows[2][0]);

        if pattern_found {
            return match rows[1][1] {
                Slot::Occupied(player) => Self::Won(player),
                Slot::Empty => Self::Playing,
            };
        };

        Self::Playing
    }

    pub(super) fn check_vertical(board: &Board) -> Self {
        let rows = &board.rows[..];

        for i in 0..3 {
            if rows[0][i] == rows[1][i] && rows[1][i] == rows[2][i] {
                return match rows[0][i] {
                    Slot::Occupied(player) => Self::Won(player),
                    Slot::Empty => Self::Playing,
                };
            };
        }

        Self::Playing
    }

    pub(super) fn is_board_full(board: &Board) -> bool {
        for row in &board.rows {
            for slot in row {
                if *slot == Slot::Empty {
                    return false;
                };
            }
        }

        true
    }
}
