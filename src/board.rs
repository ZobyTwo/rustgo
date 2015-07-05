use stone::Stone;
use position::Position19x19;
use group::Group;
use player::Player;

use std::hash::Hash;
use std::collections::HashSet;

/// The board trait
///
/// If something implements this, go can be played on it
pub trait BoardTrait : Sized + Eq + Hash + Clone {
    /// The Position the board uses
    type Position : Sized + Eq + Hash + Copy + Clone;
    
    /// Constructs a new empty board
    fn new() -> Self;

    /// Returns true if the position is on the board
    fn is_valid(&self, position: &Self::Position) -> bool;

    /// Returns the stone at the given position
    fn at(&self, position: &Self::Position) -> Stone;

    /// Sets the stone at the given position
    fn set(& mut self, position: &Self::Position, stone: &Stone);
    
    /// Sets the requested amount of handicap stones
    fn set_handicap(& mut self, stones : u8);

    /// Returns the vector of stone next to the given position
    ///
    /// Does not only return occupied fields but also empty ones.
    fn neighbors(&self, position: &Self::Position) -> Vec<Self::Position>;

    /// Returns the possibly empty group that contains the position
    fn group_at<'boardlt>(& 'boardlt self, position: &Self::Position) -> Group<'boardlt, Self> {
        Group::new(self, position)
    }

    /// Returns the vector of groups that have a liberty at the given position
    fn groups_with_liberty_at<'boardlt>(& 'boardlt self, position : &Self::Position) -> Vec<Group<'boardlt, Self>> {
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
    fn would_be_captured(&self, player : &Player, position : &Self::Position) -> HashSet<Self::Position> {
        self.groups_with_liberty_at(position).iter()
            .filter(|g| g.stone() != player.stone() && g.liberties().len() == 1)
            .flat_map(|g| g.positions.clone())
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
    fn would_be_suicide(&self, position: &Self::Position, player : &Player) -> bool {
        //  OOOO   consider X to play in the middle
        // .X.XO   the left X has still a remaining liberty
        //  OOOO   => no group of X can die
        let mut friendly_looses_last_liberty = false;

        for group in self.groups_with_liberty_at(position).iter() {
            let liberties = group.liberties();

            if liberties.len() == 1 && group.stone() == player.other().stone() {
                return false; //we kill something
            }

            if liberties.len() == 1 && group.stone() == player.stone() {
                friendly_looses_last_liberty = true;
            }

            if liberties.len() > 1 && group.stone() == player.stone() {
                return false; //a friendly stone has a remaining liberty
            }
        }

        friendly_looses_last_liberty
    }
}

/// A default 19x19 go board
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Board19x19 {
    state: [[Stone; 19]; 19]
}

impl BoardTrait for Board19x19 {
    type Position = Position19x19;
    
    fn new() -> Self {
        Board19x19 {
            state : [[Stone::Empty; 19]; 19]
        }
    }

    fn is_valid(&self, position: &Position19x19) -> bool {
        position.x < 19 && position.y < 19
    }

    fn at(&self, position: &Position19x19) -> Stone {
        self.state[position.y][position.x]
    }

    fn set(& mut self, position: &Position19x19, stone: &Stone) {
        self.state[position.y][position.x] = *stone;
    }
    
    fn set_handicap(& mut self, stones : u8) {    
        if 2 <= stones && stones <= 9 { //upper right and lower left
            self.set(&Position19x19{x: 14, y: 4}, &Stone::Black);
            self.set(&Position19x19{x: 4, y: 14}, &Stone::Black);
        }
        if 3 <= stones && stones <= 9 { //lower right
            self.set(&Position19x19{x: 14, y: 14}, &Stone::Black);
        }
        if 4 <= stones && stones <= 9 { //upper left
            self.set(&Position19x19{x: 4, y: 4}, &Stone::Black);
        }
        if stones == 5 || stones == 7 || stones == 9 { //middle
            self.set(&Position19x19{x: 10, y: 10}, &Stone::Black);
        }
        if 6 <= stones && stones <= 9 { //left side and right side
            self.set(&Position19x19{x: 4, y: 10}, &Stone::Black);
            self.set(&Position19x19{x: 14, y: 10}, &Stone::Black);
        }
        if stones == 8 || stones == 9 { //upper side and lower side
            self.set(&Position19x19{x: 10, y: 4}, &Stone::Black);
            self.set(&Position19x19{x: 10, y: 14}, &Stone::Black);
        }
    }

    fn neighbors(&self, position: &Position19x19) -> Vec<Position19x19> {
        let mut n = Vec::<Position19x19>::new();

        if position.x < 18 {
            n.push(Position19x19{x: position.x + 1, y: position.y});
        }
        if position.x > 0 {
            n.push(Position19x19{x: position.x - 1, y: position.y});
        }
        if position.y < 18 {
            n.push(Position19x19{x: position.x, y: position.y + 1});
        }
        if position.y > 0 {
            n.push(Position19x19{x: position.x, y: position.y - 1});
        }

        n
    }
}


#[test]
fn groups_with_liberty_at(){
    let mut board = Board19x19::new();

    board.set(&Position19x19{x : 4, y : 3}, &Stone::White); //
    board.set(&Position19x19{x : 3, y : 4}, &Stone::Black); // XX
    board.set(&Position19x19{x : 2, y : 3}, &Stone::Black); // X.X
    board.set(&Position19x19{x : 3, y : 2}, &Stone::Black); //  O
    board.set(&Position19x19{x : 2, y : 2}, &Stone::Black);

    let groups = board.groups_with_liberty_at(&Position19x19{x : 3, y : 3});
    assert_eq!(groups.len(), 3);
}

#[test]
fn board_neighbors(){
    let board = Board19x19::new();

    assert_eq!(board.neighbors(&Position19x19{x : 0, y : 5}).len(), 3);
    assert_eq!(board.neighbors(&Position19x19{x : 9, y : 9}).len(), 4);
    assert_eq!(board.neighbors(&Position19x19{x : 0, y : 0}).len(), 2);
}

#[test]
fn board_would_be_captured(){
    let mut board = Board19x19::new();

    board.set(&Position19x19{x : 0, y : 0}, &Stone::White); // OTO.
    board.set(&Position19x19{x : 0, y : 1}, &Stone::Black); // XOX.
    board.set(&Position19x19{x : 1, y : 1}, &Stone::White); // .X.
    board.set(&Position19x19{x : 1, y : 2}, &Stone::Black); //  .
    board.set(&Position19x19{x : 2, y : 0}, &Stone::White); // gonna play with X at T
    board.set(&Position19x19{x : 2, y : 1}, &Stone::Black); // should capture both white stones

    assert_eq!(board.would_be_captured(&Player::Black, (&Position19x19{x : 1, y : 0})).len(), 2);
}
