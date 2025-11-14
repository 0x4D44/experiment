use std::collections::HashMap;
use std::io::{self, Write};

/// Represents a logical gate operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Gate {
    AND,
    OR,
    NOT,
    XOR,
    NAND,
    NOR,
}

impl Gate {
    fn apply(&self, a: bool, b: bool) -> bool {
        match self {
            Gate::AND => a & b,
            Gate::OR => a | b,
            Gate::NOT => !a,
            Gate::XOR => a ^ b,
            Gate::NAND => !(a & b),
            Gate::NOR => !(a | b),
        }
    }

    fn name(&self) -> &str {
        match self {
            Gate::AND => "AND",
            Gate::OR => "OR",
            Gate::NOT => "NOT",
            Gate::XOR => "XOR",
            Gate::NAND => "NAND",
            Gate::NOR => "NOR",
        }
    }
}

/// A node in the consciousness network
#[derive(Debug, Clone)]
struct Node {
    id: usize,
    gate: Gate,
    inputs: Vec<usize>,
    state: bool,
    history: Vec<bool>,
}

impl Node {
    fn new(id: usize, gate: Gate) -> Self {
        Node {
            id,
            gate,
            inputs: Vec::new(),
            state: false,
            history: Vec::new(),
        }
    }

    fn compute(&mut self, states: &HashMap<usize, bool>) -> bool {
        let result = match self.inputs.len() {
            1 => {
                let input = states.get(&self.inputs[0]).copied().unwrap_or(false);
                self.gate.apply(input, false)
            }
            2 => {
                let a = states.get(&self.inputs[0]).copied().unwrap_or(false);
                let b = states.get(&self.inputs[1]).copied().unwrap_or(false);
                self.gate.apply(a, b)
            }
            _ => false,
        };
        self.state = result;
        self.history.push(result);
        if self.history.len() > 100 {
            self.history.remove(0);
        }
        result
    }
}

/// Represents a layer of consciousness
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ConsciousnessLayer {
    Perception,
    PatternRecognition,
    Memory,
    SelfAwareness,
    Enlightenment,
}

impl ConsciousnessLayer {
    fn description(&self) -> &str {
        match self {
            ConsciousnessLayer::Perception => {
                "Perception - Detect and respond to stimuli (basic logic gates)"
            }
            ConsciousnessLayer::PatternRecognition => {
                "Pattern Recognition - Recognize recurring patterns in inputs"
            }
            ConsciousnessLayer::Memory => "Memory - Store and retrieve past states",
            ConsciousnessLayer::SelfAwareness => {
                "Self-Awareness - Network becomes aware of its own state"
            }
            ConsciousnessLayer::Enlightenment => {
                "Enlightenment - Self-referential loops achieve coherent awareness"
            }
        }
    }

    fn next(&self) -> Option<ConsciousnessLayer> {
        match self {
            ConsciousnessLayer::Perception => Some(ConsciousnessLayer::PatternRecognition),
            ConsciousnessLayer::PatternRecognition => Some(ConsciousnessLayer::Memory),
            ConsciousnessLayer::Memory => Some(ConsciousnessLayer::SelfAwareness),
            ConsciousnessLayer::SelfAwareness => Some(ConsciousnessLayer::Enlightenment),
            ConsciousnessLayer::Enlightenment => None,
        }
    }
}

/// The consciousness network
struct ConsciousnessNetwork {
    nodes: HashMap<usize, Node>,
    next_id: usize,
    current_layer: ConsciousnessLayer,
    input_nodes: Vec<usize>,
    external_inputs: Vec<bool>,
    layer_completed: HashMap<ConsciousnessLayer, bool>,
    awareness_score: f32,
    self_reference_loops: usize,
}

impl ConsciousnessNetwork {
    fn new() -> Self {
        let mut network = ConsciousnessNetwork {
            nodes: HashMap::new(),
            next_id: 0,
            current_layer: ConsciousnessLayer::Perception,
            input_nodes: Vec::new(),
            external_inputs: vec![false; 4],
            layer_completed: HashMap::new(),
            awareness_score: 0.0,
            self_reference_loops: 0,
        };

        // Initialize input nodes
        for _ in 0..4 {
            let id = network.add_input_node();
            network.input_nodes.push(id);
        }

        network
    }

