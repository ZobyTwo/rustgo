#![allow(dead_code)]

#[cfg(test)]
mod test;

/// A game state
pub trait GameState {
    /// constructs the initial game state
    fn new() -> Self;
}


/// An game action
pub trait Action {
    /// The states these actions modify
    type GameState: GameState;

    /// Tests if the action is applicable to the given state
    fn test(self: &Self, state: &Self::GameState) -> bool;

    /// Executes the action on the given state
    fn execute(self: &Self, state: &mut Self::GameState);
}

/// An history item for use in the game tree
#[derive(Debug)]
struct HistoryItem<SomeAction>
    where SomeAction: Action
{
    /// The path to the parent item
    parent: Path,

    /// An action to be executed after the parent iten
    action: SomeAction,
}

/// The game tree
///
/// A game is a tree of history items representing actions.
/// This allows for easy undo/redo. Represents the tree
/// as a flat array of items interlinked by parent-ids.
#[derive(Debug)]
pub struct Game<SomeAction>
    where SomeAction: Action
{
    data: Vec<HistoryItem<SomeAction>>,
}

/// The path to one game tree item
///
/// Stores the path as an id to the parent item.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Path {
    /// There is no parent (we mean the trees root)
    Empty,
    /// The parents id
    HistoryItemId(usize),
}

impl<SomeAction> Game<SomeAction>
    where SomeAction: Action
{
    /// Creates a new game
    pub fn new() -> Self {
        Game { data: Vec::new() }
    }

    /// Inserts the action after parent
    ///
    /// Does reconstruct the game state at path and applies action
    pub fn insert(self: &mut Self, parent: &Path, action: SomeAction) -> Path {
        let state = self.get_state(parent);

        if action.test(&state) {
            self.data.push(HistoryItem {
                parent: parent.clone(),
                action: action,
            });

            Path::HistoryItemId(self.data.len() - 1)
        } else {
            Path::Empty
        }
    }

    /// Returns the state at the given path
    ///
    /// Does reapply all previous actions
    pub fn get_state(self: &Self, at: &Path) -> SomeAction::GameState {
        let mut state = SomeAction::GameState::new();

        if let &Path::HistoryItemId(up_to) = at {
            let mut path = Vec::<usize>::new();
            let mut curr = up_to;

            while let Path::HistoryItemId(next) = self.data[curr].parent {
                path.push(curr);
                curr = next;
            }

            path.push(curr);

            for idx in path.iter().rev() {
                self.data[*idx].action.execute(&mut state);
            }
        }

        state
    }
}
