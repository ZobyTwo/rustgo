use engine::{Game, Path};
use go::{Player, Stone, Board};
use aga::{Action, GamePhase, Position19x19, Board19x19};

type AGAGame = Game<Action<Board19x19>>;

#[test]
fn create_game() {
    let game = AGAGame::new();
    let state = game.get_state(&Path::Empty);

    assert!(state.ply == 0);
    assert!(state.current_player() == Player::Black);
    assert!(state.dead_stones == Option::None);
}

#[test]
fn play() {
    let mut game = AGAGame::new();
    assert!(game.insert(&Path::Empty,
                        Action::Play {
                            player: Player::Black,
                            at: Position19x19 { x: 3, y: 3 },
                        }) != Path::Empty);
}

#[test]
fn suicide() {
    let mut game = AGAGame::new();
    let actions: Vec<Action<Board19x19>> = vec![Action::Play {
                                                        player: Player::Black,
                                                        at: Position19x19 { x: 0, y: 1 },
                                                    },
                                                    Action::Play {
                                                        player: Player::White,
                                                        at: Position19x19 { x: 0, y: 2 },
                                                    },
                                                    Action::Play {
                                                        player: Player::Black,
                                                        at: Position19x19 { x: 1, y: 0 },
                                                    },
                                                    Action::Play {
                                                        player: Player::White,
                                                        at: Position19x19 { x: 1, y: 1 },
                                                    },
                                                    Action::Play {
                                                        player: Player::Black,
                                                        at: Position19x19 { x: 5, y: 5 },
                                                    },
                                                    Action::Play {
                                                        player: Player::White,
                                                        at: Position19x19 { x: 2, y: 0 },
                                                    }];

    let mut cursor = Path::Empty;
    for action in actions {
        cursor = game.insert(&cursor, action);
        assert!(cursor != Path::Empty);
    }

    assert!(game.insert(&cursor,
                        Action::Play {
                            player: Player::Black,
                            at: Position19x19 { x: 0, y: 0 },
                        }) == Path::Empty);
}

#[test]
fn capture_ko() {
    let mut game = AGAGame::new();
    let actions: Vec<Action<Board19x19>> = vec![Action::Play {
                                                        player: Player::Black,
                                                        at: Position19x19 { x: 0, y: 0 },
                                                    },
                                                    Action::Play {
                                                        player: Player::White,
                                                        at: Position19x19 { x: 1, y: 0 },
                                                    },
                                                    Action::Play {
                                                        player: Player::Black,
                                                        at: Position19x19 { x: 2, y: 0 },
                                                    },
                                                    Action::Play {
                                                        player: Player::White,
                                                        at: Position19x19 { x: 0, y: 1 },
                                                    },
                                                    Action::Play {
                                                        player: Player::Black,
                                                        at: Position19x19 { x: 1, y: 1 },
                                                    },
                                                    Action::Play {
                                                        player: Player::White,
                                                        at: Position19x19 { x: 2, y: 1 },
                                                    },
                                                    Action::Play {
                                                        player: Player::Black,
                                                        at: Position19x19 { x: 0, y: 0 },
                                                    }];
    // # . . .  # O . .   # O # .   . O # .  . O # .  . O # .  # . # .  recap is ko
    // . . . .  . . . .   . . . .   O . . .  O # . .  O # O .  O # O .
    // . . . .  . . . .   . . . .   . . . .  . . . .  . . . .  . . . .

    let mut cursor = Path::Empty;
    for action in actions {
        cursor = game.insert(&cursor, action);
        assert!(cursor != Path::Empty);
    }

    assert!(game.insert(&cursor,
                        Action::Play {
                            player: Player::White,
                            at: Position19x19 { x: 1, y: 0 },
                        }) == Path::Empty);
}

#[test]
fn pass() {
    let mut game = AGAGame::new();
    let mut cursor = Path::Empty;

    cursor = game.insert(&cursor, Action::Pass { player: Player::Black });
    assert!(cursor != Path::Empty);
    assert!(game.insert(&cursor, Action::Pass { player: Player::Black }) == Path::Empty);

    cursor = game.insert(&cursor, Action::Pass { player: Player::White });
    assert!(cursor != Path::Empty);

    let state = game.get_state(&cursor);
    assert!(state.ply == 2);
    assert!(state.phase == GamePhase::Ending);
}

#[test]
fn handicap() {
    let mut game = AGAGame::new();
    let mut cursor = Path::Empty;

    cursor = game.insert(&cursor, Action::Handicap { stones: 3 });
    let state = game.get_state(&cursor);

    assert!(state.current_player() == Player::White);
    assert!(state.board.at(&Position19x19 { x: 14, y: 4 }) == Stone::Black);
    assert!(state.board.at(&Position19x19 { x: 4, y: 14 }) == Stone::Black);
    assert!(state.board.at(&Position19x19 { x: 14, y: 14 }) == Stone::Black);
}

#[test]
fn end() {
    let mut game = AGAGame::new();
    let mut cursor = Path::Empty;

    cursor = game.insert(&cursor,
                            Action::Play {
                                player: Player::Black,
                                at: Position19x19 { x: 2, y: 2 },
                            });
    cursor = game.insert(&cursor, Action::Pass { player: Player::White });
    cursor = game.insert(&cursor, Action::Pass { player: Player::Black });

    assert!(game.get_state(&cursor).phase == GamePhase::BlackPassed);
    cursor = game.insert(&cursor, Action::Pass { player: Player::White });
    assert!(game.get_state(&cursor).phase == GamePhase::Ending);

    assert!(game.insert(&cursor, Action::RejectEnd { player: Player::Black }) ==
            Path::Empty);
    assert!(game.insert(&cursor, Action::RejectEnd { player: Player::White }) ==
            Path::Empty);
    assert!(game.insert(&cursor, Action::AcceptEnd { player: Player::Black }) ==
            Path::Empty);
    assert!(game.insert(&cursor, Action::AcceptEnd { player: Player::White }) ==
            Path::Empty);

    assert!(game.insert(&cursor,
                        Action::RequestEnd {
                            player: Player::Black,
                            dead_stones: vec![Position19x19 { x: 2, y: 3 }],
                        }) == Path::Empty);

    cursor = game.insert(&cursor,
                            Action::RequestEnd {
                                player: Player::Black,
                                dead_stones: vec![],
                            });
    assert!(cursor != Path::Empty);

    assert!(game.insert(&cursor, Action::AcceptEnd { player: Player::Black }) ==
            Path::Empty);
    cursor = game.insert(&cursor, Action::AcceptEnd { player: Player::White });
    assert!(cursor != Path::Empty);
}