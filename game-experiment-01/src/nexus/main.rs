pub mod hex;
pub mod game;
pub mod ai;
pub mod ui;

use game::Game;
use ui::GameUI;

fn main() {
    let mut game = Game::new(11);
    let ui = GameUI::new();

    ui.display_welcome();
    ui.display_board(&game);

    loop {
        if game.is_game_over() {
            ui.display_winner(game.get_winner());
            break;
        }

        if game.current_player_is_human() {
            match ui.get_player_move(&game) {
                Some(pos) => {
                    if !game.make_move(pos) {
                        ui.display_error("Invalid move!");
                        continue;
                    }
                }
                None => {
                    ui.display_goodbye();
                    break;
                }
            }
        } else {
            ui.display_thinking();
            let ai_move = ai::get_best_move(&game);
            game.make_move(ai_move);
            ui.display_ai_move(ai_move);
        }

        ui.display_board(&game);
    }
}