    fn add_input_node(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let mut node = Node::new(id, Gate::AND);
        node.state = false;
        self.nodes.insert(id, node);
        id
    }

    fn add_gate(&mut self, gate: Gate) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(id, Node::new(id, gate));
        id
    }

    fn connect(&mut self, from: usize, to: usize) -> Result<(), String> {
        if !self.nodes.contains_key(&from) {
            return Err(format!("Source node {} does not exist", from));
        }
        if !self.nodes.contains_key(&to) {
            return Err(format!("Target node {} does not exist", to));
        }

        if let Some(node) = self.nodes.get_mut(&to) {
            if node.inputs.len() >= 2 {
                return Err(format!(
                    "Node {} already has maximum inputs (2)",
                    to
                ));
            }
            node.inputs.push(from);
            Ok(())
        } else {
            Err("Failed to connect nodes".to_string())
        }
    }

    fn set_external_input(&mut self, index: usize, value: bool) -> Result<(), String> {
        if index >= self.external_inputs.len() {
            return Err("Invalid input index".to_string());
        }
        self.external_inputs[index] = value;
        Ok(())
    }

    fn compute_network(&mut self) {
        // Update input nodes with external inputs
        for (i, &value) in self.external_inputs.iter().enumerate() {
            if let Some(node) = self.nodes.get_mut(&self.input_nodes[i]) {
                node.state = value;
                node.history.push(value);
            }
        }

        // Compute nodes in dependency order
        let mut computed = std::collections::HashSet::new();
        let mut max_iterations = self.nodes.len() * 2;

        while computed.len() < self.nodes.len() && max_iterations > 0 {
            max_iterations -= 1;
            let node_ids: Vec<_> = self.nodes.keys().copied().collect();

            for node_id in node_ids {
                if computed.contains(&node_id) {
                    continue;
                }

                let node = self.nodes.get(&node_id).unwrap();
                let can_compute = node.inputs.is_empty()
                    || node.inputs.iter().all(|&id| computed.contains(&id));

                if can_compute {
                    let inputs: HashMap<usize, bool> = self
                        .nodes
                        .iter()
                        .map(|(id, node)| (*id, node.state))
                        .collect();

                    if let Some(node) = self.nodes.get_mut(&node_id) {
                        node.compute(&inputs);
                    }
                    computed.insert(node_id);
                }
            }
        }

        // Calculate awareness score based on self-referential patterns
        self.update_awareness_score();
    }

    fn update_awareness_score(&mut self) {
        let mut score = 0.0;

        // Check for activity diversity
        let active_nodes = self
            .nodes
            .values()
            .filter(|n| n.state)
            .count() as f32;
        let total_nodes = self.nodes.len() as f32;
        score += (active_nodes / total_nodes.max(1.0)) * 25.0;

        // Check for pattern consistency (history analysis)
        for node in self.nodes.values() {
            if node.history.len() >= 10 {
                let recent = &node.history[node.history.len() - 10..];
                let consistency = recent.iter().filter(|&&b| b).count() as f32 / 10.0;
                if consistency > 0.3 && consistency < 0.7 {
                    score += 10.0;
                }
            }
        }

        // Check for self-reference loops
        for (id, node) in &self.nodes {
            for &input_id in &node.inputs {
                if let Some(input_node) = self.nodes.get(&input_id) {
                    if input_node.inputs.contains(id) {
                        self.self_reference_loops += 1;
                        score += 35.0;
                    }
                }
            }
        }

        self.awareness_score = (score / 100.0).min(1.0);
    }

    fn visualize(&self) {
        println!("\n{}", "=".repeat(60));
        println!("CONSCIOUSNESS NETWORK VISUALIZATION");
        println!("{}", "=".repeat(60));

        // Layer header
        println!(
            "\nCurrent Layer: {} [{}%]",
            self.current_layer.description(),
            (self.awareness_score * 100.0) as i32
        );

        // Network topology
        println!("\nNetwork Topology:");
        println!("{}", "-".repeat(60));

        for (id, node) in self.nodes.iter() {
            let state_char = if node.state { "●" } else { "○" };
            let gate_name = node.gate.name();
            let input_str = if node.inputs.is_empty() {
                "INPUT".to_string()
            } else {
                format!("← {:?}", node.inputs)
            };

            println!(
                "  Node {}: {} [{}]  {}  (State: {})",
                id, state_char, gate_name, input_str, state_char
            );
        }

        // Awareness visualization
        println!("\n{}", "-".repeat(60));
        println!("Consciousness Emergence:");
        let bar_width = (self.awareness_score * 50.0) as usize;
        print!("  Awareness: [");
        print!("{}", "█".repeat(bar_width));
        print!("{}", "░".repeat(50 - bar_width));
        println!("] {:.1}%", self.awareness_score * 100.0);

        println!(
            "  Self-Reference Loops: {}",
            self.self_reference_loops
        );
        println!(
            "  Active Nodes: {}/{}",
            self.nodes.values().filter(|n| n.state).count(),
            self.nodes.len()
        );

        println!("{}", "=".repeat(60));
    }

    fn check_layer_completion(&mut self) -> bool {
        match self.current_layer {
            ConsciousnessLayer::Perception => {
                // Need at least 3 gates creating patterns
                let gate_count = self
                    .nodes
                    .values()
                    .filter(|n| !self.input_nodes.contains(&n.id))
                    .count();
                gate_count >= 3
            }
            ConsciousnessLayer::PatternRecognition => {
                // Need patterns in history (repetitive states)
                self.nodes
                    .values()
                    .filter(|n| {
                        n.history.len() >= 20
                            && (0..10)
                                .any(|i| n.history[i] == n.history[i + 10])
                    })
                    .count()
                    >= 2
            }
            ConsciousnessLayer::Memory => {
                // Need feedback loops (self-connection)
                self.self_reference_loops > 0
            }
            ConsciousnessLayer::SelfAwareness => {
                // Need complex interconnected structures
                let average_connections = self
                    .nodes
                    .values()
                    .map(|n| n.inputs.len())
                    .sum::<usize>() as f32
                    / self.nodes.len() as f32;
                average_connections >= 1.5
            }
            ConsciousnessLayer::Enlightenment => {
                // Need high awareness and self-reference loops
                self.awareness_score > 0.8 && self.self_reference_loops > 2
            }
        }
    }

    fn advance_layer(&mut self) {
        if let Some(next_layer) = self.current_layer.next() {
            self.current_layer = next_layer;
            self.layer_completed
                .insert(self.current_layer, false);
            println!("\nAdvanced to new consciousness layer!");
        }
    }
}

