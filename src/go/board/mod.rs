use go::{Group, Player, Stone};

use std::hash::Hash;
use std::collections::HashSet;

#[cfg(test)]
mod test;

/// The board trait
///
/// If something implements this, go can be played on it
pub trait Board: Sized + Eq + Hash + Clone {
    /// The Position the board uses
    type Position: Sized + Eq + Hash + Copy + Clone;

    /// Constructs a new empty board
    fn new() -> Self;

    /// Returns true if the position is on the board
    fn on_board(&self, position: &Self::Position) -> bool;

    /// Returns the stone at the given position
    fn at(&self, position: &Self::Position) -> Stone;

    /// Sets the stone at the given position
    fn set(&mut self, position: &Self::Position, stone: &Stone);

    /// Sets the requested amount of handicap stones
    fn set_handicap(&mut self, stones: u8);

    /// Returns all positions.
    fn positions(&self) -> Vec<Self::Position>;

    /// Returns the vector of stone next to the given position
    ///
    /// Does not only return occupied fields but also empty ones.
    fn neighbors(&self, position: &Self::Position) -> Vec<Self::Position>;

    /// Returns the possibly empty group that contains the position
    fn group_at<'boardlt>(&'boardlt self, position: &Self::Position) -> Group<'boardlt, Self> {
        Group::new(self, position)
    }

    /// Returns the vector of groups that have a liberty at the given position
    fn groups_with_liberty_at<'boardlt>(&'boardlt self,
                                        position: &Self::Position)
                                        -> Vec<Group<'boardlt, Self>> {
        if self.at(position) != Stone::Empty {
            return Vec::new();
        }

        let mut found_groups = Vec::<Group<Self>>::new();
        for pos in &self.neighbors(position) {
            if found_groups.iter().any(|g| g.contains(pos)) {
                continue;
            }

            found_groups.push(Group::new(self, pos));
        }

        found_groups
    }

    /// Returns the set of stones that would be captured if the given player plays at the given position
    fn would_be_captured(&self,
                         player: &Player,
                         position: &Self::Position)
                         -> HashSet<Self::Position> {
        self.groups_with_liberty_at(position)
            .iter()
            .filter(|g| g.stone().unwrap_or(Stone::Empty) != player.stone() && g.liberties().len() == 1)
            .flat_map(|g| g.positions.iter())
            .cloned()
            .collect()
    }

    /// Returns if a play here would be suicide
    ///
    /// Returns false if a play at position by player would:
    /// * kill something
    /// * connect own groups that have at least two remaining liberties
    ///
    /// If none of those match, returns true if a friendly neighboring group looses
    /// its last liberty. Note that it returns true if it is a suicidal move.
    fn would_be_suicide(&self, position: &Self::Position, player: &Player) -> bool {
        //  OOOO   consider X to play in the middle
        // .X.XO   the left X has still a remaining liberty
        //  OOOO   => no group of X can die
        let mut friendly_looses_last_liberty = false;

        for group in self.groups_with_liberty_at(position).iter() {
            let liberties = group.liberties();
            let group_owner = group.stone().unwrap_or(Stone::Empty);

            if liberties.len() == 1 && group_owner == player.other().stone() {
                return false; //we kill something
            }

            if liberties.len() == 1 && group_owner == player.stone() {
                friendly_looses_last_liberty = true;
            }

            if liberties.len() > 1 && group_owner == player.stone() {
                return false; //a friendly stone has a remaining liberty
            }
        }

        friendly_looses_last_liberty
    }

    /// Fills all empty intersections that neighbor a stone with the given color by
    /// stones of that color. Repeats until nothing changes.
    fn erode(&mut self, stone: Stone)
    {
        let mut change = true;
        let positions = self.positions();

        while change {
            change = false;

            let empty_positions: Vec<_> = positions.iter()
                .filter(|pos| self.at(pos) == Stone::Empty)
                .collect();

            for empty_position in empty_positions {
                let any_set = self.neighbors(empty_position)
                    .iter()
                    .any(|pos| self.at(pos) == stone);

                if any_set {
                    self.set(empty_position, &stone);
                    change = true;
                }
            }
        }
    }

    fn area_scoring(&self) -> (usize, usize)
    {
        let mut white_board = self.clone();
        let mut black_board = self.clone();

        white_board.erode(Stone::White);
        black_board.erode(Stone::Black);

        // A position is either:
        // + played by me (me_board = me, other_board = me),
        // + my territory (me_board = me, other_board = empty),
        // ~ seki (me_board = me, other_board = other),
        // ~ not mine (me_board != me).

        let white_score = self.positions()
            .iter()
            .filter(|pos| white_board.at(pos) == Stone::White || black_board.at(pos) != Stone::Black)
            .count();

        let black_score = self.positions()
            .iter()
            .filter(|pos| black_board.at(pos) == Stone::Black || white_board.at(pos) != Stone::White)
            .count();

        (black_score, white_score)
    }
}