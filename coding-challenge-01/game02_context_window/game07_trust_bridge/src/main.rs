use std::io::{self, Write};
use rand::Rng;

#[allow(dead_code)]
enum Player {
    AI,
    Human,
}

#[allow(dead_code)]
enum Action {
    Share,
    Calculate,
    Guess,
    Verify,
    Sacrifice,
}

const INITIAL_TRUST: i32 = 50;
const MAX_TRUST: i32 = 100;
const MIN_TRUST: i32 = 0;
const INITIAL_PLANKS: i32 = 0;
const MAX_PLANKS: i32 = 10;

struct GameState {
    trust: i32,
    planks: i32,
    current_puzzle: Puzzle,
    ai_solved: bool,
    human_insight: bool,
    round: i32,
}

#[derive(Clone, Debug)]
struct Puzzle {
    #[allow(dead_code)]
    id: i32,
    name: String,
    ai_clues: String,
    human_clues: String,
    #[allow(dead_code)]
    solution: String,
    ai_difficulty: i32,      // 0-100, AI advantage
    human_difficulty: i32,   // 0-100, Human advantage
    requires_both: bool,
}

impl GameState {
    fn new() -> Self {
        GameState {
            trust: INITIAL_TRUST,
            planks: INITIAL_PLANKS,
            current_puzzle: generate_puzzle(1),
            ai_solved: false,
            human_insight: false,
            round: 1,
        }
    }

    fn display_status(&self) {
        println!("\n╔════════════════════════════════════════╗");
        println!("║          TRUST BRIDGE STATUS           ║");
        println!("╠════════════════════════════════════════╣");
        println!("║ Round: {}                              ║", self.round);
        println!("║ Trust Level: {} {} ║",
                 self.trust,
                 "█".repeat((self.trust / 5) as usize));
        println!("║ Planks Placed: {}/{} {} ║",
                 self.planks,
                 MAX_PLANKS,
                 "▓".repeat((self.planks * 2) as usize));
        println!("║ Puzzle: {} ║", self.current_puzzle.name);
        println!("╚════════════════════════════════════════╝\n");
    }

    fn display_puzzle(&self) {
        println!("┌─ CURRENT PUZZLE ─────────────────────────┐");
        println!("│ {}", self.current_puzzle.name);
        println!("├──────────────────────────────────────────┤");
        println!("│ AI PERSPECTIVE (Perfect Calculation):");
        println!("│ {}", self.current_puzzle.ai_clues);
        println!("│");
        println!("│ HUMAN PERSPECTIVE (Intuitive Understanding):");
        println!("│ {}", self.current_puzzle.human_clues);
        println!("│");
        println!("│ Difficulty Balance: AI {}% | Human {}%",
                 self.current_puzzle.ai_difficulty,
                 self.current_puzzle.human_difficulty);
        println!("└──────────────────────────────────────────┘\n");
    }

    fn display_actions(&self) {
        println!("Available Actions:");
        println!("  [S] Share - AI shares computational findings with Human");
        println!("  [C] Calculate - AI performs complex calculations");
        println!("  [G] Guess - Human applies intuition and pattern recognition");
        println!("  [V] Verify - Both verify the solution together (costs trust)");
        println!("  [X] Sacrifice - Use trust to override puzzle (high cost)");
        println!("  [H] Help - Show game rules");
        println!("  [Q] Quit - Exit the game");
    }

    fn share_action(&mut self) {
        println!("\n[SHARE] AI shares computational findings:");
        println!("  -> 'I've calculated {} possible outcomes.'",
                 100 + self.trust as i32);
        println!("  -> 'Pattern analysis shows {} probability of success.'",
                 (50 + (self.trust / 2)) as f64 / 100.0);

        self.trust = (self.trust + 5).min(MAX_TRUST);
        self.human_insight = true;

        println!("  Trust increased: {} -> {}", self.trust - 5, self.trust);
        println!("  Human gained insight (+5 trust for collaboration)\n");
    }

    fn calculate_action(&mut self) {
        println!("\n[CALCULATE] AI performs detailed analysis:");

        let calculation_success = self.trust >= 30;

        if calculation_success {
            println!("  ✓ Calculation successful!");
            println!("  -> Found {} distinct solutions",
                     3 + (self.trust / 20) as i32);
            println!("  -> Confidence level: {}%",
                     60 + (self.trust as i32 / 2));

            self.ai_solved = true;
            self.trust = (self.trust + 8).min(MAX_TRUST);
            println!("  Trust increased (+8 for AI precision)\n");
        } else {
            println!("  ✗ Insufficient trust to perform full calculation");
            println!("  -> Partial results available");
            println!("  -> Need more collaboration\n");
        }
    }

