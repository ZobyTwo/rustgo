#![allow(dead_code)]
use std::collections::HashSet;

use player::Player;
use board::BoardTrait;
use position::Position;
use stone::Stone;
use game::{GameState, Action};
use std::marker::PhantomData;

/// A KoState as used by the aga super ko rules
///
/// Stores a board-layout and the current player. Such a
/// combination is not allowed to repeat with the same game. 
#[derive(Hash, PartialEq, Clone, Eq)]
struct KoState<Board>
    where Board : BoardTrait
{
    board: Board,
    player: Player
}

impl<Board> KoState<Board> 
    where Board : BoardTrait
{
    /// Constructs a KoState from a board, position and player
    fn from_move(board : &Board, position : &Position, player : &Player) -> Self {
        let mut board_copy = board.clone();

        let captured_stones = board_copy.would_be_captured(player, position);
        board_copy.set(position, &player.stone());
        for captured_stone in &captured_stones {
            board_copy.set(captured_stone, &Stone::Empty);
        }

        KoState{
            board: board_copy,
            player: player.other()
        }
    }
}

/// The state of a game as used by the aga rule set
pub struct AGAGameState<Board>
    where Board : BoardTrait
{
    /// The current board layout
    board : Board,
    /// The current number of plys in the game
    ply : u32,
    /// The current game phase
    phase : GamePhase,
    /// The positions currently marked as dead 
    dead_stones : Option<Vec<Position>>,
    /// The set of ko states that are not allowed to repeat
    ko_states : HashSet<KoState<Board>>
}

impl<Board> GameState for AGAGameState<Board>
    where Board : BoardTrait
{
    fn new() -> Self {
        AGAGameState{
            board : Board::new(),
            ply : 0,
            phase : GamePhase::Running,
            dead_stones : Option::None,
            ko_states : HashSet::new(),
        }
    }
}

impl<Board> AGAGameState<Board> 
    where Board : BoardTrait
{
    /// Return the current player
    ///
    /// Since it is not possible to make an odd number of turns
    /// or to make an action that does not require an response
    /// from the other player und aga rules, the current player
    /// is black if the ply-count is even and white otherwise.
    fn current_player(self : &Self) -> Player {
        if self.ply % 2 == 0 {
            Player::Black
        } else {
            Player::White
        }
    }

    /// Register the current game state as a ko state
    fn register_ko_state(self : &mut Self) {
        let state = KoState{
            board: self.board.clone(),
            player: self.current_player()
        };

        self.ko_states.insert(state);
    }

    /// Check if a ply at position by player would result in ko
    fn would_be_ko(self : &Self, position : &Position, player : &Player) -> bool {
        self.ko_states.contains(&KoState::from_move(&self.board, position, player))
    }
}

/// Possible actions in an game
pub enum AGAAction<Board>
    where Board : BoardTrait
{
    /// Sets handicap stones.
    ///
    /// allowed at 1st ply, stones is the number of stones to set
    Handicap {stones: u8},
    
    /// The given player passes
    Pass { player: Player},
    
    /// The given player plays at the given position
    Play { player: Player, at: Position},
    
    /// The given player requests the game to end
    ///
    /// The requesting player does also propose the
    /// dead stones.
    RequestEnd { player: Player, dead_stones: Vec<Position>},
    
    /// The given player rejects the request to end the game
    RejectEnd { player: Player},
    
    /// The given player accepts the request to end the game
    AcceptEnd { player: Player},
    
    /// Used to hide unused param error
    ///
    /// The param Board is used to set the associated type when
    /// implementing game::Action.
    Phantom_ { board: PhantomData<Board> }
}

/// The set of possible game phases
#[derive(PartialEq)]
enum GamePhase {
    /// Tha game is running. 
    ///
    /// The current player is allowed to play, pass and request to
    /// end the game.
    Running,
    
    /// Black has passed
    ///
    /// If white passes next, the game ends
    BlackPassed,
    
    /// The game is ending.
    ///
    /// White has passed after black passed. It is time to specify
    /// dead stones or to continue playing.
    Ending,
    
    /// Black has requested to end the game
    ///
    /// White has to accept or reject the request.
    EndRequestedBlack,
    
    /// White has requested to end the game
    ///
    /// Black has to accept or reject the request. 
    EndRequestedWhite,
    
    /// The game ended
    ///
    /// There is nothing to be done except to count the points.
    Ended
}

