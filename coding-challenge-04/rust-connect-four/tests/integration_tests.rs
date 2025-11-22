use rust_connect_four::board::{Board, GameState, Player};
use rust_connect_four::ai::{AI, Difficulty};

#[test]
fn test_complete_game_horizontal_win() {
    let mut board = Board::new();
    let mut current_player = Player::Red;

    // Simulate a game where Red wins horizontally
    let moves = vec![
        (0, Player::Red),
        (0, Player::Yellow),
        (1, Player::Red),
        (1, Player::Yellow),
        (2, Player::Red),
        (2, Player::Yellow),
        (3, Player::Red), // Red wins
    ];

    for (col, player) in moves {
        assert_eq!(current_player, player);
        board.drop_piece(col, player);
        current_player = current_player.other();
    }

    assert_eq!(board.check_winner(), Some(Player::Red));
    assert_eq!(board.check_game_state(), GameState::Won(Player::Red));
}

#[test]
fn test_complete_game_vertical_win() {
    let mut board = Board::new();

    // Yellow wins vertically
    for i in 0..4 {
        board.drop_piece(3, Player::Yellow);
        if i < 3 {
            board.drop_piece(4, Player::Red); // Red plays elsewhere
        }
    }

    assert_eq!(board.check_winner(), Some(Player::Yellow));
}

#[test]
fn test_complex_diagonal_scenario() {
    let mut board = Board::new();

    // Create a complex board state
    let moves = vec![
        (3, Player::Red),
        (4, Player::Yellow),
        (4, Player::Red),
        (5, Player::Yellow),
        (5, Player::Red),
        (6, Player::Yellow),
        (5, Player::Red),
        (6, Player::Yellow),
        (6, Player::Red),
        (6, Player::Red), // Red wins diagonally
    ];

    for (col, player) in moves {
        board.drop_piece(col, player);
    }

    assert_eq!(board.check_winner(), Some(Player::Red));
}

#[test]
fn test_ai_vs_ai_game_completes() {
    let mut board = Board::new();
    let ai_red = AI::new(Player::Red, Difficulty::Easy);
    let ai_yellow = AI::new(Player::Yellow, Difficulty::Easy);

    let mut current_player = Player::Red;
    let mut moves = 0;
    let max_moves = Board::ROWS * Board::COLS;

    while board.check_game_state() == GameState::InProgress && moves < max_moves {
        let ai = if current_player == Player::Red {
            &ai_red
        } else {
            &ai_yellow
        };

        if let Some(col) = ai.get_best_move(&board) {
            board.drop_piece(col, current_player);
            current_player = current_player.other();
            moves += 1;
        } else {
            break;
        }
    }

    // Game should end in a win or draw
    let state = board.check_game_state();
    assert!(state != GameState::InProgress || moves == max_moves);
}

#[test]
fn test_move_history_accuracy() {
    let mut board = Board::new();

    board.drop_piece(0, Player::Red);
    board.drop_piece(1, Player::Yellow);
    board.drop_piece(2, Player::Red);

    assert_eq!(board.move_history, vec![0, 1, 2]);
    assert_eq!(board.move_count, 3);
}

#[test]
fn test_multiple_undos() {
    let mut board = Board::new();

    board.drop_piece(0, Player::Red);
    board.drop_piece(1, Player::Yellow);
    board.drop_piece(2, Player::Red);
    board.drop_piece(3, Player::Yellow);

    assert_eq!(board.move_count, 4);

    board.undo_move();
    board.undo_move();

    assert_eq!(board.move_count, 2);
    assert_eq!(board.move_history, vec![0, 1]);
}

#[test]
fn test_board_full_detection() {
    let mut board = Board::new();

    // Fill entire board
    for _ in 0..Board::ROWS {
        for col in 0..Board::COLS {
            board.drop_piece(col, Player::Red);
        }
    }

    assert_eq!(board.valid_moves(), vec![]);
    assert_eq!(board.move_count, Board::ROWS * Board::COLS);
}

