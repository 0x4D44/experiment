use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type MemoryId = String;
pub type Time = u32; // Minutes since 8:00 PM (e.g., 60 = 9:00 PM)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryGraph {
    pub memories: HashMap<MemoryId, Memory>,
    pub discovered: Vec<MemoryId>,
}

impl MemoryGraph {
    pub fn new() -> Self {
        MemoryGraph {
            memories: HashMap::new(),
            discovered: Vec::new(),
        }
    }

    pub fn add_memory(&mut self, memory: Memory) {
        self.memories.insert(memory.id.clone(), memory);
    }

    pub fn discover_memory(&mut self, memory_id: &str) -> Option<&Memory> {
        if let Some(memory) = self.memories.get(memory_id) {
            if !self.discovered.contains(&memory_id.to_string()) {
                self.discovered.push(memory_id.to_string());
            }
            Some(memory)
        } else {
            None
        }
    }

    pub fn get_memory(&self, memory_id: &str) -> Option<&Memory> {
        self.memories.get(memory_id)
    }

    pub fn is_discovered(&self, memory_id: &str) -> bool {
        self.discovered.contains(&memory_id.to_string())
    }

    pub fn get_discovered_memories(&self) -> Vec<&Memory> {
        self.discovered
            .iter()
            .filter_map(|id| self.memories.get(id))
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: MemoryId,
    pub owner: String, // Character ID
    pub timestamp: Time,
    pub location: String, // Location ID
    pub title: String,
    pub content: String,
    pub emotional_state: EmotionalState,
    pub reliability: f32, // 0.0 = completely false, 1.0 = accurate
    pub fragments: Vec<MemoryFragment>,
    pub connections: Vec<MemoryId>, // Related memories
    pub unlock_condition: Option<UnlockCondition>,
}

impl Memory {
    pub fn new(id: String, owner: String, timestamp: Time, location: String) -> Self {
        Memory {
            id,
            owner,
            timestamp,
            location,
            title: String::new(),
            content: String::new(),
            emotional_state: EmotionalState::Calm,
            reliability: 1.0,
            fragments: Vec::new(),
            connections: Vec::new(),
            unlock_condition: None,
        }
    }

    pub fn display(&self) {
        println!("\n{}", "=".repeat(60));
        println!("[ MEMORY FRAGMENT - {} ]", self.title);
        println!("Time: {}, Location: {}", format_time(self.timestamp), self.location);
        println!("Emotional State: {:?}, Reliability: {:.0}%",
                 self.emotional_state, self.reliability * 100.0);
        println!("{}", "-".repeat(60));
        println!("{}", self.content);

        if !self.fragments.is_empty() {
            println!("\n{}", "-".repeat(60));
            println!("Details visible:");
            for (i, fragment) in self.fragments.iter().enumerate() {
                if fragment.revealed {
                    println!("  {}. {}", i + 1, fragment.description);
                    if fragment.can_focus && !fragment.focused {
                        println!("     [Use FOCUS ON {} to examine more closely]", fragment.focus_keyword);
                    } else if fragment.focused && !fragment.focus_content.is_empty() {
                        println!("     -> {}", fragment.focus_content);
                    }
                }
            }
        }

        println!("{}", "=".repeat(60));
    }

    pub fn get_fragment_by_keyword(&self, keyword: &str) -> Option<&MemoryFragment> {
        self.fragments.iter().find(|f|
            f.focus_keyword.to_lowercase() == keyword.to_lowercase()
        )
    }

    pub fn get_fragment_mut_by_keyword(&mut self, keyword: &str) -> Option<&mut MemoryFragment> {
        self.fragments.iter_mut().find(|f|
            f.focus_keyword.to_lowercase() == keyword.to_lowercase()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFragment {
    pub description: String,
    pub detail_level: usize, // 0 = vague, 3 = highly detailed
    pub revealed: bool,
    pub can_focus: bool,
    pub focused: bool,
    pub focus_keyword: String,
    pub focus_content: String,
}

impl MemoryFragment {
    pub fn new(description: String, focus_keyword: String) -> Self {
        MemoryFragment {
            description,
            detail_level: 1,
            revealed: true,
            can_focus: true,
            focused: false,
            focus_keyword,
            focus_content: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EmotionalState {
    Calm,
    Anxious,
    Fearful,
    Angry,
    Guilty,
    Deceptive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnlockCondition {
    None,
    DiscoveredMemory(MemoryId),
    HasEvidence(String),
    TalkedToCharacter(String),
    GamePhase(String),
}

pub fn format_time(time: Time) -> String {
    let hour = 20 + time / 60; // Starts at 8:00 PM
    let minute = time % 60;
    format!("{}:{:02} PM", if hour > 12 { hour - 12 } else { hour }, minute)
}
