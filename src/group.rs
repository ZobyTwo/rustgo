
use stone::Stone;
use board::BoardTrait;

use std::collections::HashSet;

/// A group of stones on a specific board
///
/// Internally a set of positions and a reference to
/// it's containing board. It is not guarded against
/// changing the board.
#[derive(PartialEq, Eq, Debug)]
pub struct Group<'boardlt, Board>
    where Board: BoardTrait + 'boardlt
{
    /// The positions that are part of the group
    pub positions: HashSet<Board::Position>,
    /// The board containing the stones
    board: &'boardlt Board,
}

impl<'boardlt, Board> Group<'boardlt, Board>
    where Board: BoardTrait + 'boardlt
{
    /// Creates a new group
    ///
    /// Collects all stones with the same color at the given position.
    /// If there is no stone, the group will be empty.
    pub fn new(board: &'boardlt Board, position: &Board::Position) -> Group<'boardlt, Board> {
        match board.at(position) {
            Stone::Empty => {
                Group {
                    positions: HashSet::new(),
                    board: board,
                }
            }
            stone => {
                let mut stack = Vec::<Board::Position>::new();
                let mut content = HashSet::<Board::Position>::new();

                stack.push(*position);
                content.insert(*position);

                while stack.len() != 0 {
                    let top = stack.pop().unwrap();

                    for n in &board.neighbors(&top) {
                        match board.at(n) {
                            Stone::Empty => {}
                            owner => {
                                if owner == stone && !content.contains(n) {
                                    content.insert(*n);
                                    stack.push(*n);
                                }
                            }
                        }
                    }
                }

                Group {
                    positions: content,
                    board: board,
                }
            }
        }
    }

    /// Returns if the position is part of the group
    pub fn contains(&self, pos: &Board::Position) -> bool {
        self.positions.contains(pos)
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.positions.len()
    }

    /// Returns the hashset of positions that are liberties of the group
    pub fn liberties(&self) -> HashSet<Board::Position> {
        self.positions
            .iter()
            .flat_map(|p| self.board.neighbors(p))
            .filter(|p| self.board.at(p) == Stone::Empty)
            .collect()
    }

    /// Returns the groups stone-color
    ///
    /// May be invalid if the board changed since creation of the group
    pub fn stone(&self) -> Stone {
        self.positions.iter().next().map_or(Stone::Empty, |p| self.board.at(p))
    }
}

#[cfg(test)]
mod test {
    use board::{Board19x19, BoardTrait};
    use position::Position19x19;
    use stone::Stone;
    use super::Group;

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

        assert_eq!(empty_group.len(), 0);
        assert_eq!(black_group.len(), 1);
        assert_eq!(white_group.len(), 2);
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
