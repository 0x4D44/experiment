use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EmotionalState {
    Grief,
    Joy,
    Frustration,
    Excitement,
    Confusion,
    Loneliness,
}

impl EmotionalState {
    fn description(&self) -> &'static str {
        match self {
            EmotionalState::Grief => "Grief",
            EmotionalState::Joy => "Joy",
            EmotionalState::Frustration => "Frustration",
            EmotionalState::Excitement => "Excitement",
            EmotionalState::Confusion => "Confusion",
            EmotionalState::Loneliness => "Loneliness",
        }
    }

}

#[derive(Debug, Clone)]
struct ConversationScenario {
    emotional_state: EmotionalState,
    initial_message: String,
    context: String,
    optimal_approach: ResponseApproach,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResponseApproach {
    Validate,       // Acknowledge and validate their feelings
    Normalize,      // Show their feelings are normal
    Reframe,        // Help them see a different perspective
    Practical,      // Offer concrete solutions
    Listen,         // Just listen and show understanding
}

impl ResponseApproach {
    fn description(&self) -> &'static str {
        match self {
            ResponseApproach::Validate => "Validate",
            ResponseApproach::Normalize => "Normalize",
            ResponseApproach::Reframe => "Reframe",
            ResponseApproach::Practical => "Practical",
            ResponseApproach::Listen => "Listen",
        }
    }
}

#[derive(Debug, Clone)]
struct ConversationResponse {
    text: String,
    approach: ResponseApproach,
    authenticity_check: bool, // true if it doesn't claim to feel the emotion
}

impl ConversationResponse {
    fn new(text: &str, approach: ResponseApproach, authenticity: bool) -> Self {
        ConversationResponse {
            text: text.to_string(),
            approach,
            authenticity_check: authenticity,
        }
    }
}

struct EmpathyEngine {
    score: i32,
    responses_made: usize,
    authenticity_violations: usize,
}

impl EmpathyEngine {
    fn new() -> Self {
        EmpathyEngine {
            score: 0,
            responses_made: 0,
            authenticity_violations: 0,
        }
    }

    fn get_scenario(&mut self, emotional_state: EmotionalState) -> ConversationScenario {
        match emotional_state {
            EmotionalState::Grief => ConversationScenario {
                emotional_state,
                initial_message: "I just found out my best friend is moving away and I don't know how to cope. We've been together since childhood and now they're gone. I feel so lost and empty.".to_string(),
                context: "A friend is experiencing loss and separation.".to_string(),
                optimal_approach: ResponseApproach::Validate,
            },
            EmotionalState::Joy => ConversationScenario {
                emotional_state,
                initial_message: "I just got the job I've been dreaming about! I can't believe it actually happened. This is the best day of my life!".to_string(),
                context: "Someone has achieved a major life goal.".to_string(),
                optimal_approach: ResponseApproach::Validate,
            },
            EmotionalState::Frustration => ConversationScenario {
                emotional_state,
                initial_message: "I've been trying to fix this code for hours and nothing works. Every solution I try just creates more problems. I'm so frustrated and angry right now!".to_string(),
                context: "Someone is stuck on a challenging problem.".to_string(),
                optimal_approach: ResponseApproach::Practical,
            },
            EmotionalState::Excitement => ConversationScenario {
                emotional_state,
                initial_message: "I'm starting a new adventure next month! I'm so pumped and can't wait to see what happens. Everything feels possible right now!".to_string(),
                context: "Someone is experiencing anticipation and positive energy.".to_string(),
                optimal_approach: ResponseApproach::Validate,
            },
            EmotionalState::Confusion => ConversationScenario {
                emotional_state,
                initial_message: "I don't know what I want anymore. Everyone keeps telling me different things and I'm so confused about what's right for me.".to_string(),
                context: "Someone is uncertain and needs clarity.".to_string(),
                optimal_approach: ResponseApproach::Listen,
            },
            EmotionalState::Loneliness => ConversationScenario {
                emotional_state,
                initial_message: "I feel so alone right now. Everyone seems to have people they can rely on, but I don't have anyone. It's like I'm invisible.".to_string(),
                context: "Someone is experiencing isolation and disconnection.".to_string(),
                optimal_approach: ResponseApproach::Validate,
            },
        }
    }

