use go::{Player, Stone};

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