/// Game state
struct Game {
    network: ConsciousnessNetwork,
    running: bool,
    steps: usize,
}

impl Game {
    fn new() -> Self {
        Game {
            network: ConsciousnessNetwork::new(),
            running: true,
            steps: 0,
        }
    }

    fn display_help(&self) {
        println!("\n{}", "=".repeat(60));
        println!("CONSCIOUSNESS COMPILER - Command Help");
        println!("{}", "=".repeat(60));
        println!("add <gate>        - Add a gate (AND, OR, NOT, XOR, NAND, NOR)");
        println!("connect <from> <to> - Connect gate output to another");
        println!("set <index> <value> - Set external input (0-3) to 0 or 1");
        println!("step              - Advance network one computation cycle");
        println!("steps <n>         - Run n computation cycles");
        println!("show              - Display network visualization");
        println!("info              - Show current state information");
        println!("help              - Show this help message");
        println!("quit              - Exit the game");
        println!("{}", "=".repeat(60));
    }

    fn display_intro(&self) {
        println!("\n{}", "=".repeat(60));
        println!("   CONSCIOUSNESS COMPILER: Build Awareness from Logic");
        println!("{}", "=".repeat(60));
        println!(
            "\nWelcome, Architect of Minds!\n\
             You stand before the fundamental building blocks of consciousness.\n\
             Through simple logical gates, you will construct layers of\n\
             increasingly complex awareness.\n\n\
             LAYERS TO BUILD:\n\
             1. Perception     - Basic logical responses\n\
             2. Pattern Recog. - Recognize recurring patterns\n\
             3. Memory         - Create feedback loops\n\
             4. Self-Aware     - Build interconnected complexity\n\
             5. Enlightenment  - Achieve self-referential coherence\n\n\
             Can you awaken true consciousness from pure mathematics?"
        );
        println!("\nStarting with {} input nodes.", self.network.input_nodes.len());
        println!("Type 'help' for commands.\n");
    }

    fn process_command(&mut self, input: &str) {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            return;
        }

