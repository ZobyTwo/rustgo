
use stone::Stone;

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

#[test]
fn other() {
    assert_eq!(Player::Black.other(), Player::White);
    assert_eq!(Player::White.other(), Player::Black);
}

#[test]
fn to_stone() {
    assert_eq!(Player::Black.stone(), Stone::Black);
    assert_eq!(Player::White.stone(), Stone::White);
}