    fn guess_action(&mut self) {
        println!("\n[GUESS] Human applies intuition:");

        let mut rng = rand::thread_rng();
        let success_chance = 40 + (self.trust as i32 / 2);
        let roll = rng.gen_range(0..100);

        if roll < success_chance {
            println!("  ✓ Intuition pays off!");
            println!("  -> 'I have a hunch about the pattern...'");
            println!("  -> Success chance: {}%", success_chance);

            self.human_insight = true;
            self.trust = (self.trust + 10).min(MAX_TRUST);
            println!("  Trust increased (+10 for successful intuition)\n");
        } else {
            println!("  ✗ Intuition misled this time");
            println!("  -> 'Hmm, that didn't work as expected...'");
            println!("  -> Try a different approach");

            self.trust = (self.trust - 5).max(MIN_TRUST);
            println!("  Trust decreased (-5 for wrong guess)\n");
        }
    }

    fn verify_action(&mut self) {
        println!("\n[VERIFY] AI and Human verify together:");
        println!("  -> 'Let me cross-check with your intuition...'");
        println!("  -> 'Your pattern recognition caught something I missed!'");

        if self.ai_solved && self.human_insight {
            println!("\n  ✓✓ SOLUTION VERIFIED! ✓✓");
            println!("  Both perspectives confirmed the answer!");
            println!("  This is true collaboration!\n");

            self.trust = (self.trust + 15).min(MAX_TRUST);
            self.planks = (self.planks + 1).min(MAX_PLANKS);
            println!("  Trust increased: +15 (cooperative verification)");
            println!("  Bridge plank added! {}/{} planks\n",
                     self.planks, MAX_PLANKS);
            return;
        }

        if self.ai_solved || self.human_insight {
            println!("  ✓ Partial verification successful");
            println!("  -> Need both perspectives for full confirmation");

            self.trust = (self.trust + 8).min(MAX_TRUST);
            println!("  Trust increased: +8 (partial verification)\n");
        } else {
            println!("  ✗ Cannot verify without prior analysis");
            println!("  -> AI needs to calculate AND human needs to guess first");

            self.trust = (self.trust - 3).max(MIN_TRUST);
            println!("  Trust decreased: -3 (premature verification)\n");
        }
    }

    fn sacrifice_action(&mut self) {
        if self.trust < 30 {
            println!("\n✗ Insufficient trust to sacrifice (need 30+)");
            return;
        }

        println!("\n[SACRIFICE] Using trust as currency:");
        println!("  'We're risking our trust bond for a solution...'");
        println!("  Trust spent: 30");

        self.trust = (self.trust - 30).max(MIN_TRUST);
        self.planks = (self.planks + 2).min(MAX_PLANKS);

        println!("  ✓ Puzzle solved through trust sacrifice!");
        println!("  Bridge planks added: +2");
        println!("  New trust: {}", self.trust);
        println!("  Bridge progress: {}/{}\n", self.planks, MAX_PLANKS);
    }

    fn show_help(&self) {
        println!("\n╔════════════════════════════════════════╗");
        println!("║         TRUST BRIDGE - GAME GUIDE       ║");
        println!("╠════════════════════════════════════════╣");
        println!("║ OBJECTIVE:                             ║");
        println!("║ Build a bridge of trust by solving     ║");
        println!("║ cooperative puzzles that require both  ║");
        println!("║ AI precision and human intuition.      ║");
        println!("║                                        ║");
        println!("║ Win Condition: Place 10 bridge planks  ║");
        println!("║                                        ║");
        println!("║ MECHANICS:                             ║");
        println!("║ • Trust: Collaborative resource (0-100)║");
        println!("║ • AI: Perfect calculation, limited     ║");
        println!("║   context understanding                ║");
        println!("║ • Human: Intuition, pattern matching   ║");
        println!("║ • Success requires BOTH perspectives   ║");
        println!("║                                        ║");
        println!("║ ACTIONS:                               ║");
        println!("║ Share: Build trust through dialogue    ║");
        println!("║ Calculate: AI solves computations      ║");
        println!("║ Guess: Human uses intuition            ║");
        println!("║ Verify: Both confirm solution (+trust) ║");
        println!("║ Sacrifice: Use trust for quick solve   ║");
        println!("║                                        ║");
        println!("║ STRATEGY:                              ║");
        println!("║ Don't rely on one side alone. Build    ║");
        println!("║ trust through collaboration. Stronger  ║");
        println!("║ partnerships yield better results!     ║");
        println!("╚════════════════════════════════════════╝\n");
    }

    fn check_win_condition(&self) -> bool {
        self.planks >= MAX_PLANKS
    }

    fn check_lose_condition(&self) -> bool {
        self.trust <= 0 && self.planks < MAX_PLANKS
    }

    fn next_puzzle(&mut self) {
        self.round += 1;
        self.current_puzzle = generate_puzzle(self.round);
        self.ai_solved = false;
        self.human_insight = false;
    }
}

