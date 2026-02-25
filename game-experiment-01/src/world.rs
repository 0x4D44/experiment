use crate::commands::Direction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type LocationId = String;
pub type ObjectId = String;
pub type CharacterId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub locations: HashMap<LocationId, Location>,
    pub objects: HashMap<ObjectId, Object>,
    pub characters: HashMap<CharacterId, Character>,
}

impl World {
    pub fn new() -> Self {
        World {
            locations: HashMap::new(),
            objects: HashMap::new(),
            characters: HashMap::new(),
        }
    }

    pub fn add_location(&mut self, location: Location) {
        self.locations.insert(location.id.clone(), location);
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.insert(object.id.clone(), object);
    }

    pub fn add_character(&mut self, character: Character) {
        self.characters.insert(character.id.clone(), character);
    }

    pub fn get_location(&self, id: &str) -> Option<&Location> {
        self.locations.get(id)
    }

    pub fn get_location_mut(&mut self, id: &str) -> Option<&mut Location> {
        self.locations.get_mut(id)
    }

    pub fn get_object(&self, id: &str) -> Option<&Object> {
        self.objects.get(id)
    }

    pub fn get_character(&self, id: &str) -> Option<&Character> {
        self.characters.get(id)
    }

    pub fn find_object_by_name(&self, name: &str, location_id: &str) -> Option<&Object> {
        let location = self.get_location(location_id)?;

        for obj_id in &location.objects {
            if let Some(obj) = self.get_object(obj_id) {
                if obj.matches_name(name) {
                    return Some(obj);
                }
            }
        }
        None
    }

    pub fn find_character_by_name(&self, name: &str, location_id: &str) -> Option<&Character> {
        let location = self.get_location(location_id)?;

        for char_id in &location.characters {
            if let Some(character) = self.get_character(char_id) {
                if character.matches_name(name) {
                    return Some(character);
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: LocationId,
    pub name: String,
    pub description: String,
    pub exits: HashMap<Direction, LocationId>,
    pub objects: Vec<ObjectId>,
    pub characters: Vec<CharacterId>,
    pub visited: bool,
}

impl Location {
    pub fn new(id: String, name: String, description: String) -> Self {
        Location {
            id,
            name,
            description,
            exits: HashMap::new(),
            objects: Vec::new(),
            characters: Vec::new(),
            visited: false,
        }
    }

    pub fn add_exit(&mut self, direction: Direction, destination: LocationId) {
        self.exits.insert(direction, destination);
    }

    pub fn add_object(&mut self, object_id: ObjectId) {
        if !self.objects.contains(&object_id) {
            self.objects.push(object_id);
        }
    }

    pub fn add_character(&mut self, character_id: CharacterId) {
        if !self.characters.contains(&character_id) {
            self.characters.push(character_id);
        }
    }

    pub fn describe_exits(&self) -> String {
        if self.exits.is_empty() {
            return "There are no obvious exits.".to_string();
        }

        let mut exit_descriptions: Vec<String> = self.exits
            .keys()
            .map(|dir| dir.as_str().to_string())
            .collect();
        exit_descriptions.sort();

        format!("Exits: {}", exit_descriptions.join(", "))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub aliases: Vec<String>,
    pub can_touch: bool,
    pub can_take: bool,
    pub is_evidence: bool,
    pub associated_memories: Vec<String>, // Memory IDs
}

impl Object {
    pub fn new(id: String, name: String, description: String) -> Self {
        Object {
            id,
            name,
            description,
            aliases: Vec::new(),
            can_touch: true,
            can_take: false,
            is_evidence: false,
            associated_memories: Vec::new(),
        }
    }

    pub fn matches_name(&self, input: &str) -> bool {
        let input_lower = input.to_lowercase();
        let name_lower = self.name.to_lowercase();

        if name_lower == input_lower || name_lower.contains(&input_lower) {
            return true;
        }

        for alias in &self.aliases {
            if alias.to_lowercase() == input_lower || alias.to_lowercase().contains(&input_lower) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub description: String,
    pub aliases: Vec<String>,
    pub can_touch: bool,
    pub dialogue: HashMap<String, String>, // topic -> response
    pub is_suspect: bool,
    pub is_accused: bool,
    pub associated_memories: Vec<String>, // Memory IDs
}

impl Character {
    pub fn new(id: String, name: String, description: String) -> Self {
        Character {
            id,
            name,
            description,
            aliases: Vec::new(),
            can_touch: true,
            dialogue: HashMap::new(),
            is_suspect: true,
            is_accused: false,
            associated_memories: Vec::new(),
        }
    }

    pub fn matches_name(&self, input: &str) -> bool {
        let input_lower = input.to_lowercase();
        let name_lower = self.name.to_lowercase();

        if name_lower == input_lower || name_lower.contains(&input_lower) {
            return true;
        }

        for alias in &self.aliases {
            if alias.to_lowercase() == input_lower || alias.to_lowercase().contains(&input_lower) {
                return true;
            }
        }

        false
    }

    pub fn add_dialogue(&mut self, topic: String, response: String) {
        self.dialogue.insert(topic.to_lowercase(), response);
    }

    pub fn get_dialogue(&self, topic: &str) -> Option<&String> {
        self.dialogue.get(&topic.to_lowercase())
    }
}
