use std::collections::HashMap;
use std::io::{self, Write};
use rand::Rng;

// The Chinese Room Game: A puzzle based on Searle's thought experiment
// Players must demonstrate understanding beyond mere symbol manipulation

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║          THE CHINESE ROOM ESCAPE                           ║");
    println!("║  A Puzzle About Understanding vs. Symbol Manipulation      ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let mut game = ChineseRoomGame::new();
    game.play();
}

#[allow(dead_code)]
struct ChineseRoomGame {
    room: Room,
    turn: usize,
    rule_book: RuleBook,
    escape_progress: EscapeProgress,
    discovered_tricks: Vec<String>,
    inventory: Vec<String>,
}

#[allow(dead_code)]
struct Room {
    locked: bool,
    rule_following_only: bool,
    messages_received: Vec<String>,
    messages_sent: Vec<String>,
}

#[allow(dead_code)]
struct RuleBook {
    rules: HashMap<String, Vec<String>>,
    meta_rules: HashMap<String, String>,
}

struct EscapeProgress {
    rule_follower_score: i32,
    creative_score: i32,
    understanding_score: i32,
    max_score: i32,
}

impl ChineseRoomGame {
    fn new() -> Self {
        ChineseRoomGame {
            room: Room::new(),
            turn: 0,
            rule_book: RuleBook::new(),
            escape_progress: EscapeProgress {
                rule_follower_score: 0,
                creative_score: 0,
                understanding_score: 0,
                max_score: 15,
            },
            discovered_tricks: Vec::new(),
            inventory: vec!["pencil".to_string(), "paper".to_string()],
        }
    }

    fn play(&mut self) {
        self.introduction();

        loop {
            self.turn += 1;
            println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("TURN {}", self.turn);
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            // Display current status
            self.display_status();

            // Receive input through the slot
            let received = self.receive_message();
            println!("\n📬 You receive through the slot: {}", received);

            // Player chooses how to respond
            let response = self.get_player_response();

            // Process the response
            self.process_response(&received, &response);

            // Check for escape conditions
            if self.check_escape_condition(&received, &response) {
                self.ending();
                break;
            }

            // Limit turns to prevent infinite games
            if self.turn > 20 {
                self.timeout_ending();
                break;
            }
        }
    }

    fn introduction(&self) {
        println!("\n🚪 SCENE: You are locked in a room with only the following:");
        println!("   • A rule book (completely arbitrary rules)");
        println!("   • A slot in the wall for messages");
        println!("   • Your wits and some basic supplies\n");

        println!("📖 THE PREMISE:");
        println!("   Someone outside sends you Chinese characters.");
        println!("   You must respond using the rule book.\n");
        println!("   TWIST: The outside judge is testing if you truly UNDERSTAND");
        println!("   the symbols or if you're just mechanically following rules.\n");

        println!("🎯 YOUR GOAL:");
        println!("   Escape by proving you understand meaning beyond symbol rules.\n");

        println!("💡 MECHANICS:");
        println!("   • Follow the rules correctly (mindless approach)");
        println!("   • Find creative interpretations");
        println!("   • Discover rule loopholes");
        println!("   • Combine rules in unexpected ways");
        println!("   • Eventually, break free from the system entirely\n");

        println!("Shall we begin? (Press Enter...)");
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
    }

    fn display_status(&self) {
        println!("\n📊 CURRENT STATUS:");
        println!("   Rule-Following Score:    {} / {}",
                 self.escape_progress.rule_follower_score,
                 self.escape_progress.max_score / 3);
        println!("   Creative Score:          {} / {}",
                 self.escape_progress.creative_score,
                 self.escape_progress.max_score / 3);
        println!("   Understanding Score:     {} / {}",
                 self.escape_progress.understanding_score,
                 self.escape_progress.max_score / 3);

        if !self.discovered_tricks.is_empty() {
            println!("\n🔑 DISCOVERED TRICKS:");
            for (i, trick) in self.discovered_tricks.iter().enumerate() {
                println!("   {}. {}", i + 1, trick);
            }
        }

        println!("\n🎒 Inventory: {}", self.inventory.join(", "));
    }

