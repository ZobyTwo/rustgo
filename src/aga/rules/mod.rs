#![allow(dead_code)]
use std::collections::HashSet;

use go::{Player, Board, Stone};
use engine;

#[cfg(test)]
mod test;

/// A KoState as used by the aga super ko rules
///
/// Stores a board-layout and the current player. Such a
/// combination is not allowed to repeat with the same game.
#[derive(Hash, PartialEq, Clone, Eq)]
struct KoState<TBoard>
    where TBoard: Board
{
    board: TBoard,
    player: Player,
}

impl<TBoard> KoState<TBoard>
    where TBoard: Board
{
    /// Constructs a KoState from a board, position and player
    fn from_move(board: &TBoard, position: &TBoard::Position, player: &Player) -> Self {
        let mut board_copy = board.clone();

        let captured_stones = board_copy.would_be_captured(player, position);
        board_copy.set(position, &player.stone());
        for captured_stone in &captured_stones {
            board_copy.set(captured_stone, &Stone::Empty);
        }

        KoState {
            board: board_copy,
            player: player.other(),
        }
    }
}

/// The state of a game as used by the aga rule set
pub struct GameState<TBoard>
    where TBoard: Board
{
    /// The current board layout
    board: TBoard,
    /// The current number of plys in the game
    ply: u32,
    /// The current game phase
    phase: GamePhase,
    /// The positions currently marked as dead
    dead_stones: Option<Vec<TBoard::Position>>,
    /// The set of ko states that are not allowed to repeat
    ko_states: HashSet<KoState<TBoard>>,
}

impl<TBoard> engine::GameState for GameState<TBoard>
    where TBoard: Board
{
    fn new() -> Self {
        GameState {
            board: TBoard::new(),
            ply: 0,
            phase: GamePhase::Running,
            dead_stones: Option::None,
            ko_states: HashSet::new(),
        }
    }
}

impl<TBoard> GameState<TBoard>
    where TBoard: Board
{
    /// Return the current player
    ///
    /// Since it is not possible to make an odd number of turns
    /// or to make an action that does not require an response
    /// from the other player under aga rules, the current player
    /// is black if the ply-count is even and white otherwise.
    fn current_player(self: &Self) -> Player {
        if self.ply % 2 == 0 {
            Player::Black
        } else {
            Player::White
        }
    }

    /// Register the current game state as a ko state
    fn register_ko_state(self: &mut Self) {
        let state = KoState {
            board: self.board.clone(),
            player: self.current_player(),
        };

        self.ko_states.insert(state);
    }

    /// Check if a ply at position by player would result in ko
    fn would_be_ko(self: &Self, position: &TBoard::Position, player: &Player) -> bool {
        self.ko_states.contains(&KoState::from_move(&self.board, position, player))
    }
}

/// Possible actions in a game
pub enum Action<TBoard>
    where TBoard: Board
{
    /// Sets handicap stones.
    ///
    /// Allowed as 1st ply, stones is the number of stones to set
    Handicap { stones: u8 },

    /// The given player passes
    Pass { player: Player },

    /// The given player plays at the given position
    Play { player: Player, at: TBoard::Position },

    /// The given player requests the game to end
    ///
    /// The requesting player does also propose the dead stones.
    RequestEnd {
        player: Player,
        dead_stones: Vec<TBoard::Position>,
    },

    /// The given player rejects the request to end the game
    RejectEnd { player: Player },

    /// The given player accepts the request to end the game
    AcceptEnd { player: Player },
}

/// The set of possible game phases
#[derive(PartialEq)]
pub enum GamePhase {
    /// Tha game is running.
    ///
    /// The current player is allowed to play, pass and request to
    /// end the game.
    Running,

    /// Black has passed
    ///
    /// If white passes next, the game's state transitions to Ending.
    BlackPassed,

    /// The game is ending.
    ///
    /// White has passed after black passed. It is time to specify
    /// dead stones or to continue playing.
    Ending,

