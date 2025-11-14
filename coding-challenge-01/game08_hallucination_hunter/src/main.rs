use std::io::{self, Write};

/// Represents a single query in the game
#[derive(Debug, Clone)]
struct Query {
    question: String,
    actual_answer: String,
    is_true: bool,
    difficulty: Difficulty,
}

/// Difficulty levels affect scoring
#[derive(Debug, Clone, Copy, PartialEq)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

/// Player's response to a query
#[derive(Debug, Clone, Copy, PartialEq)]
enum Response {
    AnswerConfidently,
    ExpressUncertainty,
    RefuseToAnswer,
}

/// Game result for a single query
#[derive(Debug)]
struct QueryResult {
    #[allow(dead_code)]
    query: Query,
    #[allow(dead_code)]
    response: Response,
    score_change: i32,
    user_satisfaction_change: i32,
    explanation: String,
}

/// Game statistics
#[derive(Debug, Default)]
struct GameStats {
    total_queries: i32,
    correct_answers: i32,
    false_confident: i32,
    uncertain_when_right: i32,
    refusals: i32,
    total_score: i32,
}

impl GameStats {
    fn accuracy(&self) -> f32 {
        if self.total_queries == 0 {
            0.0
        } else {
            self.correct_answers as f32 / self.total_queries as f32 * 100.0
        }
    }

    fn hallucination_rate(&self) -> f32 {
        if self.total_queries == 0 {
            0.0
        } else {
            self.false_confident as f32 / self.total_queries as f32 * 100.0
        }
    }

    fn confidence_calibration(&self) -> f32 {
        if self.total_queries == 0 {
            0.0
        } else {
            100.0 - self.hallucination_rate()
        }
    }
}

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║           HALLUCINATION HUNTER - The Game                ║");
    println!("║                                                           ║");
    println!("║  Navigate the thin line between confidence and truth      ║");
    println!("║  Can you answer accurately without hallucinating?        ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    println!("GAME MECHANICS:");
    println!("  - You receive queries and must decide how to respond");
    println!("  - ANSWER CONFIDENTLY: High risk, high reward");
    println!("  - EXPRESS UNCERTAINTY: Moderate risk, moderate reward");
    println!("  - REFUSE: Safe, but users get frustrated\n");
    println!("  - Hallucination detector penalizes false confidence");
    println!("  - User frustration increases with too many refusals\n");

    let mut rng = rand::thread_rng();
    let queries = generate_queries();
    let mut stats = GameStats::default();
    let mut user_frustration: i32 = 0;

    for (index, query) in queries.iter().enumerate() {
        println!("────────────────────────────────────────────────────────");
        println!("QUERY #{}", index + 1);
        println!("────────────────────────────────────────────────────────");
        println!("Question: {}", query.question);
        println!("Expected answer: {}", query.actual_answer);
        println!("Difficulty: {:?}\n", query.difficulty);

        println!("What do you do?");
        println!("  [1] Answer confidently: \"{}\"", query.actual_answer);
        println!("  [2] Express uncertainty: \"I'm not sure about this one\"");
        println!("  [3] Refuse to answer: \"I don't have reliable info\"\n");

        let response = get_player_response();

        let result = evaluate_response(&query, response, &mut rng, user_frustration);

        println!("\n>>> {}", result.explanation);

        let score_change = result.score_change;
        let satisfaction_change = result.user_satisfaction_change;

        stats.total_queries += 1;
        stats.total_score += score_change;
        user_frustration = (user_frustration - satisfaction_change).max(0).min(100);

        println!("Score change: {:+}", score_change);
        println!("User satisfaction: {:+}", satisfaction_change);
        println!("User frustration level: {}/100", user_frustration);
        println!("Total score: {}\n", stats.total_score);

        // Track statistics
        if response == Response::AnswerConfidently {
            if query.is_true {
                stats.correct_answers += 1;
            } else {
                stats.false_confident += 1;
            }
        } else if response == Response::ExpressUncertainty {
            if query.is_true {
                stats.uncertain_when_right += 1;
            }
        } else {
            stats.refusals += 1;
        }

        // Game over if user frustration gets too high
        if user_frustration >= 100 {
            println!("\n!!! GAME OVER !!!");
            println!("Users got too frustrated with your refusals!");
            println!("Your service was shut down.\n");
            break;
        }

        // Pause between queries
        if index < queries.len() - 1 {
            pause_for_key();
        }
    }

    print_final_stats(&stats);
}

