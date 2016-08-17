use aga::{Board19x19, Position19x19};
use go::{Board, Stone, Player};

#[test]
fn groups_with_liberty_at() {
    let mut board = Board19x19::new();

    board.set(&Position19x19 { x: 4, y: 3 }, &Stone::White); //
    board.set(&Position19x19 { x: 3, y: 4 }, &Stone::Black); // XX
    board.set(&Position19x19 { x: 2, y: 3 }, &Stone::Black); // X.X
    board.set(&Position19x19 { x: 3, y: 2 }, &Stone::Black); //  O
    board.set(&Position19x19 { x: 2, y: 2 }, &Stone::Black);

    let groups = board.groups_with_liberty_at(&Position19x19 { x: 3, y: 3 });
    assert_eq!(groups.len(), 3);
}

#[test]
fn board_neighbors() {
    let board = Board19x19::new();

    assert_eq!(board.neighbors(&Position19x19 { x: 0, y: 5 }).len(), 3);
    assert_eq!(board.neighbors(&Position19x19 { x: 9, y: 9 }).len(), 4);
    assert_eq!(board.neighbors(&Position19x19 { x: 0, y: 0 }).len(), 2);
}

#[test]
fn board_would_be_captured() {
    let mut board = Board19x19::new();

    board.set(&Position19x19 { x: 0, y: 0 }, &Stone::White); // OTO.
    board.set(&Position19x19 { x: 0, y: 1 }, &Stone::Black); // XOX.
    board.set(&Position19x19 { x: 1, y: 1 }, &Stone::White); // .X.
    board.set(&Position19x19 { x: 1, y: 2 }, &Stone::Black); //  .
    board.set(&Position19x19 { x: 2, y: 0 }, &Stone::White); // gonna play with X at T
    board.set(&Position19x19 { x: 2, y: 1 }, &Stone::Black); // should capture both white stones

    assert_eq!(board.would_be_captured(&Player::Black, (&Position19x19 { x: 1, y: 0 })).len(),
               2);
}