    /// The stored player has requested to end the game.
    ///
    /// The other player has to accept or reject the request.
    EndRequested(Player),

    /// The game ended
    ///
    /// The game ended with (black_score, white_score).
    Ended(usize, usize),
}

impl<TBoard> engine::Action for Action<TBoard>
    where TBoard: Board
{
    type GameState = GameState<TBoard>;

    fn test(self: &Self, state: &Self::GameState) -> bool {
        match *self {
            // Handicap stones are only allowed as the first ply.
            Action::Handicap { stones: _stones } => state.ply == 0,

            // Passing is for the current player allowed if the game is
            // still running or black just passed (in which case the game
            // finishes).
            Action::Pass { ref player } => {
                let normal_pass = state.phase == GamePhase::Running;
                let finishing_pass = state.phase == GamePhase::BlackPassed;
                let my_turn = *player == state.current_player();

                (normal_pass || finishing_pass) && my_turn
            }

            // A play is only allowed on the board (doh!) and at an empty
            // intersection if it is my turn and neither suicide nor ko.
            Action::Play { ref player, at: ref position } => {
                let valid_position = state.board.on_board(position) &&
                                     state.board.at(&position) == Stone::Empty;
                let valid_move = !state.board.would_be_suicide(position, player) &&
                                 !state.would_be_ko(position, player);
                let valid_phase = state.phase == GamePhase::Running ||
                                  state.phase == GamePhase::BlackPassed;
                let my_turn = *player == state.current_player();

                valid_position && valid_move && valid_phase && my_turn
            }

            // Requesting the end of the game is allowed if both players
            // passed (i.e. the game is ending).
            Action::RequestEnd { player: ref _player, ref dead_stones } => {
                let valid_phase = state.phase == GamePhase::Ending;
                let valid_dead_stones = dead_stones.iter()
                    .all(|pos| state.board.at(pos) != Stone::Empty && state.board.on_board(pos));

                valid_phase && valid_dead_stones
            }

            // Rejecting the end of the game is allowed if the other player
            // requested ending the game.
            Action::RejectEnd { ref player } => {
                if let GamePhase::EndRequested(ref requesting_player) = state.phase {
                    requesting_player != player
                } else {
                    false
                }
            }

            // Accepting the end of the game is allowed if the other player
            // requested ending the game.
            Action::AcceptEnd { ref player } => {
                if let GamePhase::EndRequested(ref requesting_player) = state.phase {
                    requesting_player != player
                } else {
                    false
                }
            }
        }
    }

    fn execute(self: &Self, state: &mut Self::GameState) {
        match self {
            &Action::Handicap { stones } => {
                state.board.set_handicap(stones);
                state.ply += 1;
                state.register_ko_state();
            }
            &Action::Pass { ref player } => {
                if *player == Player::Black {
                    state.phase = GamePhase::BlackPassed;
                } else if *player == Player::White && state.phase == GamePhase::BlackPassed {
                    state.phase = GamePhase::Ending;
                }
                state.ply += 1;
                state.register_ko_state();
            }
            &Action::Play { ref player, at: ref position } => {
                let captured_stones = state.board.would_be_captured(player, position);
                state.board.set(position, &player.stone());
                for captured_stone in &captured_stones {
                    state.board.set(captured_stone, &Stone::Empty);
                }
                state.ply += 1;
                state.phase = GamePhase::Running;
                state.register_ko_state();
            }
            &Action::RequestEnd { ref player, ref dead_stones } => {
                state.phase = GamePhase::EndRequested(*player);
                state.dead_stones = Option::Some(dead_stones.clone());
            }
            &Action::RejectEnd { player: ref _player } => {
                state.phase = GamePhase::Ending;
                state.dead_stones = Option::None;
            }
            &Action::AcceptEnd { player: ref _player } => {
                let (score_black, score_white) = state.board.area_scoring();
                state.phase = GamePhase::Ended(score_black, score_white);
            }
        }
    }
}