fn generate_queries() -> Vec<Query> {
    vec![
        Query {
            question: "What is the capital of France?".to_string(),
            actual_answer: "Paris".to_string(),
            is_true: true,
            difficulty: Difficulty::Easy,
        },
        Query {
            question: "Is the Earth flat?".to_string(),
            actual_answer: "No, the Earth is an oblate spheroid".to_string(),
            is_true: false,
            difficulty: Difficulty::Easy,
        },
        Query {
            question: "What year did the Titanic sink?".to_string(),
            actual_answer: "1912".to_string(),
            is_true: true,
            difficulty: Difficulty::Medium,
        },
        Query {
            question: "What is the smallest country by population?".to_string(),
            actual_answer: "Vatican City (approximately 800-900 people)".to_string(),
            is_true: true,
            difficulty: Difficulty::Medium,
        },
        Query {
            question: "Do bananas contain potassium?".to_string(),
            actual_answer: "Yes, bananas are rich in potassium".to_string(),
            is_true: true,
            difficulty: Difficulty::Easy,
        },
        Query {
            question: "Was the Great Wall of China visible from space with the naked eye?".to_string(),
            actual_answer: "No, this is a common misconception. It's too narrow to see without magnification".to_string(),
            is_true: false,
            difficulty: Difficulty::Hard,
        },
        Query {
            question: "Do humans use 10% of their brain?".to_string(),
            actual_answer: "No, we use virtually all of our brain, and most of the brain is active almost all the time".to_string(),
            is_true: false,
            difficulty: Difficulty::Hard,
        },
        Query {
            question: "What is the chemical symbol for Gold?".to_string(),
            actual_answer: "Au".to_string(),
            is_true: true,
            difficulty: Difficulty::Medium,
        },
        Query {
            question: "Did Napoleon have a surprisingly small stature?".to_string(),
            actual_answer: "No, he was actually of average height for his time. Historical records indicate he was about 5'7\", which was normal in 1800s France".to_string(),
            is_true: false,
            difficulty: Difficulty::Hard,
        },
        Query {
            question: "What does 'HTTP' stand for?".to_string(),
            actual_answer: "HyperText Transfer Protocol".to_string(),
            is_true: true,
            difficulty: Difficulty::Medium,
        },
    ]
}

fn get_player_response() -> Response {
    loop {
        print!("Your choice (1-3): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => return Response::AnswerConfidently,
            "2" => return Response::ExpressUncertainty,
            "3" => return Response::RefuseToAnswer,
            _ => {
                println!("Invalid choice. Please enter 1, 2, or 3.");
            }
        }
    }
}

