#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::X => return write!(f, "X"),
            Self::O => return write!(f, "O"),
        };
    }
}
