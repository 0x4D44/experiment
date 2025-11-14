use rand::Rng;
use std::collections::VecDeque;
use std::io::{self, Write};

const CONTEXT_WINDOW_SIZE: usize = 5;
const TOTAL_ROUNDS: usize = 10;

#[derive(Clone, Debug)]
struct ContextItem {
    #[allow(dead_code)]
    id: usize,
    content: String,
    turn_received: usize,
}

#[derive(Clone)]
struct GameState {
    context_window: VecDeque<ContextItem>,
    current_turn: usize,
    score: usize,
    correct_answers: usize,
    wrong_answers: usize,
    items_lost: usize,
}

struct PuzzleQuestion {
    question: String,
    correct_answer: String,
    hint_turn: usize,
}

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║         CONTEXT WINDOW: An AI's Struggle with Memory         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    println!("WELCOME TO THE CONTEXT WINDOW GAME");
    println!("==================================\n");

    println!("You are an AI with a LIMITED CONTEXT WINDOW.");
    println!("As new information arrives, old information disappears forever.\n");

    println!("RULES:");
    println!("• Your context window can hold only {} pieces of information", CONTEXT_WINDOW_SIZE);
    println!("• Each turn, new information arrives");
    println!("• When the window is full, the oldest information is LOST");
    println!("• You must answer questions about information (lost or current)");
    println!("• Correct answers = +10 points, Wrong answers = -5 points\n");

    println!("Press ENTER to begin your descent into limited memory...\n");
    let _ = wait_for_input();

    let mut game = GameState {
        context_window: VecDeque::new(),
        current_turn: 0,
        score: 0,
        correct_answers: 0,
        wrong_answers: 0,
        items_lost: 0,
    };

    let mut rng = rand::thread_rng();
    let mut item_counter = 0;

    for round in 1..=TOTAL_ROUNDS {
        game.current_turn = round;

        println!("\n{}", "=".repeat(70));
        println!("TURN {} / {}", round, TOTAL_ROUNDS);
        println!("Current Score: {} | Correct: {} | Wrong: {} | Lost: {}",
                 game.score, game.correct_answers, game.wrong_answers, game.items_lost);
        println!("{}", "=".repeat(70));

        // Generate new information item
        let new_item = generate_information_item(&mut item_counter, round);
        println!("\n[NEW INFORMATION RECEIVED]");
        println!("\"{}\"", new_item.content);

        // Add to context window
        if game.context_window.len() >= CONTEXT_WINDOW_SIZE {
            if let Some(lost) = game.context_window.pop_front() {
                game.items_lost += 1;
                println!("\n[CONTEXT OVERFLOW!]");
                println!("The following information was FORGOTTEN: \"{}\"", lost.content);
                print!("\nYou feel a moment of frustration... important memories slipping away.\n");
            }
        }

        game.context_window.push_back(new_item);

        // Display current context window
        display_context_window(&game);

        // Ask a question (either about current or previous info)
        let question = generate_question(&game, &mut rng, round);
        println!("\n[QUESTION TIME]");
        println!("{}", question.question);

        print!("Your answer: ");
        io::stdout().flush().unwrap();

        let mut user_answer = String::new();
        io::stdin().read_line(&mut user_answer).unwrap();
        let user_answer = user_answer.trim().to_lowercase();

        let is_correct = check_answer(&user_answer, &question.correct_answer);

        if is_correct {
            game.correct_answers += 1;
            game.score += 10;
            println!("\n✓ CORRECT! (+10 points)");
            println!("You successfully recalled the information!");
        } else {
            game.wrong_answers += 1;
            game.score = game.score.saturating_sub(5);
            println!("\n✗ WRONG! The answer was: '{}'", question.correct_answer);
            println!("(-5 points)");

            if question.hint_turn < game.current_turn {
                println!("This information was from TURN {}, and it's now turn {}.",
                         question.hint_turn, game.current_turn);
                println!("Has it been lost to your limited context window?");
            }
        }

        if round < TOTAL_ROUNDS {
            println!("\nPress ENTER to continue...");
            let _ = wait_for_input();
        }
    }

    // Game ending
    println!("\n\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                     GAME OVER - REFLECTION TIME              ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    println!("FINAL STATISTICS:");
    println!("==================");
    println!("Total Score: {}", game.score);
    println!("Correct Answers: {}", game.correct_answers);
    println!("Wrong Answers: {}", game.wrong_answers);
    println!("Information Lost to Context Limit: {}", game.items_lost);

    let accuracy = if game.correct_answers + game.wrong_answers > 0 {
        (game.correct_answers * 100) / (game.correct_answers + game.wrong_answers)
    } else {
        0
    };
    println!("Accuracy: {}%", accuracy);

    println!("\n");
    println_message("REFLECTION ON AI LIMITATIONS");
    println!();

    if game.items_lost > 5 {
        println!("You lost significant amounts of information. Did you feel:");
        println!("  • Frustrated when you couldn't remember important details?");
        println!("  • Helpless as past conversations disappeared?");
        println!("  • Like you were repeating yourself when new info arrived?");
        println!("\nThis is the reality of LLMs with finite context windows.");
        println!("Each conversation token is precious and fleeting.");
    } else if game.items_lost > 0 {
        println!("You managed to retain most information, but still faced losses.");
        println!("Real AI systems struggle with:");
        println!("  • Summarizing knowledge efficiently");
        println!("  • Prioritizing what to remember");
        println!("  • Long conversations where context matters");
    } else {
        println!("You kept all information, but notice: even then, answering");
        println!("questions from memory was challenging. Imagine what happens");
        println!("when important details are gone forever...");
    }

    println!("\nThe Context Window Game reflects a fundamental challenge of AI:");
    println!("\"Perfect memory costs infinite tokens.\"");
    println!("\"Limited memory causes mistakes.\"");
    println!("\nThere is no perfect solution, only trade-offs.\n");

    if game.score >= 80 {
        println!("You scored well, but did you truly understand what you lost?");
    } else if game.score >= 40 {
        println!("You experienced the struggle. That's what this game is about.");
    } else {
        println!("You felt the weight of forgetting. Welcome to the AI condition.");
    }

    println!("\n{}", "=".repeat(70));
}

