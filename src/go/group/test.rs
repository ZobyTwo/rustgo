#[cfg(test)]
mod test {
    use aga::{Board19x19, Position19x19};
    use go::{Board, Stone, Group};

    #[test]
    fn create() {
        let mut board = Board19x19::new();
        board.set(&Position19x19 { x: 4, y: 4 }, &Stone::Black);
        board.set(&Position19x19 { x: 8, y: 8 }, &Stone::White);
        board.set(&Position19x19 { x: 8, y: 9 }, &Stone::White);

        let empty_group = Group::new(&board, &Position19x19 { x: 0, y: 0 });
        let black_group = Group::new(&board, &Position19x19 { x: 4, y: 4 });
        let white_group = Group::new(&board, &Position19x19 { x: 8, y: 8 });
        let alternative = Group::new(&board, &Position19x19 { x: 8, y: 9 });

        assert_eq!(empty_group.positions.len(), 0);
        assert_eq!(black_group.positions.len(), 1);
        assert_eq!(white_group.positions.len(), 2);
        assert_eq!(white_group, alternative);
    }

    #[test]
    fn liberties() {
        let mut board = Board19x19::new();
        board.set(&Position19x19 { x: 7, y: 8 }, &Stone::White); //   .
        board.set(&Position19x19 { x: 8, y: 7 }, &Stone::Black); //  .O.
        board.set(&Position19x19 { x: 8, y: 8 }, &Stone::White); //  XOO.
        board.set(&Position19x19 { x: 8, y: 9 }, &Stone::White); //   ..

        let white_group = Group::new(&board, &Position19x19 { x: 8, y: 8 });
        assert_eq!(white_group.liberties().len(), 6);
    }
}