impl<Board> Action for AGAAction<Board>
    where Board : BoardTrait
{
    type GameState = AGAGameState<Board>;

    fn test(self : &Self, state : &Self::GameState) -> bool {
        match self {
            &AGAAction::Handicap{stones: _stones} => {
                state.ply == 0
            },
            &AGAAction::Pass{ ref player } => {
                (state.phase == GamePhase::Running
                    || state.phase == GamePhase::BlackPassed)
                && *player == state.current_player()
            },
            &AGAAction::Play { ref player, at: ref position } => {
                state.board.is_valid(position)
                && state.board.at(&position) == Stone::Empty
                && (state.phase == GamePhase::Running
                    || state.phase == GamePhase::BlackPassed)
                && *player == state.current_player()
                && !state.board.would_be_suicide(position, player)
                && !state.would_be_ko(position, player)
            },
            &AGAAction::RequestEnd { player: ref _player, ref dead_stones } => {
                state.phase == GamePhase::Ending
                && dead_stones.iter().all(|pos| state.board.is_valid(pos))
                && dead_stones.iter().map(|pos| state.board.at(pos)).all(|stone| stone != Stone::Empty)
            },
            &AGAAction::RejectEnd{ ref player } => {
                (state.phase == GamePhase::EndRequestedBlack && *player == Player::White
                    || state.phase == GamePhase::EndRequestedWhite && *player == Player::Black)
            },
            &AGAAction::AcceptEnd { ref player } => {
                (state.phase == GamePhase::EndRequestedBlack && *player == Player::White
                    || state.phase == GamePhase::EndRequestedWhite && *player == Player::Black)
            }
            _ => { false }
        }
    }

    fn execute(self : &Self, state : &mut Self::GameState) {
        match self {
            &AGAAction::Handicap{ stones } => {
                if 2 <= stones && stones <= 9 { //upper right and lower left
                    state.board.set(&Position{x: 14, y: 4}, &Stone::Black);
                    state.board.set(&Position{x: 4, y: 14}, &Stone::Black);
                }
                if 3 <= stones && stones <= 9 { //lower right
                    state.board.set(&Position{x: 14, y: 14}, &Stone::Black);
                }
                if 4 <= stones && stones <= 9 { //upper left
                    state.board.set(&Position{x: 4, y: 4}, &Stone::Black);
                }
                if stones == 5 || stones == 7 || stones == 9 { //middle
                    state.board.set(&Position{x: 10, y: 10}, &Stone::Black);
                }
                if 6 <= stones && stones <= 9 { //left side and right sizde
                    state.board.set(&Position{x: 4, y: 10}, &Stone::Black);
                    state.board.set(&Position{x: 14, y: 10}, &Stone::Black);
                }
                if stones == 8 || stones == 9 { //upper side and lower side
                    state.board.set(&Position{x: 10, y: 4}, &Stone::Black);
                    state.board.set(&Position{x: 10, y: 14}, &Stone::Black);
                }
                state.ply += 1;
                state.register_ko_state();
            },
            &AGAAction::Pass { ref player } => {
                if *player == Player::Black {
                    state.phase = GamePhase::BlackPassed;
                } else if *player == Player::White
                       && state.phase == GamePhase::BlackPassed {
                    state.phase = GamePhase::Ending;
                }
                state.ply += 1;
                state.register_ko_state();
            },
            &AGAAction::Play { ref player, at: ref position } => {
                let captured_stones = state.board.would_be_captured(player, position);
                state.board.set(position, &player.stone());
                for captured_stone in &captured_stones {
                    state.board.set(captured_stone, &Stone::Empty);
                }
                state.ply += 1;
                state.phase = GamePhase::Running;
                state.register_ko_state();
            },
            &AGAAction::RequestEnd { ref player, ref dead_stones } => {
                if *player == Player::Black {
                    state.phase = GamePhase::EndRequestedBlack;
                } else if *player == Player::White {
                    state.phase = GamePhase::EndRequestedWhite;
                }

                state.dead_stones = Option::Some(dead_stones.clone());
            },
            &AGAAction::RejectEnd{ player: ref _player } => {
                state.phase = GamePhase::Ending;
                state.dead_stones = Option::None;
            },
            &AGAAction::AcceptEnd { player: ref _player } => {
                state.phase = GamePhase::Ended;
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod test{
    use game;
    use game::Path;
    use player::Player;
    use super::{AGAAction, GamePhase};
    use position::Position;
    use stone::Stone;
    use board::{BoardTrait, Board19x19};

    type Game = game::Game<AGAAction<Board19x19>>;

    #[test]
    fn create_game(){
        let game = Game::new();
        let state = game.get_state(&Path::Empty);

        assert!(state.ply == 0);
        assert!(state.current_player() == Player::Black);
        assert!(state.dead_stones == Option::None);
    }

    #[test]
    fn play(){
        let mut game = Game::new();
        assert!(game.insert(&Path::Empty,
            AGAAction::Play {
                player: Player::Black,
                at: Position{x: 3, y: 3}
            }) != Path::Empty);
    }

    #[test]
    fn suicide(){
        let mut game = Game::new();
        let actions : Vec<AGAAction<Board19x19>> = vec!(
            AGAAction::Play{player: Player::Black, at: Position{x: 0, y: 1}},
            AGAAction::Play{player: Player::White, at: Position{x: 0, y: 2}},
            AGAAction::Play{player: Player::Black, at: Position{x: 1, y: 0}},
            AGAAction::Play{player: Player::White, at: Position{x: 1, y: 1}},
            AGAAction::Play{player: Player::Black, at: Position{x: 5, y: 5}},
            AGAAction::Play{player: Player::White, at: Position{x: 2, y: 0}}
        );

        let mut cursor = Path::Empty;
        for action in actions {
            cursor = game.insert(&cursor, action);
            assert!(cursor != Path::Empty);
        }

        assert!(game.insert(&cursor, AGAAction::Play{ player: Player::Black, at: Position{x: 0, y: 0}})
            == Path::Empty);
    }

    #[test]
    fn capture_ko() {
        let mut game = Game::new();
        let actions : Vec<AGAAction<Board19x19>> = vec!(
            AGAAction::Play{ player: Player::Black, at: Position{x: 0, y: 0}},
            AGAAction::Play{ player: Player::White, at: Position{x: 1, y: 0}},
            AGAAction::Play{ player: Player::Black, at: Position{x: 2, y: 0}},
            AGAAction::Play{ player: Player::White, at: Position{x: 0, y: 1}},
            AGAAction::Play{ player: Player::Black, at: Position{x: 1, y: 1}},
            AGAAction::Play{ player: Player::White, at: Position{x: 2, y: 1}},
            AGAAction::Play{ player: Player::Black, at: Position{x: 0, y: 0}}
        );
        // # . . .  # O . .   # O # .   . O # .  . O # .  . O # .  # . # .  recap is ko
        // . . . .  . . . .   . . . .   O . . .  O # . .  O # O .  O # O .
        // . . . .  . . . .   . . . .   . . . .  . . . .  . . . .  . . . .

        let mut cursor = Path::Empty;
        for action in actions {
            cursor = game.insert(&cursor, action);
            assert!(cursor != Path::Empty);
        }

        assert!(game.insert(&cursor, AGAAction::Play{ player: Player::White, at: Position{x: 1, y: 0}})
            == Path::Empty);
    }

    #[test]
    fn pass() {
        let mut game = Game::new();
        let mut cursor = Path::Empty;

        cursor = game.insert(&cursor, AGAAction::Pass{ player: Player::Black });
        assert!(cursor != Path::Empty);
        assert!(game.insert(&cursor, AGAAction::Pass{ player: Player::Black}) == Path::Empty);

        cursor = game.insert(&cursor, AGAAction::Pass{ player: Player::White});
        assert!(cursor != Path::Empty);

        let state = game.get_state(&cursor);
        assert!(state.ply == 2);
        assert!(state.phase == GamePhase::Ending);
    }

    #[test]
    fn handicap() {
        let mut game = Game::new();
        let mut cursor = Path::Empty;

        cursor = game.insert(&cursor, AGAAction::Handicap{ stones: 3});
        let state = game.get_state(&cursor);

        assert!(state.current_player() == Player::White);
        assert!(state.board.at(&Position{x: 14, y:  4}) == Stone::Black);
        assert!(state.board.at(&Position{x:  4, y: 14}) == Stone::Black);
        assert!(state.board.at(&Position{x: 14, y: 14}) == Stone::Black);
    }

    #[test]
    fn end() {
        let mut game = Game::new();
        let mut cursor = Path::Empty;

        cursor = game.insert(&cursor, AGAAction::Play{ player: Player::Black, at: Position{x: 2, y: 2}});
        cursor = game.insert(&cursor, AGAAction::Pass{player: Player::White});
        cursor = game.insert(&cursor, AGAAction::Pass{player: Player::Black});

        assert!(game.get_state(&cursor).phase == GamePhase::BlackPassed);
        cursor = game.insert(&cursor, AGAAction::Pass{player: Player::White});
        assert!(game.get_state(&cursor).phase == GamePhase::Ending);

        assert!(game.insert(&cursor, AGAAction::RejectEnd{player: Player::Black}) == Path::Empty);
        assert!(game.insert(&cursor, AGAAction::RejectEnd{player: Player::White}) == Path::Empty);
        assert!(game.insert(&cursor, AGAAction::AcceptEnd{player: Player::Black}) == Path::Empty);
        assert!(game.insert(&cursor, AGAAction::AcceptEnd{player: Player::White}) == Path::Empty);

        assert!(game.insert(&cursor,
                            AGAAction::RequestEnd{player: Player::Black, dead_stones: vec!(Position{x: 2, y: 3})})
            == Path::Empty);

        cursor = game.insert(&cursor, AGAAction::RequestEnd{player: Player::Black, dead_stones: vec!()});
        assert!(cursor != Path::Empty);

        assert!(game.insert(&cursor, AGAAction::AcceptEnd{player: Player::Black}) == Path::Empty);
        cursor = game.insert(&cursor, AGAAction::AcceptEnd{player: Player::White});
        assert!(cursor != Path::Empty);
    }
}