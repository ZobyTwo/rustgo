use go::{Board, Stone};
use aga::Position19x19;

/// A default 19x19 go board
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Board19x19 {
    state: [[Stone; 19]; 19],
}

impl Board for Board19x19 {
    type Position = Position19x19;

    fn new() -> Self {
        Board19x19 { state: [[Stone::Empty; 19]; 19] }
    }

    fn on_board(&self, position: &Position19x19) -> bool {
        position.x < 19 && position.y < 19
    }

    fn at(&self, position: &Position19x19) -> Stone {
        self.state[position.y][position.x]
    }

    fn set(&mut self, position: &Position19x19, stone: &Stone) {
        self.state[position.y][position.x] = *stone;
    }

    fn set_handicap(&mut self, stones: u8) {
        if 2 <= stones && stones <= 9 {
            // upper right and lower left
            self.set(&Position19x19 { x: 14, y: 4 }, &Stone::Black);
            self.set(&Position19x19 { x: 4, y: 14 }, &Stone::Black);
        }
        if 3 <= stones && stones <= 9 {
            // lower right
            self.set(&Position19x19 { x: 14, y: 14 }, &Stone::Black);
        }
        if 4 <= stones && stones <= 9 {
            // upper left
            self.set(&Position19x19 { x: 4, y: 4 }, &Stone::Black);
        }
        if stones == 5 || stones == 7 || stones == 9 {
            // middle
            self.set(&Position19x19 { x: 10, y: 10 }, &Stone::Black);
        }
        if 6 <= stones && stones <= 9 {
            // left side and right side
            self.set(&Position19x19 { x: 4, y: 10 }, &Stone::Black);
            self.set(&Position19x19 { x: 14, y: 10 }, &Stone::Black);
        }
        if stones == 8 || stones == 9 {
            // upper side and lower side
            self.set(&Position19x19 { x: 10, y: 4 }, &Stone::Black);
            self.set(&Position19x19 { x: 10, y: 14 }, &Stone::Black);
        }
    }

    fn positions(&self) -> Vec<Position19x19> {
        let mut n = Vec::<Position19x19>::new();
        for x in 0..19 {
            for y in 0..19 {
                n.push(Position19x19 { x: x, y: y });
            }
        }

        n
    }

    fn neighbors(&self, position: &Position19x19) -> Vec<Position19x19> {
        let mut n = Vec::<Position19x19>::new();

        if position.x < 18 {
            n.push(Position19x19 {
                x: position.x + 1,
                y: position.y,
            });
        }
        if position.x > 0 {
            n.push(Position19x19 {
                x: position.x - 1,
                y: position.y,
            });
        }
        if position.y < 18 {
            n.push(Position19x19 {
                x: position.x,
                y: position.y + 1,
            });
        }
        if position.y > 0 {
            n.push(Position19x19 {
                x: position.x,
                y: position.y - 1,
            });
        }

        n
    }
}