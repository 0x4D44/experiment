use crate::commands::{Command, Direction};
use crate::evidence::{Evidence, Timeline};
use crate::memory::{Memory, MemoryGraph};
use crate::world::{Character, CharacterId, Location, LocationId, Object, World};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub current_location: LocationId,
    pub world: World,
    pub memory_graph: MemoryGraph,
    pub timeline: Timeline,
    pub evidence: Evidence,
    pub game_phase: GamePhase,
    pub inventory: Vec<String>, // Object IDs
    pub accused_character: Option<CharacterId>,
    pub game_over: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GamePhase {
    Discovery,
    Investigation,
    Confrontation,
    Resolution,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = GameState {
            current_location: "foyer".to_string(),
            world: World::new(),
            memory_graph: MemoryGraph::new(),
            timeline: Timeline::new(),
            evidence: Evidence::new(),
            game_phase: GamePhase::Discovery,
            inventory: Vec::new(),
            accused_character: None,
            game_over: false,
        };

        // Initialize the world with all content
        state.initialize_world();
        state.initialize_memories();

        state
    }

    pub fn look(&self) {
        if let Some(location) = self.world.get_location(&self.current_location) {
            println!("\n{}", "=".repeat(60));
            println!("{}", location.name);
            println!("{}", "-".repeat(60));
            println!("{}", location.description);

            // List objects
            if !location.objects.is_empty() {
                println!("\nYou can see:");
                for obj_id in &location.objects {
                    if let Some(obj) = self.world.get_object(obj_id) {
                        println!("  - {}", obj.name);
                    }
                }
            }

            // List characters
            if !location.characters.is_empty() {
                println!("\nPeople here:");
                for char_id in &location.characters {
                    if let Some(character) = self.world.get_character(char_id) {
                        println!("  - {}", character.name);
                    }
                }
            }

            println!("\n{}", location.describe_exits());
            println!("{}", "=".repeat(60));
        }
    }

    pub fn execute_command(&mut self, command: Command) -> Result<bool, String> {
        match command {
            Command::Go(direction) => {
                self.handle_go(direction)?;
                Ok(false)
            }
            Command::Look => {
                self.look();
                Ok(false)
            }
            Command::Examine(target) => {
                self.handle_examine(&target)?;
                Ok(false)
            }
            Command::Touch(target) => {
                self.handle_touch(&target)?;
                Ok(false)
            }
            Command::Focus(detail) => {
                self.handle_focus(&detail)?;
                Ok(false)
            }
            Command::Compare(mem1, mem2) => {
                self.handle_compare(&mem1, &mem2)?;
                Ok(false)
            }
            Command::Talk(character) => {
                self.handle_talk(&character)?;
                Ok(false)
            }
            Command::AskAbout(character, topic) => {
                self.handle_ask_about(&character, &topic)?;
                Ok(false)
            }
            Command::Confront(character, topic) => {
                self.handle_confront(&character, &topic)?;
                Ok(false)
            }
            Command::Accuse(character) => {
                self.handle_accuse(&character)?;
                Ok(false)
            }
            Command::Inventory => {
                self.show_inventory();
                Ok(false)
            }
            Command::Timeline => {
                self.timeline.display();
                Ok(false)
            }
            Command::Notes => {
                self.evidence.display_all();
                Ok(false)
            }
            Command::Help => {
                self.show_help();
                Ok(false)
            }
            Command::Quit => Ok(true),
            _ => Err("Command not yet implemented.".to_string()),
        }
    }

    fn handle_go(&mut self, direction: Direction) -> Result<(), String> {
        let current_location = self.world.get_location(&self.current_location)
            .ok_or_else(|| "Current location not found!".to_string())?;

        if let Some(destination) = current_location.exits.get(&direction) {
            self.current_location = destination.clone();

            // Mark new location as visited
            if let Some(loc) = self.world.get_location_mut(&self.current_location) {
                loc.visited = true;
            }

            self.look();
            Ok(())
        } else {
            Err("You can't go that way.".to_string())
        }
    }

    fn handle_examine(&self, target: &str) -> Result<(), String> {
        // Try to find object in current location
        if let Some(obj) = self.world.find_object_by_name(target, &self.current_location) {
            println!("\n{}", obj.description);
            if obj.can_touch && !obj.associated_memories.is_empty() {
                println!("\n[You could TOUCH this to experience associated memories]");
            }
            return Ok(());
        }

        // Try to find character in current location
        if let Some(character) = self.world.find_character_by_name(target, &self.current_location) {
            println!("\n{}", character.description);
            if character.can_touch && !character.associated_memories.is_empty() {
                println!("\n[You could TOUCH {} to experience their memories]", character.name);
            }
            return Ok(());
        }

        Err(format!("You don't see '{}' here.", target))
    }

    fn handle_touch(&mut self, target: &str) -> Result<(), String> {
        // Find object or character and get their memories
        let memory_ids = if let Some(obj) = self.world.find_object_by_name(target, &self.current_location) {
            if !obj.can_touch {
                return Err("You can't touch that.".to_string());
            }
            obj.associated_memories.clone()
        } else if let Some(character) = self.world.find_character_by_name(target, &self.current_location) {
            if !character.can_touch {
                return Err(format!("{} doesn't want you to touch them.", character.name));
            }
            character.associated_memories.clone()
        } else {
            return Err(format!("You don't see '{}' here.", target));
        };

        if memory_ids.is_empty() {
            return Err("You sense no memories from this.".to_string());
        }

        // Discover and display the first undiscovered memory
        for memory_id in &memory_ids {
            if !self.memory_graph.is_discovered(memory_id) {
                // Clone the memory to avoid borrow checker issues
                let memory_clone = if let Some(memory) = self.memory_graph.memories.get(memory_id) {
                    memory.clone()
                } else {
                    continue;
                };

                // Now discover it
                self.memory_graph.discover_memory(memory_id);
                self.display_memory(&memory_clone);
                self.process_memory_discovery(&memory_clone);
                return Ok(());
            }
        }

        // All memories already discovered
        println!("You've already experienced all the memories from this.");
        Ok(())
    }

    fn handle_focus(&mut self, detail: &str) -> Result<(), String> {
        // Find the most recent memory with this focusable detail
        let discovered = self.memory_graph.discovered.clone();

        for memory_id in discovered.iter().rev() {
            if let Some(memory) = self.memory_graph.memories.get_mut(memory_id) {
                if let Some(fragment) = memory.get_fragment_mut_by_keyword(detail) {
                    if !fragment.can_focus {
                        return Err("You can't focus on that detail.".to_string());
                    }
                    if fragment.focused {
                        return Err("You've already examined this detail closely.".to_string());
                    }

                    fragment.focused = true;
                    println!("\n{}", "=".repeat(60));
                    println!("[ FOCUSING ON: {} ]", detail);
                    println!("{}", "-".repeat(60));
                    println!("{}", fragment.focus_content);
                    println!("{}", "=".repeat(60));

                    // Check if focusing reveals new evidence
                    self.check_for_new_evidence_from_focus(memory_id, detail);

                    return Ok(());
                }
            }
        }

        Err(format!("You don't remember any detail about '{}'.", detail))
    }

    fn handle_compare(&self, mem1_str: &str, mem2_str: &str) -> Result<(), String> {
        // For now, implement basic comparison logic
        // In a full implementation, this would be more sophisticated
        println!("\n{}", "=".repeat(60));
        println!("[ COMPARING MEMORIES ]");
        println!("{}", "-".repeat(60));
        println!("You carefully compare the two memories...");
        println!("(This feature would compare '{}' with '{}')", mem1_str, mem2_str);
        println!("{}", "=".repeat(60));
        Ok(())
    }

    fn handle_talk(&self, character_name: &str) -> Result<(), String> {
        if let Some(character) = self.world.find_character_by_name(character_name, &self.current_location) {
            println!("\nYou approach {}.", character.name);
            println!("\"Hello, detective,\" {} says.", character.name);
            println!("\n(Use ASK {} ABOUT <topic> to question them)", character.name.to_uppercase());
            Ok(())
        } else {
            Err(format!("{} is not here.", character_name))
        }
    }

    fn handle_ask_about(&self, character_name: &str, topic: &str) -> Result<(), String> {
        if let Some(character) = self.world.find_character_by_name(character_name, &self.current_location) {
            if let Some(response) = character.get_dialogue(topic) {
                println!("\n{} says:", character.name);
                println!("\"{}\"", response);
                Ok(())
            } else {
                println!("\n{} doesn't seem to know anything about that.", character.name);
                Ok(())
            }
        } else {
            Err(format!("{} is not here.", character_name))
        }
    }

    fn handle_confront(&self, character_name: &str, topic: &str) -> Result<(), String> {
        if let Some(character) = self.world.find_character_by_name(character_name, &self.current_location) {
            println!("\nYou confront {} about {}.", character.name, topic);
            println!("(Confrontation system would go here)");
            Ok(())
        } else {
            Err(format!("{} is not here.", character_name))
        }
    }

    fn handle_accuse(&mut self, character_name: &str) -> Result<(), String> {
        // Find the character (not necessarily in current location)
        let character_id = self.world.characters.iter()
            .find(|(_, c)| c.matches_name(character_name))
            .map(|(id, _)| id.clone());

        if let Some(char_id) = character_id {
            println!("\n{}", "=".repeat(60));
            println!("You are about to accuse {} of murder.", character_name);
            println!("This decision is FINAL and will end the game.");
            println!("Are you sure? (This is a simplified version - accusation goes through)");
            println!("{}", "=".repeat(60));

            self.accused_character = Some(char_id);
            self.game_over = true;
            Ok(())
        } else {
            Err(format!("You don't know anyone named '{}'.", character_name))
        }
    }

    fn display_memory(&self, memory: &Memory) {
        memory.display();
    }

    fn process_memory_discovery(&mut self, memory: &Memory) {
        // Add evidence from memory
        let evidence_id = format!("evidence_{}", memory.id);
        let evidence_item = crate::evidence::EvidenceItem::new(
            evidence_id,
            format!("Memory: {}", memory.title),
            crate::evidence::EvidenceSource::Memory(memory.id.clone()),
        );
        self.evidence.add_item(evidence_item);

        // Add to timeline
        let event = crate::evidence::TimelineEvent::new(
            memory.timestamp,
            memory.title.clone(),
            memory.location.clone(),
            memory.owner.clone(),
        );
        self.timeline.add_event(event);
    }

    fn check_for_new_evidence_from_focus(&mut self, _memory_id: &str, _detail: &str) {
        // Check if focusing on this detail reveals new evidence or contradictions
        // This would be implemented based on specific game logic
    }

    fn show_inventory(&self) {
        println!("\n=== INVENTORY ===\n");
        if self.inventory.is_empty() {
            println!("You are not carrying anything.");
        } else {
            for obj_id in &self.inventory {
                if let Some(obj) = self.world.get_object(obj_id) {
                    println!("  - {}", obj.name);
                }
            }
        }
    }

    pub fn show_help(&self) {
        println!("\n{}", "=".repeat(60));
        println!("THE MEMORY DETECTIVE - COMMAND REFERENCE");
        println!("{}", "=".repeat(60));
        println!("\nMOVEMENT:");
        println!("  GO [direction] / N/S/E/W/U/D - Move in a direction");
        println!("  LOOK / L                    - Examine your surroundings");
        println!("  EXAMINE [object/person]     - Look at something closely");
        println!("\nINVESTIGATION:");
        println!("  TOUCH [object/person]       - Experience memories");
        println!("  FOCUS ON [detail]           - Examine memory detail closely");
        println!("  COMPARE [mem1] WITH [mem2]  - Find contradictions");
        println!("\nINTERACTION:");
        println!("  TALK TO [person]            - Start conversation");
        println!("  ASK [person] ABOUT [topic]  - Question someone");
        println!("  CONFRONT [person] ABOUT [topic] - Challenge inconsistency");
        println!("  ACCUSE [person]             - Make final accusation");
        println!("\nMANAGEMENT:");
        println!("  INVENTORY / I               - Show carried items");
        println!("  TIMELINE                    - View reconstructed timeline");
        println!("  NOTES / EVIDENCE            - Review evidence & contradictions");
        println!("  HELP / ?                    - Show this help");
        println!("  QUIT / Q                    - Exit game");
        println!("{}", "=".repeat(60));
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn show_ending(&self) {
        println!("\n{}", "=".repeat(60));
        println!("THE END");
        println!("{}", "=".repeat(60));

        if let Some(accused_id) = &self.accused_character {
            if let Some(character) = self.world.get_character(accused_id) {
                println!("\nYou have accused {} of the murder of Lord Edmund Ashford.", character.name);

                // Check if correct
                if accused_id == "charlotte" {
                    println!("\n{} breaks down and confesses.", character.name);
                    println!("She killed her father in a moment of rage after he forbade her marriage.");
                    println!("Your brilliant deduction has solved the case!");
                    println!("\n** PERFECT SOLUTION **");
                } else if accused_id == "james" {
                    println!("\n{} denies the murder but admits to covering it up.", character.name);
                    println!("The real killer goes free. You've made a mistake.");
                    println!("\n** PARTIAL SOLUTION **");
                } else {
                    println!("\nYou were wrong. {} is innocent.", character.name);
                    println!("The real killer escapes justice.");
                    println!("\n** INCORRECT **");
                }
            }
        } else {
            println!("\nThe case remains unsolved.");
        }

        println!("\nMemories discovered: {}/{}",
                 self.memory_graph.discovered.len(),
                 self.memory_graph.memories.len());
        println!("Evidence collected: {}", self.evidence.items.len());
        println!("Contradictions found: {}", self.evidence.contradictions.len());
        println!("{}", "=".repeat(60));
    }

    // This will be a large method to initialize all game content
    fn initialize_world(&mut self) {
        self.create_locations();
        self.create_objects();
        self.create_characters();
    }

    fn create_locations(&mut self) {
        // Create all locations
        let mut foyer = Location::new(
            "foyer".to_string(),
            "Grand Foyer".to_string(),
            "You stand in the grand foyer of Ashford Manor. A magnificent chandelier hangs overhead, casting flickering light on the marble floors. The atmosphere is heavy with tension - everyone knows Lord Edmund has been found dead in his study.".to_string(),
        );
        foyer.add_exit(Direction::North, "drawing_room".to_string());
        foyer.add_exit(Direction::East, "study".to_string());
        foyer.add_exit(Direction::West, "dining_room".to_string());
        foyer.add_exit(Direction::Up, "upper_hall".to_string());
        self.world.add_location(foyer);

        let mut drawing_room = Location::new(
            "drawing_room".to_string(),
            "Drawing Room".to_string(),
            "An elegant drawing room with plush furniture and dark wood paneling. The guests gathered here after dinner, before the body was discovered. An air of suspicion hangs over everything.".to_string(),
        );
        drawing_room.add_exit(Direction::South, "foyer".to_string());
        drawing_room.add_exit(Direction::West, "library".to_string());
        self.world.add_location(drawing_room);

        let mut study = Location::new(
            "study".to_string(),
            "Lord Ashford's Study".to_string(),
            "This is where Lord Edmund's body was found. The room is in disarray - papers scattered, a chair overturned. Blood stains the Persian rug near the desk. The heavy scent of cigars still lingers.".to_string(),
        );
        study.add_exit(Direction::West, "foyer".to_string());
        self.world.add_location(study);

        let mut dining_room = Location::new(
            "dining_room".to_string(),
            "Dining Room".to_string(),
            "A long mahogany table dominates the room, still set with the remnants of dinner. Wine glasses stand half-empty, and plates of cold food remain. The dinner was interrupted by the discovery of the murder.".to_string(),
        );
        dining_room.add_exit(Direction::East, "foyer".to_string());
        dining_room.add_exit(Direction::South, "kitchen".to_string());
        dining_room.add_exit(Direction::West, "conservatory".to_string());
        self.world.add_location(dining_room);

        let mut library = Location::new(
            "library".to_string(),
            "Library".to_string(),
            "Floor-to-ceiling bookshelves line the walls. A fire burns low in the fireplace. Leather chairs invite quiet reading, but tonight they've been witness to darker things.".to_string(),
        );
        library.add_exit(Direction::East, "drawing_room".to_string());
        self.world.add_location(library);

        let mut kitchen = Location::new(
            "kitchen".to_string(),
            "Kitchen".to_string(),
            "A large, functional kitchen where the staff prepared tonight's ill-fated dinner. Pots and pans hang from hooks, and the scent of roasted meat still lingers.".to_string(),
        );
        kitchen.add_exit(Direction::North, "dining_room".to_string());
        self.world.add_location(kitchen);

        let mut conservatory = Location::new(
            "conservatory".to_string(),
            "Conservatory".to_string(),
            "A glass-enclosed room filled with exotic plants. Moonlight filters through the windows, creating strange shadows among the foliage. French doors lead to the garden.".to_string(),
        );
        conservatory.add_exit(Direction::East, "dining_room".to_string());
        conservatory.add_exit(Direction::South, "garden".to_string());
        self.world.add_location(conservatory);

        let mut garden = Location::new(
            "garden".to_string(),
            "Garden".to_string(),
            "The formal garden is dark and misty tonight. Gravel paths wind between topiaries and flower beds. A stone fountain burbles quietly in the center.".to_string(),
        );
        garden.add_exit(Direction::North, "conservatory".to_string());
        self.world.add_location(garden);

        let mut upper_hall = Location::new(
            "upper_hall".to_string(),
            "Upper Hallway".to_string(),
            "The upstairs hallway is carpeted in deep red. Portraits of Ashford ancestors line the walls, their painted eyes seeming to watch your every move. Bedroom doors branch off in several directions.".to_string(),
        );
        upper_hall.add_exit(Direction::Down, "foyer".to_string());
        upper_hall.add_exit(Direction::North, "master_bedroom".to_string());
        upper_hall.add_exit(Direction::East, "charlotte_room".to_string());
        upper_hall.add_exit(Direction::West, "guest_rooms".to_string());
        self.world.add_location(upper_hall);

        let mut master_bedroom = Location::new(
            "master_bedroom".to_string(),
            "Master Bedroom".to_string(),
            "Lord Edmund and Lady Margaret's bedroom is opulently furnished. A four-poster bed dominates the room, and a vanity holds Lady Margaret's jewelry and perfumes.".to_string(),
        );
        master_bedroom.add_exit(Direction::South, "upper_hall".to_string());
        self.world.add_location(master_bedroom);

        let mut charlotte_room = Location::new(
            "charlotte_room".to_string(),
            "Charlotte's Room".to_string(),
            "Charlotte Ashford's bedroom reflects her youth and education. Books are stacked on the nightstand, and a writing desk sits by the window, covered in papers and letters.".to_string(),
        );
        charlotte_room.add_exit(Direction::West, "upper_hall".to_string());
        self.world.add_location(charlotte_room);

        let mut guest_rooms = Location::new(
            "guest_rooms".to_string(),
            "Guest Rooms".to_string(),
            "A hallway of guest bedrooms where tonight's visitors are staying. Each room is neat and well-appointed, though none of the guests seem comfortable staying the night after the murder.".to_string(),
        );
        guest_rooms.add_exit(Direction::East, "upper_hall".to_string());
        self.world.add_location(guest_rooms);
    }

    fn create_objects(&mut self) {
        // Study objects
        let mut candlestick = Object::new(
            "candlestick".to_string(),
            "Brass Candlestick".to_string(),
            "A heavy brass candlestick, ornately decorated. There are traces of blood on its base. This was clearly the murder weapon.".to_string(),
        );
        candlestick.aliases = vec!["weapon".to_string(), "brass".to_string()];
        candlestick.can_touch = true;
        candlestick.is_evidence = true;
        candlestick.associated_memories = vec!["memory_murder".to_string()];
        self.world.add_object(candlestick);
        self.world.get_location_mut("study").unwrap().add_object("candlestick".to_string());

        let mut watch = Object::new(
            "watch".to_string(),
            "Broken Pocket Watch".to_string(),
            "A gold pocket watch, its face cracked. The hands are stopped at 9:47 PM. It must have been broken during the struggle.".to_string(),
        );
        watch.can_touch = true;
        watch.is_evidence = true;
        watch.associated_memories = vec!["memory_struggle".to_string()];
        self.world.add_object(watch);
        self.world.get_location_mut("study").unwrap().add_object("watch".to_string());

        // Charlotte's room objects
        let mut letter = Object::new(
            "letter".to_string(),
            "Unsent Letter".to_string(),
            "A letter addressed to someone named 'William', speaking of forbidden love and desperation. It's dated today, but was never sent.".to_string(),
        );
        letter.aliases = vec!["note".to_string(), "paper".to_string()];
        letter.can_touch = true;
        letter.is_evidence = true;
        letter.associated_memories = vec!["memory_letter_writing".to_string()];
        self.world.add_object(letter);
        self.world.get_location_mut("charlotte_room").unwrap().add_object("letter".to_string());

        // Drawing room objects
        let mut wine_glass = Object::new(
            "glass".to_string(),
            "Wine Glass".to_string(),
            "A crystal wine glass with a few sips of red wine remaining. Someone held this with nervous hands.".to_string(),
        );
        wine_glass.aliases = vec!["wine".to_string(), "crystal".to_string()];
        wine_glass.can_touch = true;
        wine_glass.associated_memories = vec!["memory_toast".to_string()];
        self.world.add_object(wine_glass);
        self.world.get_location_mut("drawing_room").unwrap().add_object("glass".to_string());

        // Library objects
        let mut ledger = Object::new(
            "ledger".to_string(),
            "Business Ledger".to_string(),
            "A leather-bound business ledger with neat columns of figures. Some entries have been crossed out and rewritten.".to_string(),
        );
        ledger.aliases = vec!["book".to_string(), "accounts".to_string()];
        ledger.can_touch = true;
        ledger.is_evidence = true;
        ledger.associated_memories = vec!["memory_embezzlement".to_string()];
        self.world.add_object(ledger);
        self.world.get_location_mut("library").unwrap().add_object("ledger".to_string());

        // Garden objects
        let mut fountain = Object::new(
            "fountain".to_string(),
            "Stone Fountain".to_string(),
            "A decorative fountain with water flowing over carved stone. The area around it shows signs of two people standing close together recently - footprints in the soft earth.".to_string(),
        );
        fountain.can_touch = true;
        fountain.associated_memories = vec!["memory_secret_meeting".to_string()];
        self.world.add_object(fountain);
        self.world.get_location_mut("garden").unwrap().add_object("fountain".to_string());
    }

    fn create_characters(&mut self) {
        // Lady Margaret
        let mut margaret = Character::new(
            "margaret".to_string(),
            "Lady Margaret Ashford".to_string(),
            "Lady Margaret is a striking woman in her mid-forties, dressed in mourning black. Her composure is perfect, but there's something calculating in her eyes.".to_string(),
        );
        margaret.aliases = vec!["lady".to_string(), "wife".to_string(), "margaret".to_string()];
        margaret.can_touch = true;
        margaret.associated_memories = vec!["memory_margaret_alibi".to_string()];
        margaret.add_dialogue(
            "husband".to_string(),
            "My husband was a difficult man. Our marriage was... complicated.".to_string(),
        );
        margaret.add_dialogue(
            "alibi".to_string(),
            "I was in the conservatory at the time. Several guests can confirm it.".to_string(),
        );
        self.world.add_character(margaret);
        self.world.get_location_mut("drawing_room").unwrap().add_character("margaret".to_string());

        // Dr. Blackwood
        let mut thomas = Character::new(
            "thomas".to_string(),
            "Dr. Thomas Blackwood".to_string(),
            "Dr. Blackwood is a portly man in his fifties, Lord Ashford's business partner. He seems nervous, constantly wiping his brow with a handkerchief.".to_string(),
        );
        thomas.aliases = vec!["doctor".to_string(), "blackwood".to_string(), "thomas".to_string()];
        thomas.can_touch = true;
        thomas.associated_memories = vec!["memory_thomas_alibi".to_string()];
        thomas.add_dialogue(
            "business".to_string(),
            "Edmund and I built this company together. I can't believe he's gone.".to_string(),
        );
        thomas.add_dialogue(
            "alibi".to_string(),
            "I was reading in the library. Alone, I'm afraid, so no one can verify it.".to_string(),
        );
        self.world.add_character(thomas);
        self.world.get_location_mut("library").unwrap().add_character("thomas".to_string());

        // Charlotte
        let mut charlotte = Character::new(
            "charlotte".to_string(),
            "Charlotte Ashford".to_string(),
            "Charlotte is Lord Ashford's daughter, a young woman of twenty-three. Her eyes are red from crying, but there's something else there too - anger? Fear?".to_string(),
        );
        charlotte.aliases = vec!["daughter".to_string(), "girl".to_string()];
        charlotte.can_touch = true;
        charlotte.associated_memories = vec!["memory_charlotte_alibi".to_string()];
        charlotte.add_dialogue(
            "father".to_string(),
            "He... he wouldn't let me marry William. Said he wasn't good enough for our family.".to_string(),
        );
        charlotte.add_dialogue(
            "alibi".to_string(),
            "I was in my room, writing letters. I didn't hear anything until someone screamed.".to_string(),
        );
        self.world.add_character(charlotte);
        self.world.get_location_mut("charlotte_room").unwrap().add_character("charlotte".to_string());

        // James (Butler)
        let mut james = Character::new(
            "james".to_string(),
            "James Hartley".to_string(),
            "James is the Ashford family butler, a distinguished man in his late thirties. He moves with quiet efficiency, but seems troubled tonight.".to_string(),
        );
        james.aliases = vec!["butler".to_string(), "hartley".to_string()];
        james.can_touch = true;
        james.associated_memories = vec!["memory_james_alibi".to_string()];
        james.add_dialogue(
            "service".to_string(),
            "I've served this family for fifteen years. Lord Ashford was a fair employer.".to_string(),
        );
        james.add_dialogue(
            "alibi".to_string(),
            "I was serving drinks in the drawing room when the body was discovered.".to_string(),
        );
        self.world.add_character(james);
        self.world.get_location_mut("drawing_room").unwrap().add_character("james".to_string());

        // Victor Crane
        let mut victor = Character::new(
            "victor".to_string(),
            "Victor Crane".to_string(),
            "Victor Crane is a rival industrialist, a heavyset man with a perpetual scowl. He and Lord Ashford had a well-known business feud.".to_string(),
        );
        victor.aliases = vec!["crane".to_string(), "rival".to_string()];
        victor.can_touch = true;
        victor.associated_memories = vec!["memory_victor_alibi".to_string()];
        victor.add_dialogue(
            "edmund".to_string(),
            "Ashford and I had our differences, but I didn't kill him. I'm not a murderer.".to_string(),
        );
        victor.add_dialogue(
            "alibi".to_string(),
            "I was outside, smoking in the garden. Needed fresh air after dinner.".to_string(),
        );
        self.world.add_character(victor);
        self.world.get_location_mut("garden").unwrap().add_character("victor".to_string());

        // Miss Price
        let mut evelyn = Character::new(
            "evelyn".to_string(),
            "Miss Evelyn Price".to_string(),
            "Miss Price is Lord Ashford's secretary, an attractive young woman who seems shaken by the evening's events. She avoids eye contact.".to_string(),
        );
        evelyn.aliases = vec!["secretary".to_string(), "price".to_string(), "miss".to_string()];
        evelyn.can_touch = true;
        evelyn.associated_memories = vec!["memory_evelyn_alibi".to_string()];
        evelyn.add_dialogue(
            "lord ashford".to_string(),
            "He was... he was my employer. Nothing more.".to_string(),
        );
        evelyn.add_dialogue(
            "alibi".to_string(),
            "I was helping in the kitchen. Ask the cook.".to_string(),
        );
        self.world.add_character(evelyn);
        self.world.get_location_mut("kitchen").unwrap().add_character("evelyn".to_string());
    }

    fn initialize_memories(&mut self) {
        use crate::memory::{EmotionalState, Memory, MemoryFragment};

        // Memory 1: The Murder (from candlestick)
        let mut memory_murder = Memory::new(
            "memory_murder".to_string(),
            "charlotte".to_string(),
            107, // 9:47 PM (8:00 PM + 107 minutes)
            "study".to_string(),
        );
        memory_murder.title = "A Moment of Rage".to_string();
        memory_murder.emotional_state = EmotionalState::Angry;
        memory_murder.reliability = 0.4; // Highly emotional, distorted
        memory_murder.content = "\
You are not yourself. You are consumed by fury.\n\n\
A voice - HIS voice - thunders: \"You will NOT marry that boy! I forbid it!\"\n\n\
Hands - are they yours? - grip cold metal. Heavy. The candlestick.\n\n\
\"Father, please! I love him!\"\n\n\
\"You are an ASHFORD! You will marry who I choose!\"\n\n\
The world tilts. Red rage blinds you. The heavy brass arcs through the air.\n\n\
A sickening crack.\n\n\
Silence.\n\n\
He crumples. Blood. So much blood.\n\n\
\"What have I done? Oh God, what have I done?!\"\n\n\
The memory fragments, dissolving into guilt and horror.".to_string();

        let mut fragment1 = MemoryFragment::new(
            "Hands gripping the candlestick".to_string(),
            "hands".to_string(),
        );
        fragment1.focus_content = "You focus on the hands... they're small, delicate. A woman's hands. On the ring finger - a simple gold band with a small ruby. You've seen this ring before...".to_string();
        memory_murder.fragments.push(fragment1);

        let mut fragment2 = MemoryFragment::new(
            "The argument about marriage".to_string(),
            "argument".to_string(),
        );
        fragment2.focus_content = "The argument was about William - Charlotte's forbidden love. Lord Ashford was absolutely against the match. Charlotte was desperate.".to_string();
        memory_murder.fragments.push(fragment2);

        self.memory_graph.add_memory(memory_murder);

        // Memory 2: The broken watch
        let mut memory_struggle = Memory::new(
            "memory_struggle".to_string(),
            "edmund".to_string(),
            107,
            "study".to_string(),
        );
        memory_struggle.title = "The Struggle".to_string();
        memory_struggle.emotional_state = EmotionalState::Fearful;
        memory_struggle.reliability = 0.6;
        memory_struggle.content = "\
Pain. Confusion. Someone you trusted...\n\n\
\"Charlotte, please! Be reasonable!\"\n\n\
She's crying, screaming. You've never seen her like this.\n\n\
You reach for her - your pocket watch catches on something, tears free.\n\n\
Then - impact. Darkness closing in.\n\n\
The last thing you see is her horrified face, the candlestick falling from her hands.\n\n\
[Memory ends as consciousness fades]".to_string();

        let mut fragment3 = MemoryFragment::new(
            "The attacker's face".to_string(),
            "face".to_string(),
        );
        fragment3.focus_content = "Charlotte. Your own daughter. Tears streaming down her face, expression twisted in rage and despair.".to_string();
        memory_struggle.fragments.push(fragment3);

        self.memory_graph.add_memory(memory_struggle);

        // Memory 3: Letter writing
        let mut memory_letter = Memory::new(
            "memory_letter_writing".to_string(),
            "charlotte".to_string(),
            80, // 9:20 PM
            "charlotte_room".to_string(),
        );
        memory_letter.title = "Desperate Words".to_string();
        memory_letter.emotional_state = EmotionalState::Anxious;
        memory_letter.reliability = 0.9;
        memory_letter.content = "\
\"My dearest William,\n\n\
I can bear this no longer. Father refuses to see reason. He will never allow us to marry.\n\n\
I must speak with him tonight. I will make him understand, whatever it takes.\n\n\
If this letter reaches you, know that I love you with all my heart.\n\n\
Yours forever,\n\
Charlotte\"\n\n\
Your hand trembles as you write. You cannot send this - not yet. Not until after you've confronted Father.".to_string();

        self.memory_graph.add_memory(memory_letter);

        // Memory 4: The toast (Victor's veiled threat)
        let mut memory_toast = Memory::new(
            "memory_toast".to_string(),
            "victor".to_string(),
            60, // 9:00 PM
            "drawing_room".to_string(),
        );
        memory_toast.title = "The Toast".to_string();
        memory_toast.emotional_state = EmotionalState::Angry;
        memory_toast.reliability = 0.8;
        memory_toast.content = "\
You raise your glass, wine swirling blood-red in the crystal.\n\n\
\"A toast,\" you announce, your voice carrying an edge. \"To Lord Ashford. May he get... exactly what he deserves.\"\n\n\
The room falls silent. Edmund's face darkens.\n\n\
You smile thinly. \"For all his success, of course.\"\n\n\
But everyone heard the threat.".to_string();

        self.memory_graph.add_memory(memory_toast);

        // Memory 5: Thomas's embezzlement
        let mut memory_embezzlement = Memory::new(
            "memory_embezzlement".to_string(),
            "thomas".to_string(),
            50, // 8:50 PM
            "library".to_string(),
        );
        memory_embezzlement.title = "Falsified Accounts".to_string();
        memory_embezzlement.emotional_state = EmotionalState::Guilty;
        memory_embezzlement.reliability = 0.95;
        memory_embezzlement.content = "\
Your hands shake as you carefully alter the figures in the ledger.\n\n\
Edmund is getting suspicious. He asked to review the books tomorrow.\n\n\
Tomorrow! You need more time!\n\n\
The embezzlement started small - just enough to cover your gambling debts.\n\n\
But it spiraled. Now you've taken thousands. If he discovers this, you'll be ruined. Imprisoned.\n\n\
You close the ledger carefully. Perhaps... perhaps Edmund won't live to review these accounts.".to_string();

        self.memory_graph.add_memory(memory_embezzlement);

        // Memory 6: Secret meeting at fountain
        let mut memory_meeting = Memory::new(
            "memory_secret_meeting".to_string(),
            "james".to_string(),
            100, // 9:40 PM
            "garden".to_string(),
        );
        memory_meeting.title = "Whispered Secrets".to_string();
        memory_meeting.emotional_state = EmotionalState::Anxious;
        memory_meeting.reliability = 0.85;
        memory_meeting.content = "\
\"Margaret, we shouldn't be here,\" you whisper urgently.\n\n\
\"Hush, James. No one can see us.\" Lady Margaret's hand touches yours briefly.\n\n\
\"If the Lord discovers about us—\"\n\n\
\"Edmund knows,\" she interrupts. \"He's known for weeks. He doesn't care - as long as I'm discreet.\"\n\n\
\"But Charlotte—\"\n\n\
\"What about her?\"\n\n\
\"She's going to confront him tonight. About William. I'm worried what might happen.\"\n\n\
Margaret's expression hardens. \"Charlotte is impulsive. We should get back inside.\"";

        let mut fragment4 = MemoryFragment::new(
            "James and Margaret's relationship".to_string(),
            "relationship".to_string(),
        );
        fragment4.focus_content = "James and Margaret are having an affair. It's been going on for months. Lord Ashford knew but tolerated it.".to_string();
        memory_meeting.fragments.push(fragment4);

        let mut fragment5 = MemoryFragment::new(
            "Concern about Charlotte".to_string(),
            "charlotte".to_string(),
        );
        fragment5.focus_content = "James knew Charlotte was going to confront her father. He was worried about it. Did he know what might happen?".to_string();
        memory_meeting.fragments.push(fragment5);

        self.memory_graph.add_memory(memory_meeting);

        // Alibi memories for each character
        self.create_alibi_memories();
    }

    fn create_alibi_memories(&mut self) {
        use crate::memory::{EmotionalState, Memory};

        // Margaret's alibi
        let mut margaret_alibi = Memory::new(
            "memory_margaret_alibi".to_string(),
            "margaret".to_string(),
            105, // 9:45 PM
            "conservatory".to_string(),
        );
        margaret_alibi.title = "In the Conservatory".to_string();
        margaret_alibi.emotional_state = EmotionalState::Calm;
        margaret_alibi.reliability = 0.7;
        margaret_alibi.content = "You stand among the plants, making small talk with Mrs. Henderson. The glass walls fog slightly from your breath. Through the windows, you can see the garden fountain. You're here when the scream echoes through the house.";
        self.memory_graph.add_memory(margaret_alibi);

        // Thomas's alibi
        let mut thomas_alibi = Memory::new(
            "memory_thomas_alibi".to_string(),
            "thomas".to_string(),
            105,
            "library".to_string(),
        );
        thomas_alibi.title = "Alone in the Library".to_string();
        thomas_alibi.emotional_state = EmotionalState::Anxious;
        thomas_alibi.reliability = 0.5; // Suspicious
        thomas_alibi.content = "You sit by the fire, pretending to read. Your mind races. The ledgers. Tomorrow's review. Could you... no. You couldn't. Could you? You hear a cry from down the hall. Time seems to skip. Were you really here the whole time?";
        self.memory_graph.add_memory(thomas_alibi);

        // Charlotte's alibi
        let mut charlotte_alibi = Memory::new(
            "memory_charlotte_alibi".to_string(),
            "charlotte".to_string(),
            90, // Before the murder
            "charlotte_room".to_string(),
        );
        charlotte_alibi.title = "Writing Letters".to_string();
        charlotte_alibi.emotional_state = EmotionalState::Deceptive;
        charlotte_alibi.reliability = 0.3; // She's lying
        charlotte_alibi.content = "You're in your room, writing to William. Yes. That's where you were. You never left. You didn't go downstairs. You didn't confront Father. The blood on your sleeve - that's... that's not there. This memory feels wrong, constructed.";
        self.memory_graph.add_memory(charlotte_alibi);

        // James's alibi
        let mut james_alibi = Memory::new(
            "memory_james_alibi".to_string(),
            "james".to_string(),
            105,
            "drawing_room".to_string(),
        );
        james_alibi.title = "Serving Drinks".to_string();
        james_alibi.emotional_state = EmotionalState::Guilty;
        james_alibi.reliability = 0.6; // Mostly true but hiding something
        james_alibi.content = "You're serving brandy to the guests. Smile. Be professional. Don't think about what you saw. Don't think about Charlotte running from the study, blood on her dress. Don't think about how you helped her change, burn the evidence. Just serve drinks. You were here. You were ALWAYS here.";
        self.memory_graph.add_memory(james_alibi);

        // Victor's alibi
        let mut victor_alibi = Memory::new(
            "memory_victor_alibi".to_string(),
            "victor".to_string(),
            105,
            "garden".to_string(),
        );
        victor_alibi.title = "Smoking Outside".to_string();
        victor_alibi.emotional_state = EmotionalState::Angry;
        victor_alibi.reliability = 0.8;
        victor_alibi.content = "You stand in the cold garden, cigar smoke curling into the night. Damn Ashford. Damn his business tactics. But you didn't kill him. You wanted to, but you didn't. You hear shouting from inside - a woman's voice. Then silence.";
        self.memory_graph.add_memory(victor_alibi);

        // Evelyn's alibi
        let mut evelyn_alibi = Memory::new(
            "memory_evelyn_alibi".to_string(),
            "evelyn".to_string(),
            105,
            "kitchen".to_string(),
        );
        evelyn_alibi.title = "Helping in the Kitchen".to_string();
        evelyn_alibi.emotional_state = EmotionalState::Anxious;
        evelyn_alibi.reliability = 0.9; // She's telling the truth
        evelyn_alibi.content = "You're arranging desserts on serving plates, trying not to think about this afternoon. Lord Ashford's advances. Your rejection. His cruel laughter. 'You're dismissed, Miss Price. Pack your things tomorrow.' But you didn't kill him. You were here. The cook can confirm it.";
        self.memory_graph.add_memory(evelyn_alibi);
    }
}
