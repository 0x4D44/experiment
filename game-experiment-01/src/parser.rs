use crate::commands::{Command, Direction};

pub struct Parser {
    // Could add synonym mappings here
}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&self, input: &str) -> Result<Command, String> {
        let input = input.trim().to_lowercase();
        let words: Vec<&str> = input.split_whitespace().collect();

        if words.is_empty() {
            return Err("Please enter a command.".to_string());
        }

        let first_word = words[0];

        // Try to match commands
        match first_word {
            // Movement
            "go" | "move" | "walk" | "travel" => {
                if words.len() < 2 {
                    return Err("Go where? (e.g., 'go north')".to_string());
                }
                let direction = self.parse_direction(words[1])?;
                Ok(Command::Go(direction))
            }
            "n" | "north" => Ok(Command::Go(Direction::North)),
            "s" | "south" => Ok(Command::Go(Direction::South)),
            "e" | "east" => Ok(Command::Go(Direction::East)),
            "w" | "west" => Ok(Command::Go(Direction::West)),
            "u" | "up" => Ok(Command::Go(Direction::Up)),
            "d" | "down" => Ok(Command::Go(Direction::Down)),

            "look" | "l" => Ok(Command::Look),

            "examine" | "x" | "inspect" | "check" => {
                if words.len() < 2 {
                    return Err("Examine what?".to_string());
                }
                let target = words[1..].join(" ");
                Ok(Command::Examine(target))
            }

            // Investigation
            "touch" => {
                if words.len() < 2 {
                    return Err("Touch what or whom?".to_string());
                }
                let target = words[1..].join(" ");
                Ok(Command::Touch(target))
            }

            "focus" => {
                if words.len() < 2 {
                    return Err("Focus on what detail?".to_string());
                }
                // Handle "focus on X" or just "focus X"
                let start_idx = if words.len() > 1 && words[1] == "on" { 2 } else { 1 };
                if start_idx >= words.len() {
                    return Err("Focus on what detail?".to_string());
                }
                let detail = words[start_idx..].join(" ");
                Ok(Command::Focus(detail))
            }

            "remember" | "recall" => {
                if words.len() < 2 {
                    return Err("Remember what?".to_string());
                }
                let topic = words[1..].join(" ");
                Ok(Command::Remember(topic))
            }

            "compare" => {
                // Parse "compare X with Y"
                let with_idx = words.iter().position(|&w| w == "with" || w == "and");
                if let Some(idx) = with_idx {
                    if idx <= 1 || idx >= words.len() - 1 {
                        return Err("Usage: compare <memory1> with <memory2>".to_string());
                    }
                    let memory1 = words[1..idx].join(" ");
                    let memory2 = words[idx + 1..].join(" ");
                    Ok(Command::Compare(memory1, memory2))
                } else {
                    Err("Usage: compare <memory1> with <memory2>".to_string())
                }
            }

            // Interaction
            "talk" => {
                if words.len() < 2 {
                    return Err("Talk to whom?".to_string());
                }
                // Handle "talk to X" or just "talk X"
                let start_idx = if words.len() > 1 && words[1] == "to" { 2 } else { 1 };
                if start_idx >= words.len() {
                    return Err("Talk to whom?".to_string());
                }
                let character = words[start_idx..].join(" ");
                Ok(Command::Talk(character))
            }

            "ask" => {
                // Parse "ask X about Y"
                let about_idx = words.iter().position(|&w| w == "about");
                if let Some(idx) = about_idx {
                    if idx <= 1 || idx >= words.len() - 1 {
                        return Err("Usage: ask <person> about <topic>".to_string());
                    }
                    let character = words[1..idx].join(" ");
                    let topic = words[idx + 1..].join(" ");
                    Ok(Command::AskAbout(character, topic))
                } else {
                    Err("Usage: ask <person> about <topic>".to_string())
                }
            }

            "show" => {
                // Parse "show X to Y"
                let to_idx = words.iter().position(|&w| w == "to");
                if let Some(idx) = to_idx {
                    if idx <= 1 || idx >= words.len() - 1 {
                        return Err("Usage: show <item> to <person>".to_string());
                    }
                    let item = words[1..idx].join(" ");
                    let character = words[idx + 1..].join(" ");
                    Ok(Command::Show(item, character))
                } else {
                    Err("Usage: show <item> to <person>".to_string())
                }
            }

            "confront" => {
                // Parse "confront X about Y"
                let about_idx = words.iter().position(|&w| w == "about");
                if let Some(idx) = about_idx {
                    if idx <= 1 || idx >= words.len() - 1 {
                        return Err("Usage: confront <person> about <topic>".to_string());
                    }
                    let character = words[1..idx].join(" ");
                    let topic = words[idx + 1..].join(" ");
                    Ok(Command::Confront(character, topic))
                } else {
                    Err("Usage: confront <person> about <topic>".to_string())
                }
            }

            "accuse" => {
                if words.len() < 2 {
                    return Err("Accuse whom?".to_string());
                }
                let character = words[1..].join(" ");
                Ok(Command::Accuse(character))
            }

            // Management
            "inventory" | "i" => Ok(Command::Inventory),
            "timeline" => Ok(Command::Timeline),
            "notes" | "evidence" => Ok(Command::Notes),
            "help" | "h" | "?" => Ok(Command::Help),
            "quit" | "q" | "exit" => Ok(Command::Quit),

            _ => Err(format!(
                "I don't understand '{}'. Type 'help' for a list of commands.",
                first_word
            )),
        }
    }

    fn parse_direction(&self, word: &str) -> Result<Direction, String> {
        match word {
            "n" | "north" => Ok(Direction::North),
            "s" | "south" => Ok(Direction::South),
            "e" | "east" => Ok(Direction::East),
            "w" | "west" => Ok(Direction::West),
            "u" | "up" => Ok(Direction::Up),
            "d" | "down" => Ok(Direction::Down),
            _ => Err(format!("'{}' is not a valid direction.", word)),
        }
    }
}