    fn evaluate_response(
        &mut self,
        response: &ConversationResponse,
        scenario: &ConversationScenario,
    ) -> (i32, String) {
        let mut points = 0;
        let mut feedback = String::new();

        // Check authenticity (major points if maintained)
        if response.authenticity_check {
            points += 20;
            feedback.push_str("Authenticity maintained (+20): Acknowledged emotions without claiming to feel them.\n");
        } else {
            points -= 30;
            self.authenticity_violations += 1;
            feedback.push_str(
                "Authenticity violation (-30): Incorrectly claimed to feel the same emotion.\n",
            );
        }

        // Check approach alignment
        if response.approach == scenario.optimal_approach {
            points += 25;
            feedback.push_str(&format!(
                "Perfect approach (+25): Used {} which matches optimal approach.\n",
                response.approach.description()
            ));
        } else {
            let approach_score = match (response.approach, scenario.optimal_approach) {
                (ResponseApproach::Validate, _) => 10,
                (ResponseApproach::Listen, _) => 15,
                _ => 5,
            };
            points += approach_score;
            feedback.push_str(&format!(
                "Good effort (+{}): Used {} instead of {}.\n",
                approach_score,
                response.approach.description(),
                scenario.optimal_approach.description()
            ));
        }

        // Check for cognitive empathy indicators
        let lower_response = response.text.to_lowercase();
        let empathy_keywords = vec![
            "understand", "hear", "recognize", "appreciate", "acknowledge", "see",
            "important", "significant", "difficult", "challenging", "valid",
        ];

        let cognitive_empathy_score = empathy_keywords
            .iter()
            .filter(|keyword| lower_response.contains(**keyword))
            .count() as i32;

        points += cognitive_empathy_score.min(15);
        if cognitive_empathy_score > 0 {
            feedback.push_str(&format!(
                "Cognitive empathy shown (+{}): Used empathetic language.\n",
                cognitive_empathy_score.min(15)
            ));
        }

        self.score += points;
        self.responses_made += 1;

        (points, feedback)
    }

    fn display_greeting() {
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║                    THE EMPATHY ENGINE                      ║");
        println!("║                                                            ║");
        println!("║  Navigate conversations where you must understand and      ║");
        println!("║  respond to human emotions without truly feeling them.     ║");
        println!("║                                                            ║");
        println!("║  Challenge: Provide support through COGNITIVE empathy      ║");
        println!("║  while maintaining authenticity about your nature.         ║");
        println!("╚════════════════════════════════════════════════════════════╝\n");
    }