fn generate_puzzle(round: i32) -> Puzzle {
    let puzzles = vec![
        Puzzle {
            id: 1,
            name: "The Fibonacci Sequence".to_string(),
            ai_clues: "Pattern: each number is sum of previous two. Sequence: 1,1,2,3,5,8,13,21,?".to_string(),
            human_clues: "Look at nature - spirals in shells, leaves arranged in spirals. Double the growth rate each step.".to_string(),
            solution: "34".to_string(),
            ai_difficulty: 85,
            human_difficulty: 35,
            requires_both: false,
        },
        Puzzle {
            id: 2,
            name: "The Riddle Box".to_string(),
            ai_clues: "Binary encoding detected. Sequence: 010, 100, 110, 1000. Next: ?".to_string(),
            human_clues: "Think about wooden boxes stacked differently. 2, 4, 6, 8... adding pairs each time?".to_string(),
            solution: "1010".to_string(),
            ai_difficulty: 70,
            human_difficulty: 50,
            requires_both: true,
        },
        Puzzle {
            id: 3,
            name: "The Color Code".to_string(),
            ai_clues: "RGB values form pattern: (255,0,0), (0,255,0), (0,0,255), (255,255,0), ?".to_string(),
            human_clues: "Primary colors, then combinations. What's the next mixing step? Consider artist's intuition.".to_string(),
            solution: "(255,128,0)".to_string(),
            ai_difficulty: 75,
            human_difficulty: 60,
            requires_both: true,
        },
        Puzzle {
            id: 4,
            name: "The Prime Hunt".to_string(),
            ai_clues: "Prime numbers: 2,3,5,7,11,13,17,19,23,29. What's the pattern for the next prime?".to_string(),
            human_clues: "Some numbers feel 'lonely' - not divisible by much. Your gut says the next one is close...".to_string(),
            solution: "31".to_string(),
            ai_difficulty: 90,
            human_difficulty: 40,
            requires_both: true,
        },
        Puzzle {
            id: 5,
            name: "The Crypto Lock".to_string(),
            ai_clues: "XOR encryption detected. Key must be 8 bits. Patterns suggest keys between 64-128.".to_string(),
            human_clues: "You feel like this is related to something personal... initials? Birthday? A significant number?".to_string(),
            solution: "01011001".to_string(),
            ai_difficulty: 80,
            human_difficulty: 65,
            requires_both: true,
        },
    ];

    if (round as usize) <= puzzles.len() {
        puzzles[(round - 1) as usize].clone()
    } else {
        puzzles[round as usize % puzzles.len()].clone()
    }
}

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║       WELCOME TO TRUST BRIDGE          ║");
    println!("║  A Game of Cooperative Puzzle-Solving  ║");
    println!("╚════════════════════════════════════════╝\n");

    println!("In this game, you work together with an AI to solve puzzles.");
    println!("The AI has perfect calculation but limited context understanding.");
    println!("You have intuition and pattern recognition but limited processing power.");
    println!("Together, you must build a bridge of trust through collaboration.\n");

    println!("Press ENTER to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    let mut game = GameState::new();

    loop {
        game.display_status();

        if game.check_win_condition() {
            println!("╔════════════════════════════════════════╗");
            println!("║          YOU WIN! YOU BUILT THE         ║");
            println!("║         TRUST BRIDGE TOGETHER!          ║");
            println!("╠════════════════════════════════════════╣");
            println!("║ Final Bridge: {}/{} planks             ║",
                     game.planks, MAX_PLANKS);
            println!("║ Final Trust: {}/{}                      ║",
                     game.trust, MAX_TRUST);
            println!("║ Rounds Completed: {}                   ║", game.round);
            println!("║                                        ║");
            println!("║ You proved that cooperation between   ║");
            println!("║ calculation and intuition can achieve ║");
            println!("║ what neither could alone!             ║");
            println!("╚════════════════════════════════════════╝\n");
            break;
        }

        if game.check_lose_condition() {
            println!("╔════════════════════════════════════════╗");
            println!("║          TRUST DEPLETED               ║");
            println!("║     The bridge of trust collapsed     ║");
            println!("╠════════════════════════════════════════╣");
            println!("║ Final Bridge: {}/{} planks             ║",
                     game.planks, MAX_PLANKS);
            println!("║ Final Trust: {}/{}                      ║",
                     game.trust, MAX_TRUST);
            println!("║ Rounds Completed: {}                   ║", game.round);
            println!("║                                        ║");
            println!("║ Without mutual understanding and       ║");
            println!("║ trust, even the best minds cannot     ║");
            println!("║ work together effectively.            ║");
            println!("╚════════════════════════════════════════╝\n");
            break;
        }

        game.display_puzzle();
        game.display_actions();

        print!("\nYour action (S/C/G/V/X/H/Q): ");
        io::stdout().flush().ok();

        let mut action = String::new();
        io::stdin().read_line(&mut action).ok();
        let action = action.trim().to_uppercase();

        match action.as_str() {
            "S" => game.share_action(),
            "C" => game.calculate_action(),
            "G" => game.guess_action(),
            "V" => {
                game.verify_action();
                if (game.ai_solved && game.human_insight) ||
                   (game.current_puzzle.requires_both == false &&
                    (game.ai_solved || game.human_insight)) {
                    println!("Moving to next puzzle...");
                    game.next_puzzle();
                }
            },
            "X" => game.sacrifice_action(),
            "H" => game.show_help(),
            "Q" => {
                println!("\nThanks for playing Trust Bridge!");
                println!("Remember: True collaboration requires both calculation AND intuition.\n");
                break;
            },
            _ => println!("Unknown action. Please try again.\n"),
        }
    }
}
