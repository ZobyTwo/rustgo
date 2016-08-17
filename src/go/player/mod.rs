use go::Stone;

#[cfg(test)]
mod test;

/// A player
///
/// Either black or white.
#[derive(Copy, PartialEq, Clone, Eq, Hash, Debug)]
pub enum Player {
    Black,
    White,
}

impl Player {
    /// Returns the other player
    pub fn other(&self) -> Player {
        match *self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    /// Returns the stone the player uses
    pub fn stone(&self) -> Stone {
        match *self {
            Player::Black => Stone::Black,
            Player::White => Stone::White,
        }
    }
}
