#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

#[derive(PartialEq)]
pub enum Move {
    Exit,
    Valid(usize, usize),
    Invalid,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}
