use go::Stone;
use go::Board;

use std::collections::HashSet;

#[cfg(test)]
mod test;

/// A group of stones on a specific board
///
/// Internally a set of positions and a reference to
/// it's containing board. It is not guarded against
/// changing the board.
#[derive(PartialEq, Eq, Debug)]
pub struct Group<'boardlt, TBoard>
    where TBoard: Board + 'boardlt
{
    /// The positions that are part of the group
    pub positions: HashSet<TBoard::Position>,
    /// The board containing the stones
    board: &'boardlt TBoard,
}

impl<'boardlt, TBoard> Group<'boardlt, TBoard>
    where TBoard: Board + 'boardlt
{
    /// Creates a new group
    ///
    /// Collects all stones with the same color at the given position.
    /// If there is no stone, the group will be empty.
    pub fn new(board: &'boardlt TBoard, position: &TBoard::Position) -> Group<'boardlt, TBoard> {
        match board.at(position) {
            Stone::Empty => {
                Group {
                    positions: HashSet::new(),
                    board: board,
                }
            }
            stone => {
                let mut stack = Vec::<TBoard::Position>::new();
                let mut content = HashSet::<TBoard::Position>::new();

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

    /// Returns a view into the contained positions
    pub fn positions(&self) -> &HashSet<TBoard::Position> {
        &self.positions
    }

    /// Returns the hashset of positions that are liberties of the group
    pub fn liberties(&self) -> HashSet<TBoard::Position> {
        self.positions
            .iter()
            .flat_map(|p| self.board.neighbors(p))
            .filter(|p| self.board.at(p) == Stone::Empty)
            .collect()
    }

    /// Returns the groups stone-color
    pub fn stone(&self) -> Option<Stone> {
        self.positions.iter().next().map(|p| self.board.at(p))
    }
}
