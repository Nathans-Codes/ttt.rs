use super::{ player::Player, game_state::State::{ self, * }, };

#[derive(PartialEq, Eq)]
pub(super) enum Slot {
    Empty,
    Occupied(Player),
}

impl std::fmt::Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Slot::Empty => write!(f, " "),
            Slot::Occupied(Player::Cross) => write!(f, "X"),
            Slot::Occupied(Player::Circle) => write!(f, "O"),
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
                [ Slot::Empty, Slot::Empty, Slot::Empty ],
                [ Slot::Empty, Slot::Empty, Slot::Empty ],
                [ Slot::Empty, Slot::Empty, Slot::Empty ],
            ],
            current_player: Player::Circle,
        }
    }

    pub fn play_move(&mut self, m: &str) {
        let m_bytes = m.as_bytes();

        let row: usize = match m_bytes[0] {
            b'a' => 0,
            b'b' => 1,
            b'c' => 2,
            _ => usize::MAX,
        };

        let column: usize = match m_bytes[1] {
            b'1' => 0,
            b'2' => 1,
            b'3' => 2,
            _ => usize::MAX,
        };

        let is_out_of_bounds = row == usize::MAX || column == usize::MAX;

        if is_out_of_bounds {
            return;
        }

        let is_legal_move = match self.rows[row][column] {
            Slot::Empty => true,
            _ => false,
        };

        if !is_legal_move {
            return;
        }

        self.rows[row][column] = Slot::Occupied(self.current_player);

        self.current_player = match self.current_player {
            Player::Cross => Player::Circle,
            Player::Circle => Player::Cross,
        };

    }

    pub fn check_state(&self) -> State {
        if let Won(player) = State::check_vertical(self) {
            return Won(player)
        };
        if let Won(player) = State::check_diagonal(self) {
            return Won(player)
        };
        if let Won(player) = State::check_horizontal(self) {
            return Won(player)
        };
        
        if State::is_board_full(self) {
            return Draw;
        };

        Playing
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "
          1  2  3
        a {}  {}  {}
        b {}  {}  {}
        c {}  {}  {}
        ", self.rows[0][0], self.rows[0][1], self.rows[0][2], 
        self.rows[1][0], self.rows[1][1], self.rows[1][2], 
        self.rows[2][0], self.rows[2][1], self.rows[2][2])
    }
}

