use super::{Game, GameState, Action, Path};

struct SimpleGameState {
    acc: i32,
}

impl GameState for SimpleGameState {
    fn new() -> SimpleGameState {
        SimpleGameState { acc: 0 }
    }
}

enum SimpleAction {
    Inc,
    Dec,
}

impl Action for SimpleAction {
    type GameState = SimpleGameState;

    fn test(self: &Self, state: &SimpleGameState) -> bool {
        match self {
            &SimpleAction::Inc => true,
            &SimpleAction::Dec => state.acc > 0,
        }
    }

    fn execute(self: &Self, state: &mut SimpleGameState) {
        match self {
            &SimpleAction::Inc => state.acc += 1,
            &SimpleAction::Dec => state.acc -= 1,
        }
    }
}

#[test]
fn tree() {
    let mut g = Game::<SimpleAction>::new();
    let root_cursor = Path::Empty;

    let parent_cursor = g.insert(&root_cursor, SimpleAction::Inc);
    assert!(g.get_state(&parent_cursor).acc == 1);

    let invalid_cursor = g.insert(&root_cursor, SimpleAction::Dec);
    assert!(invalid_cursor == Path::Empty);

    let child_0 = g.insert(&parent_cursor, SimpleAction::Dec);
    assert!(child_0 != Path::Empty);
    assert!(g.get_state(&child_0).acc == 0);

    let child_1 = g.insert(&parent_cursor, SimpleAction::Inc);
    assert!(child_1 != Path::Empty);
    assert!(g.get_state(&child_1).acc == 2);
}