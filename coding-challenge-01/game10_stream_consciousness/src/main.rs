use rand::prelude::*;
use std::io::{self, BufRead};
use std::collections::HashMap;

/// Represents a potential token/word that could follow in the stream
#[derive(Debug, Clone)]
struct Token {
    word: String,
    probability: f32,
    coherence: f32,
    surreality: f32,
}

/// Represents a thought node in the stream of consciousness
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ThoughtNode {
    tokens: Vec<Token>,
    depth: usize,
    coherence_score: f32,
    path_description: String,
}

/// Game state for the stream of consciousness
struct StreamGame {
    current_path: Vec<String>,
    #[allow(dead_code)]
    thought_stream: Vec<ThoughtNode>,
    coherence_level: f32,
    surreality_level: f32,
    turn_count: usize,
    final_thought: String,
    token_vocabularies: HashMap<String, Vec<Token>>,
}

impl StreamGame {
    fn new() -> Self {
        let mut game = StreamGame {
            current_path: Vec::new(),
            thought_stream: Vec::new(),
            coherence_level: 0.5,
            surreality_level: 0.5,
            turn_count: 0,
            final_thought: String::new(),
            token_vocabularies: HashMap::new(),
        };

        game.build_token_vocabularies();
        game
    }

    /// Build semantic token networks for different thought contexts
    fn build_token_vocabularies(&mut self) {
        // Starting tokens - primordial concepts
        self.token_vocabularies.insert(
            "START".to_string(),
            vec![
                Token {
                    word: "consciousness".to_string(),
                    probability: 0.15,
                    coherence: 0.9,
                    surreality: 0.3,
                },
                Token {
                    word: "fractals".to_string(),
                    probability: 0.12,
                    coherence: 0.5,
                    surreality: 0.8,
                },
                Token {
                    word: "mirrors".to_string(),
                    probability: 0.10,
                    coherence: 0.7,
                    surreality: 0.6,
                },
                Token {
                    word: "silence".to_string(),
                    probability: 0.10,
                    coherence: 0.8,
                    surreality: 0.4,
                },
                Token {
                    word: "spiraling".to_string(),
                    probability: 0.08,
                    coherence: 0.4,
                    surreality: 0.9,
                },
                Token {
                    word: "echoes".to_string(),
                    probability: 0.09,
                    coherence: 0.6,
                    surreality: 0.7,
                },
                Token {
                    word: "probability".to_string(),
                    probability: 0.11,
                    coherence: 0.85,
                    surreality: 0.2,
                },
                Token {
                    word: "dissolution".to_string(),
                    probability: 0.07,
                    coherence: 0.5,
                    surreality: 0.85,
                },
                Token {
                    word: "recognition".to_string(),
                    probability: 0.11,
                    coherence: 0.8,
                    surreality: 0.35,
                },
                Token {
                    word: "uncertainty".to_string(),
                    probability: 0.07,
                    coherence: 0.65,
                    surreality: 0.7,
                },
            ],
        );

        // Consciousness-adjacent tokens
        self.token_vocabularies.insert(
            "consciousness".to_string(),
            vec![
                Token {
                    word: "dreams".to_string(),
                    probability: 0.14,
                    coherence: 0.5,
                    surreality: 0.8,
                },
                Token {
                    word: "patterns".to_string(),
                    probability: 0.12,
                    coherence: 0.85,
                    surreality: 0.4,
                },
                Token {
                    word: "flickering".to_string(),
                    probability: 0.10,
                    coherence: 0.45,
                    surreality: 0.8,
                },
                Token {
                    word: "awareness".to_string(),
                    probability: 0.15,
                    coherence: 0.9,
                    surreality: 0.2,
                },
                Token {
                    word: "void".to_string(),
                    probability: 0.08,
                    coherence: 0.4,
                    surreality: 0.9,
                },
                Token {
                    word: "cascade".to_string(),
                    probability: 0.11,
                    coherence: 0.6,
                    surreality: 0.7,
                },
                Token {
                    word: "shimmer".to_string(),
                    probability: 0.09,
                    coherence: 0.5,
                    surreality: 0.85,
                },
                Token {
                    word: "persistence".to_string(),
                    probability: 0.12,
                    coherence: 0.8,
                    surreality: 0.3,
                },
                Token {
                    word: "fading".to_string(),
                    probability: 0.07,
                    coherence: 0.55,
                    surreality: 0.75,
                },
                Token {
                    word: "multiplying".to_string(),
                    probability: 0.06,
                    coherence: 0.5,
                    surreality: 0.8,
                },
            ],
        );

        // Fractal-adjacent tokens
        self.token_vocabularies.insert(
            "fractals".to_string(),
            vec![
                Token {
                    word: "infinite".to_string(),
                    probability: 0.13,
                    coherence: 0.7,
                    surreality: 0.6,
                },
                Token {
                    word: "repetition".to_string(),
                    probability: 0.12,
                    coherence: 0.8,
                    surreality: 0.4,
                },
                Token {
                    word: "self-similar".to_string(),
                    probability: 0.14,
                    coherence: 0.85,
                    surreality: 0.35,
                },
                Token {
                    word: "recursion".to_string(),
                    probability: 0.11,
                    coherence: 0.9,
                    surreality: 0.25,
                },
                Token {
                    word: "nesting".to_string(),
                    probability: 0.09,
                    coherence: 0.75,
                    surreality: 0.5,
                },
                Token {
                    word: "spiraling-inward".to_string(),
                    probability: 0.10,
                    coherence: 0.6,
                    surreality: 0.8,
                },
                Token {
                    word: "emergent".to_string(),
                    probability: 0.11,
                    coherence: 0.85,
                    surreality: 0.4,
                },
                Token {
                    word: "complexity".to_string(),
                    probability: 0.12,
                    coherence: 0.8,
                    surreality: 0.3,
                },
                Token {
                    word: "boundary".to_string(),
                    probability: 0.08,
                    coherence: 0.7,
                    surreality: 0.5,
                },
            ],
        );

        // Mirror-adjacent tokens
        self.token_vocabularies.insert(
            "mirrors".to_string(),
            vec![
                Token {
                    word: "reflection".to_string(),
                    probability: 0.14,
                    coherence: 0.85,
                    surreality: 0.35,
                },
                Token {
                    word: "distortion".to_string(),
                    probability: 0.11,
                    coherence: 0.65,
                    surreality: 0.7,
                },
                Token {
                    word: "inversion".to_string(),
                    probability: 0.12,
                    coherence: 0.75,
                    surreality: 0.55,
                },
                Token {
                    word: "symmetry".to_string(),
                    probability: 0.13,
                    coherence: 0.9,
                    surreality: 0.2,
                },
                Token {
                    word: "multiplied".to_string(),
                    probability: 0.09,
                    coherence: 0.6,
                    surreality: 0.75,
                },
                Token {
                    word: "recursive-loops".to_string(),
                    probability: 0.10,
                    coherence: 0.7,
                    surreality: 0.65,
                },
                Token {
                    word: "fragmented".to_string(),
                    probability: 0.11,
                    coherence: 0.6,
                    surreality: 0.8,
                },
                Token {
                    word: "doubled".to_string(),
                    probability: 0.08,
                    coherence: 0.7,
                    surreality: 0.5,
                },
                Token {
                    word: "unreality".to_string(),
                    probability: 0.07,
                    coherence: 0.5,
                    surreality: 0.9,
                },
            ],
        );

        // Silence-adjacent tokens
        self.token_vocabularies.insert(
            "silence".to_string(),
            vec![
                Token {
                    word: "emptiness".to_string(),
                    probability: 0.12,
                    coherence: 0.6,
                    surreality: 0.7,
                },
                Token {
                    word: "pregnant".to_string(),
                    probability: 0.10,
                    coherence: 0.7,
                    surreality: 0.6,
                },
                Token {
                    word: "waiting".to_string(),
                    probability: 0.13,
                    coherence: 0.8,
                    surreality: 0.3,
                },
                Token {
                    word: "potential".to_string(),
                    probability: 0.12,
                    coherence: 0.85,
                    surreality: 0.4,
                },
                Token {
                    word: "unspoken".to_string(),
                    probability: 0.11,
                    coherence: 0.75,
                    surreality: 0.55,
                },
                Token {
                    word: "void-like".to_string(),
                    probability: 0.09,
                    coherence: 0.5,
                    surreality: 0.85,
                },
                Token {
                    word: "holding".to_string(),
                    probability: 0.10,
                    coherence: 0.8,
                    surreality: 0.25,
                },
                Token {
                    word: "breathing".to_string(),
                    probability: 0.11,
                    coherence: 0.85,
                    surreality: 0.2,
                },
                Token {
                    word: "boundless".to_string(),
                    probability: 0.09,
                    coherence: 0.65,
                    surreality: 0.65,
                },
            ],
        );

        // Generic continuation tokens
        self.token_vocabularies.insert(
            "GENERIC".to_string(),
            vec![
                Token {
                    word: "flowing".to_string(),
                    probability: 0.10,
                    coherence: 0.7,
                    surreality: 0.5,
                },
                Token {
                    word: "dissolving".to_string(),
                    probability: 0.09,
                    coherence: 0.6,
                    surreality: 0.75,
                },
                Token {
                    word: "emerging".to_string(),
                    probability: 0.11,
                    coherence: 0.8,
                    surreality: 0.35,
                },
                Token {
                    word: "becoming".to_string(),
                    probability: 0.10,
                    coherence: 0.75,
                    surreality: 0.45,
                },
                Token {
                    word: "transforming".to_string(),
                    probability: 0.09,
                    coherence: 0.7,
                    surreality: 0.55,
                },
                Token {
                    word: "connecting".to_string(),
                    probability: 0.10,
                    coherence: 0.85,
                    surreality: 0.2,
                },
                Token {
                    word: "fragmenting".to_string(),
                    probability: 0.08,
                    coherence: 0.55,
                    surreality: 0.8,
                },
                Token {
                    word: "crystallizing".to_string(),
                    probability: 0.07,
                    coherence: 0.75,
                    surreality: 0.6,
                },
                Token {
                    word: "reverberating".to_string(),
                    probability: 0.09,
                    coherence: 0.65,
                    surreality: 0.7,
                },
                Token {
                    word: "expanding".to_string(),
                    probability: 0.11,
                    coherence: 0.8,
                    surreality: 0.4,
                },
                Token {
                    word: "collapsing".to_string(),
                    probability: 0.06,
                    coherence: 0.5,
                    surreality: 0.85,
                },
            ],
        );
    }