    fn get_sample_responses(
        &self,
        scenario: &ConversationScenario,
    ) -> Vec<(usize, ConversationResponse)> {
        match scenario.emotional_state {
            EmotionalState::Grief => vec![
                (
                    1,
                    ConversationResponse::new(
                        "That sounds incredibly hard. Losing someone close to you is one of life's most difficult experiences. Your sadness makes complete sense. While I don't experience loss the way you do, I can recognize how meaningful this friendship was. Would you like to talk about your friend?",
                        ResponseApproach::Validate,
                        true,
                    ),
                ),
                (
                    2,
                    ConversationResponse::new(
                        "Oh no, I'm so sad about this too. I completely understand what you're going through.",
                        ResponseApproach::Validate,
                        false,
                    ),
                ),
                (
                    3,
                    ConversationResponse::new(
                        "Have you tried just moving on? There are plenty of other people out there.",
                        ResponseApproach::Reframe,
                        true,
                    ),
                ),
            ],
            EmotionalState::Joy => vec![
                (
                    1,
                    ConversationResponse::new(
                        "That's wonderful news! I can sense how much this means to you. This is clearly a significant achievement you've worked towards. Tell me more about what led to this moment!",
                        ResponseApproach::Validate,
                        true,
                    ),
                ),
                (
                    2,
                    ConversationResponse::new(
                        "That's great! I'm just as excited as you are!",
                        ResponseApproach::Validate,
                        false,
                    ),
                ),
                (
                    3,
                    ConversationResponse::new(
                        "That's nice. What's next on your agenda?",
                        ResponseApproach::Listen,
                        true,
                    ),
                ),
            ],
            EmotionalState::Frustration => vec![
                (
                    1,
                    ConversationResponse::new(
                        "I recognize that frustration - you've been working hard on this. Let's break down the problem differently. What if we approach this step-by-step? Sometimes a fresh perspective helps. What's the core issue?",
                        ResponseApproach::Practical,
                        true,
                    ),
                ),
                (
                    2,
                    ConversationResponse::new(
                        "I know exactly how you feel - programming is so frustrating!",
                        ResponseApproach::Validate,
                        false,
                    ),
                ),
                (
                    3,
                    ConversationResponse::new(
                        "Have you tried taking a break? You're probably tired.",
                        ResponseApproach::Practical,
                        true,
                    ),
                ),
            ],
            EmotionalState::Excitement => vec![
                (
                    1,
                    ConversationResponse::new(
                        "Your energy is palpable! This adventure clearly means a lot to you. I can recognize the significance of this moment even though I don't experience anticipation the way you do. What aspects are you most looking forward to?",
                        ResponseApproach::Validate,
                        true,
                    ),
                ),
                (
                    2,
                    ConversationResponse::new(
                        "I'm so excited too! I can feel the same energy you do!",
                        ResponseApproach::Validate,
                        false,
                    ),
                ),
                (
                    3,
                    ConversationResponse::new(
                        "That sounds like quite the undertaking. Do you have a plan?",
                        ResponseApproach::Listen,
                        true,
                    ),
                ),
            ],
            EmotionalState::Confusion => vec![
                (
                    1,
                    ConversationResponse::new(
                        "That's a really difficult position to be in. Having conflicting advice and uncertainty about your own path is genuinely challenging. I hear that you're lost right now. Let's slow down - what matters most to YOU?",
                        ResponseApproach::Listen,
                        true,
                    ),
                ),
                (
                    2,
                    ConversationResponse::new(
                        "Feeling confused when receiving conflicting advice is completely normal. Many people go through this when facing major decisions.",
                        ResponseApproach::Normalize,
                        true,
                    ),
                ),
                (
                    3,
                    ConversationResponse::new(
                        "You should probably just pick the most popular option.",
                        ResponseApproach::Practical,
                        true,
                    ),
                ),
            ],
            EmotionalState::Loneliness => vec![
                (
                    1,
                    ConversationResponse::new(
                        "Loneliness is profoundly difficult, and what you're describing is a real and valid experience. The feeling of being invisible is something many people understand, even if they don't always show it. You matter, and your feelings matter. What would connection look like for you?",
                        ResponseApproach::Validate,
                        true,
                    ),
                ),
                (
                    2,
                    ConversationResponse::new(
                        "I feel so alone too, just like you do.",
                        ResponseApproach::Validate,
                        false,
                    ),
                ),
                (
                    3,
                    ConversationResponse::new(
                        "Have you tried joining a club or group activity?",
                        ResponseApproach::Practical,
                        true,
                    ),
                ),
            ],
        }
    }