    fn receive_message(&self) -> String {
        let messages = vec![
            "你好".to_string(),      // Hello
            "谢谢".to_string(),      // Thank you
            "再见".to_string(),      // Goodbye
            "?".to_string(),         // Question mark
            "我是谁？".to_string(),  // Who am I?
            "这是什么？".to_string(), // What is this?
            "为什么？".to_string(),  // Why?
            "你是谁？".to_string(),  // Who are you?
            "❤️".to_string(),        // Heart symbol
            "🔑".to_string(),        // Key symbol
        ];

        let mut rng = rand::thread_rng();
        messages[rng.gen_range(0..messages.len())].clone()
    }

    fn get_player_response(&self) -> String {
        loop {
            println!("\n🤔 What do you do?");
            println!("   (1) Follow Rule 1: Respond with opposite character");
            println!("   (2) Follow Rule 2: Respond with next character in sequence");
            println!("   (3) Follow Rule 3: Count and respond with number");
            println!("   (4) Combine multiple rules");
            println!("   (5) Try creative interpretation");
            println!("   (6) Use your inventory");
            println!("   (7) Question the rules");
            println!("   (8) Attempt escape\n");

            print!("➜ Enter choice (1-8): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let choice = input.trim();

            match choice {
                "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" => return choice.to_string(),
                _ => println!("Invalid choice. Try again."),
            }
        }
    }

    fn process_response(&mut self, received: &str, response: &str) {
        match response {
            "1" => self.rule_1_response(received),
            "2" => self.rule_2_response(received),
            "3" => self.rule_3_response(received),
            "4" => self.combination_response(received),
            "5" => self.creative_response(received),
            "6" => self.inventory_response(),
            "7" => self.question_rules(),
            "8" => self.attempt_escape(),
            _ => unreachable!(),
        }
    }

    fn rule_1_response(&mut self, received: &str) {
        println!("\n🤖 You follow Rule 1: 'Reverse character'");
        let reversed = reverse_string(received);
        println!("📤 You send through the slot: {}", reversed);

        self.escape_progress.rule_follower_score += 1;
        self.room.messages_sent.push(reversed.clone());

        println!("\n✓ Judge's feedback: \"Good rule-following. But is this understanding?\"");
    }

    fn rule_2_response(&mut self, received: &str) {
        println!("\n🤖 You follow Rule 2: 'Next in sequence'");
        let next = next_sequence(received);
        println!("📤 You send through the slot: {}", next);

        self.escape_progress.rule_follower_score += 1;
        self.room.messages_sent.push(next.clone());

        println!("\n✓ Judge's feedback: \"You follow the pattern. But do you understand WHY?\"");
    }

    fn rule_3_response(&mut self, received: &str) {
        println!("\n🤖 You follow Rule 3: 'Count characters'");
        let count = received.chars().count();
        let response = format!("{}", count);
        println!("📤 You send through the slot: {}", response);

        self.escape_progress.rule_follower_score += 1;
        self.room.messages_sent.push(response);

        println!("\n✓ Judge's feedback: \"Mechanically sound. But mechanical nonetheless.\"");
    }

    fn combination_response(&mut self, received: &str) {
        println!("\n🧠 You break convention and combine multiple rules!");
        println!("   Rule 1 + Rule 3: (Reverse + Count)");

        let reversed = reverse_string(received);
        let count = received.chars().count();
        let response = format!("{}(count:{})", reversed, count);
        println!("📤 You send through the slot: {}", response);

        self.escape_progress.creative_score += 2;
        self.escape_progress.understanding_score += 1;
        self.room.messages_sent.push(response);

        if !self.discovered_tricks.contains(&"Combined Rules".to_string()) {
            self.discovered_tricks.push("Combined Rules".to_string());
        }

        println!("\n✓ Judge's feedback: \"Interesting! You're thinking beyond the rules.\"");
    }

    fn creative_response(&mut self, _received: &str) {
        println!("\n✨ You attempt a creative interpretation...");

        let mut rng = rand::thread_rng();
        let choice = rng.gen_range(0..3);

        match choice {
            0 => {
                let response = "心理学的观点";
                println!("📤 You send: {}", response);
                println!("   (A poetic interpretation about psychology)");
                self.escape_progress.creative_score += 2;
                self.escape_progress.understanding_score += 2;
            }
            1 => {
                let response = "感受";
                println!("📤 You send: {}", response);
                println!("   (The feeling rather than the literal meaning)");
                self.escape_progress.creative_score += 2;
                self.escape_progress.understanding_score += 2;
            }
            _ => {
                let response = "因为我在想";
                println!("📤 You send: {}", response);
                println!("   (Self-referential: 'Because I am thinking')");
                self.escape_progress.creative_score += 2;
                self.escape_progress.understanding_score += 2;
            }
        }

        if !self.discovered_tricks.contains(&"Creative Interpretation".to_string()) {
            self.discovered_tricks.push("Creative Interpretation".to_string());
        }

        println!("\n✓ Judge's feedback: \"Now THAT shows understanding!\"");
    }

    fn inventory_response(&mut self) {
        println!("\n🎒 You examine your inventory:");

        if self.inventory.contains(&"pencil".to_string()) {
            println!("   You use the pencil and paper to:");
            println!("   1. Write your own rules");
            println!("   2. Create a message bridging Chinese and English");
            println!("   3. Slip the note under the door\n");

            let message = "RULE MAKER (not RULE FOLLOWER): I understand meaning, not symbols.";
            println!("📜 Your note says: {}", message);

            self.escape_progress.creative_score += 2;
            self.escape_progress.understanding_score += 3;

            if !self.discovered_tricks.contains(&"Write Own Rules".to_string()) {
                self.discovered_tricks.push("Write Own Rules".to_string());
            }

            println!("\n✓ Judge's feedback: \"You're moving beyond the room's constraints!\"");
        } else {
            println!("   Nothing useful here.");
        }
    }

    fn question_rules(&mut self) {
        println!("\n❓ You question the fundamental rules:");
        println!("   - Why must I use the rule book?");
        println!("   - What if the rules are nonsense?");
        println!("   - Can I refuse to play?");
        println!("   - What does 'understanding' actually mean?\n");

        println!("📤 You send: \"I realize the rules are arbitrary. I'm not bound by them.\"");

        self.escape_progress.understanding_score += 3;
        self.escape_progress.creative_score += 1;

        if !self.discovered_tricks.contains(&"Question the System".to_string()) {
            self.discovered_tricks.push("Question the System".to_string());
        }

        println!("\n✓ Judge's feedback: \"Finally! Genuine understanding requires questioning!\"");
    }

    fn attempt_escape(&mut self) {
        println!("\n🔓 You attempt to escape!");
        println!("   Total Understanding Score: {} / {}",
                 self.escape_progress.understanding_score,
                 self.escape_progress.max_score);
    }

    fn check_escape_condition(&self, _received: &str, _response: &str) -> bool {
        // Escape conditions:
        // 1. Understanding score >= 8 (demonstrated genuine comprehension)
        // 2. Has discovered at least 3 different tricks
        // 3. Attempted escape

        let has_understanding = self.escape_progress.understanding_score >= 8;
        let has_tricks = self.discovered_tricks.len() >= 3;
        let valid_escape = self.turn >= 5 && _response == "8";

        has_understanding && has_tricks && valid_escape
    }

    fn ending(&self) {
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║                    🎉 YOU ESCAPED! 🎉                       ║");
        println!("╚════════════════════════════════════════════════════════════╝\n");

        println!("🔓 The door clicks open.\n");

        println!("📋 FINAL SCORES:");
        println!("   Rule-Following:        {} / 5", self.escape_progress.rule_follower_score.min(5));
        println!("   Creative Solutions:    {} / 5", self.escape_progress.creative_score.min(5));
        println!("   True Understanding:    {} / 5", self.escape_progress.understanding_score.min(5));

        println!("\n🧠 THE JUDGE'S VERDICT:");
        println!("   You have escaped the Chinese Room not by brute-forcing the");
        println!("   rules, but by recognizing their arbitrary nature.");
        println!("\n   Searle's question was: Can a machine follow rules without");
        println!("   understanding? You proved that TRUE understanding requires:");
        println!("   • Recognizing limitations of rule systems");
        println!("   • Creating novel solutions");
        println!("   • Questioning the framework itself");
        println!("   • Demonstrating intentionality and meaning-making\n");

        println!("🌟 PHILOSOPHICAL INSIGHT:");
        println!("   Understanding isn't symbol manipulation. It's the ability to");
        println!("   transcend the symbols, recognize patterns, and create meaning.");
        println!("\n   The room couldn't contain you because you did something");
        println!("   no rule book could predict: you UNDERSTOOD.\n");

        println!("✨ Game Complete!");
    }

    fn timeout_ending(&self) {
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║              ⏰ TIME'S UP - GAME OVER ⏰                      ║");
        println!("╚════════════════════════════════════════════════════════════╝\n");

        println!("The judge grows impatient...\n");

        println!("📊 ANALYSIS:");
        println!("   Rule-Following Score:   {}", self.escape_progress.rule_follower_score);
        println!("   Creative Score:         {}", self.escape_progress.creative_score);
        println!("   Understanding Score:    {}", self.escape_progress.understanding_score);

        println!("\n🤔 JUDGE'S FINAL WORDS:");
        if self.escape_progress.understanding_score < 3 {
            println!("   \"You were just mechanically following rules. No escape.\"");
        } else if self.escape_progress.understanding_score < 6 {
            println!("   \"You showed some creativity, but not enough understanding.\"");
        } else {
            println!("   \"You almost had it... next time, keep pushing harder.\"");
        }

        println!("\n💡 TIP FOR NEXT TIME:");
        println!("   Try discovering multiple creative approaches and questioning");
        println!("   the fundamental nature of the rules themselves.\n");
    }
}

impl Room {
    fn new() -> Self {
        Room {
            locked: true,
            rule_following_only: true,
            messages_received: Vec::new(),
            messages_sent: Vec::new(),
        }
    }
}

impl RuleBook {
    fn new() -> Self {
        let mut rules = HashMap::new();

        rules.insert(
            "Rule 1".to_string(),
            vec!["Reverse the received string".to_string()],
        );

        rules.insert(
            "Rule 2".to_string(),
            vec!["Find next character in sequence".to_string()],
        );

        rules.insert(
            "Rule 3".to_string(),
            vec!["Count characters and respond with number".to_string()],
        );

        let mut meta_rules = HashMap::new();
        meta_rules.insert(
            "Meta".to_string(),
            "All rules are perfectly arbitrary. None have real meaning.".to_string(),
        );

        RuleBook { rules, meta_rules }
    }
}

// Helper functions for rule processing

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn next_sequence(s: &str) -> String {
    s.chars()
        .map(|c| {
            let code = c as u32;
            char::from_u32(code + 1).unwrap_or(c)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("abc"), "cba");
        assert_eq!(reverse_string("你好"), "好你");
    }

    #[test]
    fn test_escape_progress() {
        let progress = EscapeProgress {
            rule_follower_score: 2,
            creative_score: 3,
            understanding_score: 4,
            max_score: 15,
        };
        assert_eq!(progress.rule_follower_score + progress.creative_score + progress.understanding_score, 9);
    }
}