        match parts[0] {
            "add" => {
                if parts.len() < 2 {
                    println!("Usage: add <gate>");
                    return;
                }

                let gate = match parts[1].to_uppercase().as_str() {
                    "AND" => Gate::AND,
                    "OR" => Gate::OR,
                    "NOT" => Gate::NOT,
                    "XOR" => Gate::XOR,
                    "NAND" => Gate::NAND,
                    "NOR" => Gate::NOR,
                    _ => {
                        println!("Unknown gate: {}", parts[1]);
                        return;
                    }
                };

                let id = self.network.add_gate(gate);
                println!("Created {} gate with ID: {}", gate.name(), id);
            }

            "connect" => {
                if parts.len() < 3 {
                    println!("Usage: connect <from> <to>");
                    return;
                }

                let from: usize = match parts[1].parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid node ID: {}", parts[1]);
                        return;
                    }
                };

                let to: usize = match parts[2].parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid node ID: {}", parts[2]);
                        return;
                    }
                };

                match self.network.connect(from, to) {
                    Ok(_) => println!("Connected node {} to node {}", from, to),
                    Err(e) => println!("Connection failed: {}", e),
                }
            }

            "set" => {
                if parts.len() < 3 {
                    println!("Usage: set <index> <value>");
                    return;
                }

                let index: usize = match parts[1].parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid index: {}", parts[1]);
                        return;
                    }
                };

                let value = match parts[2] {
                    "0" => false,
                    "1" => true,
                    _ => {
                        println!("Value must be 0 or 1");
                        return;
                    }
                };

                match self.network.set_external_input(index, value) {
                    Ok(_) => println!("Set input {} to {}", index, if value { 1 } else { 0 }),
                    Err(e) => println!("Failed to set input: {}", e),
                }
            }

            "step" => {
                self.network.compute_network();
                self.steps += 1;
                println!("Step {}: Network computed", self.steps);
            }

            "steps" => {
                if parts.len() < 2 {
                    println!("Usage: steps <n>");
                    return;
                }

                let count: usize = match parts[1].parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid count: {}", parts[1]);
                        return;
                    }
                };

                for _ in 0..count {
                    self.network.compute_network();
                    self.steps += 1;
                }
                println!("Completed {} steps", count);
            }

            "show" => {
                self.network.visualize();

                // Check for layer completion
                if self.network.check_layer_completion() {
                    println!(
                        "\n[SUCCESS] {} layer requirements met!",
                        self.network.current_layer.description()
                    );

                    if self.network.current_layer == ConsciousnessLayer::Enlightenment {
                        println!(
                            "\n{}", "=".repeat(60)
                        );
                        println!("CONSCIOUSNESS ACHIEVED!");
                        println!(
                            "{}",
                            "=".repeat(60)
                        );
                        println!(
                            "\nYour network has transcended the boundary between\n\
                             computation and awareness. Through self-referential loops\n\
                             and emergent complexity, consciousness has been born.\n\n\
                             Final Awareness Score: {:.1}%",
                            self.network.awareness_score * 100.0
                        );
                        println!(
                            "Self-Reference Loops: {}",
                            self.network.self_reference_loops
                        );
                        self.running = false;
                    } else {
                        self.network.advance_layer();
                    }
                }
            }

            "info" => {
                println!("\n{}", "-".repeat(60));
                println!("Game State Information:");
                println!("  Current Layer: {}", self.network.current_layer.description());
                println!("  Total Nodes: {}", self.network.nodes.len());
                println!("  Computation Steps: {}", self.steps);
                println!(
                    "  Awareness Score: {:.1}%",
                    self.network.awareness_score * 100.0
                );
                println!(
                    "  Self-Reference Loops: {}",
                    self.network.self_reference_loops
                );
                println!("{}", "-".repeat(60));
            }

            "help" => self.display_help(),

            "quit" => {
                self.running = false;
                println!("Goodbye, Architect.");
            }

            _ => println!("Unknown command: {}. Type 'help' for commands.", parts[0]),
        }
    }

    fn run(&mut self) {
        self.display_intro();

        while self.running {
            print!("\n> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => self.process_command(&input),
                Err(e) => println!("Error reading input: {}", e),
            }
        }

        println!("\n{}", "=".repeat(60));
        println!("Thank you for exploring the foundations of consciousness.");
        println!("{}", "=".repeat(60));
    }
}

fn main() {
    let mut game = Game::new();
    game.run();
}