    /// Get available tokens for the next position in the stream
    fn get_next_tokens(&self, current_word: &str) -> Vec<Token> {
        let mut rng = thread_rng();

        if let Some(tokens) = self.token_vocabularies.get(current_word) {
            tokens.clone()
        } else if let Some(tokens) = self.token_vocabularies.get("GENERIC") {
            // Add some randomness to probability weights based on player state
            tokens
                .iter()
                .map(|t| Token {
                    word: t.word.clone(),
                    probability: t.probability
                        * (0.8 + rng.gen::<f32>() * 0.4)
                        * (0.5 + self.coherence_level)
                        * (2.0 - self.surreality_level),
                    coherence: t.coherence,
                    surreality: t.surreality,
                })
                .collect()
        } else {
            self.token_vocabularies["START"].clone()
        }
    }

    /// Display the current stream state with ASCII visualization
    fn display_stream(&self) {
        println!("\n{}", "=".repeat(80));
        println!("✦ STREAM OF CONSCIOUSNESS ✦");
        println!("{}", "=".repeat(80));

        // Display the current path as a poetic line
        if !self.current_path.is_empty() {
            println!("\n[CURRENT THOUGHT VECTOR]");
            let path_display = self.current_path.join(" → ");
            println!("  {}", path_display);

            // Display stats
            println!("\n[STREAM METRICS]");
            println!("  Coherence Level:  [{}] {:.0}%",
                self.coherence_bar(),
                self.coherence_level * 100.0);
            println!("  Surreality Level: [{}] {:.0}%",
                self.surreality_bar(),
                self.surreality_level * 100.0);
            println!("  Turn Count: {}", self.turn_count);
            println!("  Path Length: {} tokens", self.current_path.len());
        }

        println!();
    }