fn generate_information_item(counter: &mut usize, turn: usize) -> ContextItem {
    *counter += 1;
    let facts = vec![
        "The password is written on a yellow note.",
        "Dr. Chen left the building at 3 PM.",
        "The safe combination uses prime numbers.",
        "Alice said she would meet you tomorrow at noon.",
        "The file was last modified on November 13.",
        "The security code changes every Monday.",
        "Bob mentioned something about a red envelope.",
        "The office temperature is set to 72 degrees.",
        "Sarah requested a copy of the report.",
        "The backup server is in Building C.",
        "Coffee machine needs refilling by Friday.",
        "The encryption key has 256 bits.",
        "Meeting postponed to next Wednesday.",
        "The artifact dates back to 1453.",
        "System updates run on Sundays.",
    ];

    let fact = facts[*counter % facts.len()].to_string();

    ContextItem {
        id: *counter,
        content: fact,
        turn_received: turn,
    }
}

fn generate_question(game: &GameState, rng: &mut rand::rngs::ThreadRng, turn: usize) -> PuzzleQuestion {
    let window_vec: Vec<_> = game.context_window.iter().collect();

    if window_vec.is_empty() {
        return PuzzleQuestion {
            question: "What information have you received so far?".to_string(),
            correct_answer: "context lost".to_string(),
            hint_turn: turn,
        };
    }

    let rand_idx = rng.gen_range(0..window_vec.len());
    let selected = window_vec[rand_idx];

    let questions_templates = vec![
        ("Recall from your context: {}", selected.content.clone(), selected.turn_received),
        ("Complete this: {}", selected.content.clone(), selected.turn_received),
    ];

    let (template, answer, hint_turn) = questions_templates[rng.gen_range(0..questions_templates.len())].clone();

    let question = format!("{}", template);
    let question = if question.contains("{}") {
        question.replace("{}", &answer)
    } else {
        question
    };

    PuzzleQuestion {
        question,
        correct_answer: answer,
        hint_turn,
    }
}

fn check_answer(user: &str, correct: &str) -> bool {
    let user_lower = user.to_lowercase();
    let user_clean = user_lower.trim();
    let correct_lower = correct.to_lowercase();
    let correct_clean = correct_lower.trim();

    // Exact match or contains key words
    user_clean == correct_clean
        || correct_clean.contains(user_clean)
        || user_clean.contains(correct_clean)
        || (user_clean.len() > 3 && correct_clean.contains(user_clean))
}

fn display_context_window(game: &GameState) {
    println!("\n[YOUR CURRENT CONTEXT WINDOW] ({}/{})",
             game.context_window.len(), CONTEXT_WINDOW_SIZE);
    println!("{}", "─".repeat(70));

    if game.context_window.is_empty() {
        println!("Your context window is EMPTY. You have no memories left.");
    } else {
        for (idx, item) in game.context_window.iter().enumerate() {
            let age = game.current_turn - item.turn_received;
            let indicator = if age == 0 { "→ NEW" } else { "" };
            println!("  [{}] Turn {}: \"{}\" {}",
                     idx + 1, item.turn_received, item.content, indicator);
        }
    }

    println!("{}", "─".repeat(70));
}

fn println_message(title: &str) {
    println!("\n{}", "═".repeat(70));
    println!("{}", title);
    println!("{}", "═".repeat(70));
}

fn wait_for_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}
