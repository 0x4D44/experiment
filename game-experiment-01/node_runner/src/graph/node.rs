use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Start,
    Combat,
    Treasure,
    Rest,
    Shop,
    Event,
    Boss,
}

impl NodeType {
    pub fn symbol(&self) -> char {
        match self {
            NodeType::Start => 'S',
            NodeType::Combat => 'C',
            NodeType::Treasure => 'T',
            NodeType::Rest => 'R',
            NodeType::Shop => '$',
            NodeType::Event => '?',
            NodeType::Boss => 'B',
        }
    }

    pub fn description(&self) -> &str {
        match self {
            NodeType::Start => "Entrance",
            NodeType::Combat => "Combat",
            NodeType::Treasure => "Treasure",
            NodeType::Rest => "Rest Site",
            NodeType::Shop => "Shop",
            NodeType::Event => "Event",
            NodeType::Boss => "Boss",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: usize,
    pub node_type: NodeType,
    pub depth: usize,
    pub visited: bool,
}

impl Node {
    pub fn new(id: usize, node_type: NodeType, depth: usize) -> Self {
        Self {
            id,
            node_type,
            depth,
            visited: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new(0, NodeType::Start, 0);
        assert_eq!(node.id, 0);
        assert_eq!(node.node_type, NodeType::Start);
        assert_eq!(node.depth, 0);
        assert!(!node.visited);
    }

    #[test]
    fn test_node_type_symbols() {
        assert_eq!(NodeType::Start.symbol(), 'S');
        assert_eq!(NodeType::Combat.symbol(), 'C');
        assert_eq!(NodeType::Treasure.symbol(), 'T');
        assert_eq!(NodeType::Rest.symbol(), 'R');
        assert_eq!(NodeType::Shop.symbol(), '$');
        assert_eq!(NodeType::Event.symbol(), '?');
        assert_eq!(NodeType::Boss.symbol(), 'B');
    }

    #[test]
    fn test_node_type_descriptions() {
        assert_eq!(NodeType::Start.description(), "Entrance");
        assert_eq!(NodeType::Combat.description(), "Combat");
        assert_eq!(NodeType::Treasure.description(), "Treasure");
        assert_eq!(NodeType::Rest.description(), "Rest Site");
        assert_eq!(NodeType::Shop.description(), "Shop");
        assert_eq!(NodeType::Event.description(), "Event");
        assert_eq!(NodeType::Boss.description(), "Boss");
    }
}
