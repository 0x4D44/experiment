use crate::game::{Game, Player};
use crate::hex::HexCoord;
use rand::Rng;

const MAX_DEPTH: i32 = 3;
const INFINITY: i32 = 1_000_000;

pub fn get_best_move(game: &Game) -> HexCoord {
    let depth = if game.move_count() < 10 {
        2
    } else {
        MAX_DEPTH
    };

    let mut best_move = None;
    let mut best_score = -INFINITY;
    let mut alpha = -INFINITY;
    let beta = INFINITY;

    let moves = game.empty_positions();
    if moves.is_empty() {
        panic!("No moves available!");
    }

    // For the first few moves, add some randomization to make games more interesting
    if game.move_count() < 3 {
        let mut rng = rand::thread_rng();
        let center = game.size() / 2;
        let center_region: Vec<_> = moves
            .iter()
            .filter(|&pos| {
                (pos.q - center).abs() <= 2 && (pos.r - center).abs() <= 2
            })
            .copied()
            .collect();

        if !center_region.is_empty() {
            return center_region[rng.gen_range(0..center_region.len())];
        }
    }

    for mov in moves {
        let mut game_copy = game.clone();
        game_copy.make_move(mov);

        let score = -negamax(&game_copy, depth - 1, -beta, -alpha);

        if score > best_score {
            best_score = score;
            best_move = Some(mov);
        }

        alpha = alpha.max(score);
        if alpha >= beta {
            break;
        }
    }

    best_move.expect("Should have found a move")
}

fn negamax(game: &Game, depth: i32, mut alpha: i32, beta: i32) -> i32 {
    if game.is_game_over() {
        return if game.get_winner() == Some(game.current_player().other()) {
            INFINITY - (MAX_DEPTH - depth)
        } else {
            -INFINITY + (MAX_DEPTH - depth)
        };
    }

    if depth == 0 {
        return evaluate(game);
    }

    let moves = game.empty_positions();
    if moves.is_empty() {
        return evaluate(game);
    }

    let mut max_eval = -INFINITY;
    for mov in moves {
        let mut game_copy = game.clone();
        game_copy.make_move(mov);
        let eval = -negamax(&game_copy, depth - 1, -beta, -alpha);
        max_eval = max_eval.max(eval);
        alpha = alpha.max(eval);
        if alpha >= beta {
            break;
        }
    }
    max_eval
}

fn evaluate(game: &Game) -> i32 {
    let current = game.current_player();
    let opponent = current.other();

    let my_score = position_score(game, current);
    let opp_score = position_score(game, opponent);

    my_score - opp_score
}

fn position_score(game: &Game, player: Player) -> i32 {
    let mut score = 0;

    // Count pieces
    let piece_count = count_pieces(game, player);
    score += piece_count * 10;

    // Evaluate connectivity
    score += connectivity_score(game, player);

    // Evaluate distance to goal
    score += distance_to_goal_score(game, player);

    // Center control bonus
    score += center_control_score(game, player);

    score
}

fn count_pieces(game: &Game, player: Player) -> i32 {
    let mut count = 0;
    for q in 0..game.size() {
        for r in 0..game.size() {
            let pos = HexCoord::new(q, r);
            if game.get(pos) == Some(player) {
                count += 1;
            }
        }
    }
    count
}

fn connectivity_score(game: &Game, player: Player) -> i32 {
    let mut visited = std::collections::HashSet::new();
    let mut max_group_size = 0;

    for q in 0..game.size() {
        for r in 0..game.size() {
            let pos = HexCoord::new(q, r);
            if game.get(pos) == Some(player) && !visited.contains(&pos) {
                let group_size = bfs_group_size(game, pos, player, &mut visited);
                max_group_size = max_group_size.max(group_size);
            }
        }
    }

    max_group_size * 50
}

fn bfs_group_size(
    game: &Game,
    start: HexCoord,
    player: Player,
    visited: &mut std::collections::HashSet<HexCoord>,
) -> i32 {
    use std::collections::VecDeque;

    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited.insert(start);

    let mut size = 0;

    while let Some(current) = queue.pop_front() {
        size += 1;

        for neighbor in current.neighbors() {
            if !visited.contains(&neighbor)
                && neighbor.is_valid(game.size())
                && game.get(neighbor) == Some(player)
            {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    size
}

fn distance_to_goal_score(game: &Game, player: Player) -> i32 {
    use std::collections::{HashSet, VecDeque};

    let (start_edge, end_edge) = match player {
        Player::Red => (
            (0..game.size()).map(|q| HexCoord::new(q, 0)).collect::<Vec<_>>(),
            (0..game.size()).map(|q| HexCoord::new(q, game.size() - 1)).collect::<Vec<_>>(),
        ),
        Player::Blue => (
            (0..game.size()).map(|r| HexCoord::new(0, r)).collect::<Vec<_>>(),
            (0..game.size()).map(|r| HexCoord::new(game.size() - 1, r)).collect::<Vec<_>>(),
        ),
    };

    let mut min_distance = i32::MAX;

    for start_pos in &start_edge {
        if game.get(*start_pos) != Some(player) {
            continue;
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((*start_pos, 0));
        visited.insert(*start_pos);

        while let Some((current, dist)) = queue.pop_front() {
            if end_edge.contains(&current) {
                min_distance = min_distance.min(dist);
                break;
            }

            for neighbor in current.neighbors() {
                if !visited.contains(&neighbor)
                    && neighbor.is_valid(game.size())
                    && game.get(neighbor) == Some(player)
                {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }
    }

    if min_distance == i32::MAX {
        0
    } else {
        1000 / (min_distance + 1)
    }
}

fn center_control_score(game: &Game, player: Player) -> i32 {
    let center = game.size() / 2;
    let mut score = 0;

    for q in 0..game.size() {
        for r in 0..game.size() {
            let pos = HexCoord::new(q, r);
            if game.get(pos) == Some(player) {
                let dist_from_center = (q - center).abs() + (r - center).abs();
                score += (game.size() - dist_from_center) * 2;
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_best_move_returns_valid_move() {
        let game = Game::new(5);
        let mov = get_best_move(&game);
        assert!(game.is_valid_move(mov));
    }

    #[test]
    fn test_ai_takes_winning_move() {
        let mut game = Game::new(5);

        // Set up board where Blue (AI) can win in one move
        game.make_move(HexCoord::new(0, 0)); // Red
        for q in 0..4 {
            game.make_move(HexCoord::new(q, 2)); // Blue
            game.make_move(HexCoord::new(q, 3)); // Red
        }

        // Blue's turn - should complete the path
        let ai_move = get_best_move(&game);
        game.make_move(ai_move);

        assert!(game.is_game_over());
        assert_eq!(game.get_winner(), Some(Player::Blue));
    }

    #[test]
    fn test_evaluation_is_consistent() {
        let game = Game::new(5);

        // Evaluation should return a finite value
        let eval = evaluate(&game);
        assert!(eval.abs() < INFINITY);

        // Same position should evaluate the same
        let eval2 = evaluate(&game);
        assert_eq!(eval, eval2);
    }
}
