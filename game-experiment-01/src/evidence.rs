use crate::memory::{MemoryId, Time};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub items: Vec<EvidenceItem>,
    pub contradictions: Vec<Contradiction>,
}

impl Evidence {
    pub fn new() -> Self {
        Evidence {
            items: Vec::new(),
            contradictions: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: EvidenceItem) {
        if !self.items.iter().any(|e| e.id == item.id) {
            self.items.push(item);
        }
    }

    pub fn add_contradiction(&mut self, contradiction: Contradiction) {
        if !self.contradictions.iter().any(|c| c.id == contradiction.id) {
            self.contradictions.push(contradiction);
        }
    }

    pub fn has_evidence(&self, evidence_id: &str) -> bool {
        self.items.iter().any(|e| e.id == evidence_id)
    }

    pub fn display_all(&self) {
        println!("\n=== EVIDENCE COLLECTED ===\n");

        if self.items.is_empty() {
            println!("No evidence collected yet.");
        } else {
            for (i, item) in self.items.iter().enumerate() {
                println!("{}. {}", i + 1, item.description);
                if let Some(time) = item.timestamp {
                    println!("   Time: {}", crate::memory::format_time(time));
                }
                if !item.involves.is_empty() {
                    println!("   Involves: {}", item.involves.join(", "));
                }
                println!();
            }
        }

        if !self.contradictions.is_empty() {
            println!("\n=== CONTRADICTIONS FOUND ===\n");
            for (i, contradiction) in self.contradictions.iter().enumerate() {
                println!("{}. {}", i + 1, contradiction.description);
                println!();
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceItem {
    pub id: String,
    pub description: String,
    pub source: EvidenceSource,
    pub timestamp: Option<Time>,
    pub involves: Vec<String>, // Character names
}

impl EvidenceItem {
    pub fn new(id: String, description: String, source: EvidenceSource) -> Self {
        EvidenceItem {
            id,
            description,
            source,
            timestamp: None,
            involves: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceSource {
    Memory(MemoryId),
    Object(String),
    Testimony(String),
    Observation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contradiction {
    pub id: String,
    pub description: String,
    pub evidence1_id: String,
    pub evidence2_id: String,
    pub contradiction_type: ContradictionType,
}

impl Contradiction {
    pub fn new(
        id: String,
        description: String,
        evidence1_id: String,
        evidence2_id: String,
        contradiction_type: ContradictionType,
    ) -> Self {
        Contradiction {
            id,
            description,
            evidence1_id,
            evidence2_id,
            contradiction_type,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ContradictionType {
    TimelineConflict,
    LocationConflict,
    ActionConflict,
    MotiveRevelation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub events: BTreeMap<Time, Vec<TimelineEvent>>,
}

impl Timeline {
    pub fn new() -> Self {
        Timeline {
            events: BTreeMap::new(),
        }
    }

    pub fn add_event(&mut self, event: TimelineEvent) {
        self.events
            .entry(event.time)
            .or_insert_with(Vec::new)
            .push(event);
    }

    pub fn display(&self) {
        println!("\n=== RECONSTRUCTED TIMELINE ===\n");

        if self.events.is_empty() {
            println!("No timeline events recorded yet.");
            return;
        }

        for (time, events) in &self.events {
            println!("{}", crate::memory::format_time(*time));
            for event in events {
                println!("  - {} at {}", event.description, event.location);
                if let Some(ref source) = event.source {
                    println!("    (From: {})", source);
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub time: Time,
    pub description: String,
    pub location: String,
    pub character: String,
    pub certainty: f32, // 0.0 to 1.0
    pub source: Option<String>, // Where this information came from
}

impl TimelineEvent {
    pub fn new(time: Time, description: String, location: String, character: String) -> Self {
        TimelineEvent {
            time,
            description,
            location,
            character,
            certainty: 1.0,
            source: None,
        }
    }
}