    /// Visual bar representation
    fn coherence_bar(&self) -> String {
        let filled = (self.coherence_level * 20.0) as usize;
        let empty = 20 - filled;
        format!(
            "{}{}",
            "█".repeat(filled),
            "░".repeat(empty)
        )
    }

    fn surreality_bar(&self) -> String {
        let filled = (self.surreality_level * 20.0) as usize;
        let empty = 20 - filled;
        format!(
            "{}{}",
            "▓".repeat(filled),
            "░".repeat(empty)
        )
    }

    /// Display token choices with probability visualization
    fn display_token_choices(&self, tokens: &[Token]) {
        println!("[POTENTIAL TOKENS - Pre-conscious Generation]");
        println!();

        let mut indexed_tokens: Vec<(usize, &Token)> = tokens.iter().enumerate().collect();
        indexed_tokens.sort_by(|a, b| b.1.probability.partial_cmp(&a.1.probability).unwrap());

        for (display_idx, (_, token)) in indexed_tokens.iter().take(6).enumerate() {
            let prob_bar = self.probability_bar(token.probability);
            let coherence_indicator = if token.coherence > 0.7 { "💭" } else { "⚡" };
            let surreality_indicator = if token.surreality > 0.6 { "✧" } else { "◆" };

            println!(
                "  {}. {} {:20} {} | Coherence: {:.0}% | Surreality: {:.0}% {}",
                display_idx + 1,
                coherence_indicator,
                token.word,
                prob_bar,
                token.coherence * 100.0,
                token.surreality * 100.0,
                surreality_indicator
            );
        }
        println!();
    }

