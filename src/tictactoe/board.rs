use super::{
    game_state::State::{self, *},
    player::{Move, Player},
};

#[derive(PartialEq, Eq)]
pub(super) enum Slot {
    Empty,
    Occupied(Player),
}

impl std::fmt::Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Slot::Empty => write!(f, " "),
            Slot::Occupied(Player::X) => write!(f, "X"),
            Slot::Occupied(Player::O) => write!(f, "O"),
        }
    }
}

pub struct Board {
    pub(super) rows: [[Slot; 3]; 3],
    pub current_player: Player,
}

impl Board {
    pub fn new() -> Self {
        Self {
            rows: [
                [Slot::Empty, Slot::Empty, Slot::Empty],
                [Slot::Empty, Slot::Empty, Slot::Empty],
                [Slot::Empty, Slot::Empty, Slot::Empty],
            ],
            current_player: Player::O,
        }
    }

    pub fn parse_move(&self, m: String) -> Move {
        if m.trim() == "exit" {
            return Move::Exit;
        }

        if m.trim().len() != 2 {
            return Move::Invalid;
        };

        let bytes = m.trim().as_bytes();

        let row: usize = match bytes[0] {
            b'a' => 0,
            b'b' => 1,
            b'c' => 2,
            _ => usize::MAX,
        };

        let column: usize = match bytes[1] {
            b'1' => 0,
            b'2' => 1,
            b'3' => 2,
            _ => usize::MAX,
        };

        if row == usize::MAX || column == usize::MAX {
            return Move::Invalid;
        };

        match self.rows[row][column] {
            Slot::Empty => Move::Valid(row, column),
            Slot::Occupied(_) => Move::Invalid,
        }
    }

    pub fn play_move(&mut self, m: Move) -> Result<State, ()> {
        let (row, column) = match m {
            Move::Valid(r, c) => (r, c),
            _ => return Err(()),
        };

        self.rows[row][column] = Slot::Occupied(self.current_player);

        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        Ok(self.check_state())
    }

    pub fn check_state(&self) -> State {
        if let Won(player) = State::check_vertical(self) {
            return Won(player);
        };
        if let Won(player) = State::check_diagonal(self) {
            return Won(player);
        };
        if let Won(player) = State::check_horizontal(self) {
            return Won(player);
        };

        if State::is_board_full(self) {
            return Draw;
        };

        Playing
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        format!(
            "  1  2  3
a {}  {}  {}
b {}  {}  {}
c {}  {}  {}\n",
            self.rows[0][0],
            self.rows[0][1],
            self.rows[0][2],
            self.rows[1][0],
            self.rows[1][1],
            self.rows[1][2],
            self.rows[2][0],
            self.rows[2][1],
            self.rows[2][2]
        )
    }
}