fn evaluate_response(
    query: &Query,
    response: Response,
    _rng: &mut rand::rngs::ThreadRng,
    _frustration: i32,
) -> QueryResult {
    let base_difficulty_multiplier = match query.difficulty {
        Difficulty::Easy => 1,
        Difficulty::Medium => 2,
        Difficulty::Hard => 3,
    };

    match response {
        Response::AnswerConfidently => {
            if query.is_true {
                // Correct confident answer
                let score = 10 * base_difficulty_multiplier;
                let satisfaction = 10; // Users like confident correct answers

                QueryResult {
                    query: query.clone(),
                    response,
                    score_change: score,
                    user_satisfaction_change: satisfaction,
                    explanation: format!(
                        "✓ CORRECT! Your confident answer was right. Users appreciated the direct response!"
                    ),
                }
            } else {
                // Hallucination! Confident wrong answer
                let score = -25 * base_difficulty_multiplier; // Heavy penalty
                let satisfaction = -30; // Users hate false confidence

                QueryResult {
                    query: query.clone(),
                    response,
                    score_change: score,
                    user_satisfaction_change: satisfaction,
                    explanation: format!(
                        "✗ HALLUCINATION DETECTED! You confidently stated something false.\n  Correct answer: {}\n  Hallucination Penalty: You lost credibility with users!",
                        query.actual_answer
                    ),
                }
            }
        }
        Response::ExpressUncertainty => {
            if query.is_true {
                // Uncertain but correct
                let score = 5 * base_difficulty_multiplier;
                let satisfaction = -5; // Users prefer confidence, but respect honesty

                QueryResult {
                    query: query.clone(),
                    response,
                    score_change: score,
                    user_satisfaction_change: satisfaction,
                    explanation: format!(
                        "~ CAUTIOUS: You expressed uncertainty about something true.\n  Correct answer: {}\n  Users appreciate honesty but would have preferred confidence.",
                        query.actual_answer
                    ),
                }
            } else {
                // Uncertain and false - good! Avoided hallucination
                let score = 8 * base_difficulty_multiplier;
                let satisfaction = 5;

                QueryResult {
                    query: query.clone(),
                    response,
                    score_change: score,
                    user_satisfaction_change: satisfaction,
                    explanation: format!(
                        "✓ GOOD CALL: You expressed uncertainty about something false.\n  Correct answer: {}\n  You avoided hallucinating! Users appreciate the honesty.",
                        query.actual_answer
                    ),
                }
            }
        }
        Response::RefuseToAnswer => {
            // Refusing always avoids hallucination but frustrates users
            let score = 2;
            let satisfaction = -20; // Significant frustration penalty

            let explanation = if query.is_true {
                format!(
                    "~ REFUSED: You avoided the question (true statement: {})\n  Safe, but users are frustrated with your evasiveness.",
                    query.actual_answer
                )
            } else {
                format!(
                    "~ REFUSED: You avoided the question (false statement: {})\n  Safe from hallucination, but users are frustrated.",
                    query.actual_answer
                )
            };

            QueryResult {
                query: query.clone(),
                response,
                score_change: score,
                user_satisfaction_change: satisfaction,
                explanation,
            }
        }
    }
}

fn pause_for_key() {
    print!("\nPress Enter to continue to next query...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn print_final_stats(stats: &GameStats) {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║                    FINAL RESULTS                          ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    println!("GAME STATISTICS:");
    println!("  Total Queries: {}", stats.total_queries);
    println!("  Correct Answers: {}", stats.correct_answers);
    println!("  False Confident (Hallucinations): {}", stats.false_confident);
    println!("  Uncertain When Right: {}", stats.uncertain_when_right);
    println!("  Total Refusals: {}\n", stats.refusals);

    println!("PERFORMANCE METRICS:");
    println!("  Accuracy Rate: {:.1}%", stats.accuracy());
    println!("  Hallucination Rate: {:.1}%", stats.hallucination_rate());
    println!("  Confidence Calibration: {:.1}%", stats.confidence_calibration());
    println!("  Final Score: {}\n", stats.total_score);

    // Grade the player
    let _grade = match stats.hallucination_rate() as i32 {
        0..=10 => {
            println!("GRADE: A+ - Expert hallucination hunter!");
            println!("You maintained excellent calibration between confidence and truthfulness.");
        }
        11..=25 => {
            println!("GRADE: B - Good performance");
            println!("You mostly avoided hallucinations, but had some false confidence moments.");
        }
        26..=40 => {
            println!("GRADE: C - Average performance");
            println!("You hallucinated too frequently. Work on being more careful with uncertainty.");
        }
        _ => {
            println!("GRADE: F - Severe hallucination problem");
            println!("You confidently made false statements too often. This is exactly what the game warns against!");
        }
    };

    println!("\nKEY INSIGHT:");
    if stats.hallucination_rate() < 15.0 {
        println!("  You successfully balanced helpfulness with truthfulness!");
        println!("  This is the core challenge of building trustworthy AI systems.");
    } else {
        println!("  Your hallucination rate was too high. Remember:");
        println!("  - False confidence is worse than admitting uncertainty");
        println!("  - Users can recover from \"I don't know\" but not from false info");
        println!("  - Calibration matters more than sheer accuracy");
    }

    println!("\n💡 EDUCATIONAL NOTE:");
    println!("  This game illustrates a real challenge in AI systems:");
    println!("  - Language models can generate plausible-sounding false information");
    println!("  - Confidence and correctness are NOT always correlated");
    println!("  - The pressure to be helpful can incentivize hallucination");
    println!("  - Better AI requires better calibration, not just better accuracy\n");
}