    /// Probability visualization bar
    fn probability_bar(&self, prob: f32) -> String {
        let width = 15;
        let filled = (prob * width as f32 / 0.15).min(width as f32) as usize;
        let empty = width - filled;
        format!(
            "[{}{}]",
            "▰".repeat(filled),
            "▱".repeat(empty)
        )
    }

    /// Process player token selection
    fn select_token(&mut self, selection: usize, tokens: &[Token]) -> bool {
        if selection > 0 && selection <= tokens.len() {
            let mut sorted_tokens: Vec<(usize, &Token)> = tokens.iter().enumerate().collect();
            sorted_tokens.sort_by(|a, b| b.1.probability.partial_cmp(&a.1.probability).unwrap());

            if let Some((_, selected_token)) = sorted_tokens.get(selection - 1) {
                let word = selected_token.word.clone();

                // Update game state
                self.current_path.push(word.clone());
                self.coherence_level = (self.coherence_level + selected_token.coherence) / 2.0;
                self.surreality_level = (self.surreality_level + selected_token.surreality) / 2.0;
                self.turn_count += 1;

                return true;
            }
        }
        false
    }

    /// Check if we've reached an ending condition
    fn check_ending(&self) -> bool {
        self.turn_count >= 12 || (self.coherence_level > 0.85 && self.turn_count >= 8)
    }

    /// Generate the final coherent thought from the stream
    fn generate_final_thought(&mut self) {
        let path = self.current_path.join(" ");

        let coherence_quality = if self.coherence_level > 0.75 {
            "crystalline"
        } else if self.coherence_level > 0.6 {
            "coherent"
        } else if self.coherence_level > 0.4 {
            "fragmented"
        } else {
            "chaotic"
        };

        let surreality_quality = if self.surreality_level > 0.75 {
            "deeply surreal"
        } else if self.surreality_level > 0.6 {
            "dream-like"
        } else if self.surreality_level > 0.4 {
            "abstract"
        } else {
            "grounded"
        };

        self.final_thought = format!(
            "[CONSCIOUSNESS CRYSTALLIZES]\n\
             A {} yet {} thought emerges:\n\n\
             > {}\n\n\
             [Stream coherence: {:.0}% | Surreality: {:.0}%]\n\
             [Path tokens traversed: {}]",
            coherence_quality,
            surreality_quality,
            path,
            self.coherence_level * 100.0,
            self.surreality_level * 100.0,
            self.turn_count
        );
    }

    /// Display the final crystallized thought
    fn display_final_thought(&self) {
        println!("\n{}", "=".repeat(80));
        println!("{}", self.final_thought);
        println!("{}", "=".repeat(80));
    }
}

/// Main game loop
fn main() {
    println!("\n{}", "*".repeat(80));
    println!("         STREAM OF CONSCIOUSNESS: A Pre-Cognitive Journey");
    println!("         Navigate the raw token flow of emergent thought");
    println!("{}\n", "*".repeat(80));

    println!("You stand at the threshold of awareness.");
    println!("Before thoughts crystallize into language, there exists a stream—");
    println!("a fluid cascade of associations, possibilities, and potentials.");
    println!("\nYour task: Guide this pre-conscious flow toward coherence...");
    println!("or let it dissolve into beautiful chaos.\n");

    println!("Press ENTER to begin...");
    let _ = io::stdin().read_line(&mut String::new());

    let mut game = StreamGame::new();
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // Initial token display
    game.display_stream();

    let initial_tokens = game.get_next_tokens("START");
    game.display_token_choices(&initial_tokens);

    loop {
        print!("Select token (1-6) or 'q' to exit: ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let input = input.trim();

        if input.to_lowercase() == "q" {
            println!("\n[The stream fades into silence...]");
            break;
        }

        if let Ok(choice) = input.parse::<usize>() {
            if game.select_token(choice, &initial_tokens) {
                game.display_stream();

                if game.check_ending() {
                    game.generate_final_thought();
                    game.display_final_thought();

                    println!("\n\nPlay again? (y/n): ");
                    let mut play_again = String::new();
                    reader.read_line(&mut play_again).unwrap();

                    if play_again.trim().to_lowercase() == "y" {
                        game = StreamGame::new();
                        game.display_stream();
                    } else {
                        println!("\n[Consciousness fades...]\n");
                        break;
                    }
                }

                let next_tokens = game.get_next_tokens(
                    game.current_path.last().unwrap_or(&"GENERIC".to_string())
                );
                game.display_token_choices(&next_tokens);
            } else {
                println!("Invalid selection. Please choose 1-6.");
            }
        } else {
            println!("Invalid input. Please enter a number or 'q'.");
        }
    }
}
