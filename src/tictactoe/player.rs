#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Cross,
    Circle,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Cross => return write!(f, "X"),
            Self::Circle => return write!(f, "O"),
        };
    }
}