#[test]
fn test_ai_blocks_immediate_threat() {
    let mut board = Board::new();
    let ai = AI::new(Player::Yellow, Difficulty::Medium);

    // Red creates a threat with 3 in a row
    board.drop_piece(0, Player::Red);
    board.drop_piece(1, Player::Red);
    board.drop_piece(2, Player::Red);

    // AI should block
    let move_col = ai.get_best_move(&board).unwrap();
    assert_eq!(move_col, 3);
}

#[test]
fn test_ai_takes_winning_opportunity() {
    let mut board = Board::new();
    let ai = AI::new(Player::Yellow, Difficulty::Medium);

    // Yellow has 3 in a row
    board.drop_piece(0, Player::Yellow);
    board.drop_piece(1, Player::Yellow);
    board.drop_piece(2, Player::Yellow);

    // AI should complete the win
    let move_col = ai.get_best_move(&board).unwrap();
    assert_eq!(move_col, 3);
}

#[test]
fn test_all_win_conditions() {
    // Horizontal win
    let mut board1 = Board::new();
    for col in 0..4 {
        board1.drop_piece(col, Player::Red);
    }
    assert_eq!(board1.check_winner(), Some(Player::Red));

    // Vertical win
    let mut board2 = Board::new();
    for _ in 0..4 {
        board2.drop_piece(0, Player::Yellow);
    }
    assert_eq!(board2.check_winner(), Some(Player::Yellow));

    // Diagonal win (down-right)
    let mut board3 = Board::new();
    board3.drop_piece(0, Player::Red);
    board3.drop_piece(1, Player::Yellow);
    board3.drop_piece(1, Player::Red);
    board3.drop_piece(2, Player::Yellow);
    board3.drop_piece(2, Player::Yellow);
    board3.drop_piece(2, Player::Red);
    board3.drop_piece(3, Player::Yellow);
    board3.drop_piece(3, Player::Yellow);
    board3.drop_piece(3, Player::Yellow);
    board3.drop_piece(3, Player::Red);
    assert_eq!(board3.check_winner(), Some(Player::Red));

    // Diagonal win (down-left)
    let mut board4 = Board::new();
    board4.drop_piece(6, Player::Yellow);
    board4.drop_piece(5, Player::Red);
    board4.drop_piece(5, Player::Yellow);
    board4.drop_piece(4, Player::Red);
    board4.drop_piece(4, Player::Red);
    board4.drop_piece(4, Player::Yellow);
    board4.drop_piece(3, Player::Red);
    board4.drop_piece(3, Player::Red);
    board4.drop_piece(3, Player::Red);
    board4.drop_piece(3, Player::Yellow);
    assert_eq!(board4.check_winner(), Some(Player::Yellow));
}

#[test]
fn test_difficulty_levels() {
    let board = Board::new();

    let ai_easy = AI::new(Player::Yellow, Difficulty::Easy);
    let ai_medium = AI::new(Player::Yellow, Difficulty::Medium);
    let ai_hard = AI::new(Player::Yellow, Difficulty::Hard);
    let ai_expert = AI::new(Player::Yellow, Difficulty::Expert);

    // All should return valid moves
    assert!(ai_easy.get_best_move(&board).is_some());
    assert!(ai_medium.get_best_move(&board).is_some());
    assert!(ai_hard.get_best_move(&board).is_some());
    assert!(ai_expert.get_best_move(&board).is_some());
}

#[test]
fn test_edge_case_full_column() {
    let mut board = Board::new();

    // Fill column 0
    for _ in 0..Board::ROWS {
        assert!(board.drop_piece(0, Player::Red).is_some());
    }

    // Next attempt should fail
    assert_eq!(board.drop_piece(0, Player::Yellow), None);
    assert!(!board.is_valid_move(0));
}

#[test]
fn test_edge_case_undo_empty_board() {
    let mut board = Board::new();
    assert_eq!(board.undo_move(), None);
    assert_eq!(board.move_count, 0);
}

#[test]
fn test_player_alternation() {
    let player = Player::Red;
    assert_eq!(player.other(), Player::Yellow);
    assert_eq!(player.other().other(), Player::Red);
}