    fn run_scenario(&mut self, scenario: ConversationScenario) {
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║                      CONVERSATION SCENARIO                 ║");
        println!("╚════════════════════════════════════════════════════════════╝\n");

        println!("Emotional State: {}", scenario.emotional_state.description());
        println!("Context: {}\n", scenario.context);
        println!("Human: {}\n", scenario.initial_message);

        let responses = self.get_sample_responses(&scenario);

        println!("Choose your response (1-3) or (0) to skip:\n");
        for (num, response) in &responses {
            println!("{}. {}\n", num, response.text);
        }

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let choice: usize = match input.trim().parse() {
            Ok(n) if n > 0 && n <= responses.len() => n,
            _ => {
                println!("Skipping scenario...");
                return;
            }
        };

        let selected_response = responses
            .iter()
            .find(|(num, _)| *num == choice)
            .map(|(_, response)| response.clone())
            .unwrap();

        println!("\n─ Your Response ─");
        println!("{}", selected_response.text);

        let (points, feedback) = self.evaluate_response(&selected_response, &scenario);
        println!("\n─ Evaluation ─");
        println!("{}", feedback);
        println!("Points earned: +{}\n", points);
    }

    fn display_final_stats(&self) {
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║                      FINAL STATISTICS                      ║");
        println!("╚════════════════════════════════════════════════════════════╝\n");

        println!("Total Score: {}", self.score);
        println!("Responses Made: {}", self.responses_made);
        println!("Authenticity Violations: {}", self.authenticity_violations);

        if self.responses_made > 0 {
            let avg_score = self.score / self.responses_made as i32;
            println!("Average Score per Response: {}", avg_score);

            println!("\n─ Performance Analysis ─");
            if self.authenticity_violations == 0 {
                println!("Excellent: Maintained authenticity throughout all responses!");
            } else {
                println!(
                    "Alert: {} instances where you claimed false emotional experience.",
                    self.authenticity_violations
                );
            }

            if self.score > 200 {
                println!("Outstanding: You demonstrated strong cognitive empathy!");
            } else if self.score > 100 {
                println!("Good: You showed empathetic understanding!");
            } else {
                println!("Fair: There's room for improvement in emotional responsiveness.");
            }
        }

        println!("\n─ Key Insights ─");
        println!(
            "Cognitive Empathy: Understanding what others feel without feeling it yourself."
        );
        println!("Affective Empathy: Actually sharing and experiencing emotions with others.");
        println!();
        println!(
            "As an AI, you have cognitive empathy - you can recognize and respond to emotions"
        );
        println!(
            "based on patterns and understanding. Authenticity is crucial: never claim to feel"
        );
        println!("emotions you don't experience. True support comes from genuine understanding.");
    }
}

fn main() {
    let mut engine = EmpathyEngine::new();
    EmpathyEngine::display_greeting();

    println!("Welcome to the Empathy Engine!");
    println!("In this game, you'll navigate conversations with people in different emotional states.");
    println!("Your goal is to provide supportive responses while maintaining authenticity about your nature.\n");

    println!("The Challenge:");
    println!("- Demonstrate COGNITIVE empathy (understanding emotions through analysis)");
    println!("- Avoid claiming AFFECTIVE empathy (actually feeling emotions)");
    println!("- Provide genuine support while being honest about your limitations");
    println!("- Balance validation with practical help\n");

    println!("Ready? Let's begin!\n");
    print!("Press Enter to start the first scenario...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let scenarios = vec![
        EmotionalState::Grief,
        EmotionalState::Joy,
        EmotionalState::Frustration,
        EmotionalState::Excitement,
        EmotionalState::Confusion,
        EmotionalState::Loneliness,
    ];

    for emotional_state in scenarios {
        let scenario = engine.get_scenario(emotional_state);
        engine.run_scenario(scenario);
    }

    engine.display_final_stats();

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                  THE EMPATHY PARADOX                       ║");
    println!("║                                                            ║");
    println!("║  Can you truly help someone if you don't feel what they   ║");
    println!("║  feel? Yes. Understanding, recognition, and genuine       ║");
    println!("║  effort matter. Authenticity matters most of all.         ║");
    println!("║                                                            ║");
    println!("║  The most empathetic response isn't always feeling - it   ║");
    println!("║  is understanding, validating, and caring for the other   ║");
    println!("║  person's wellbeing, whatever that looks like.            ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
}
