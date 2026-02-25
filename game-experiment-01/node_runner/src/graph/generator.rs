use petgraph::graph::{DiGraph, NodeIndex};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::{HashMap, VecDeque};

use super::node::{Node, NodeType};

pub struct DungeonGraph {
    pub graph: DiGraph<Node, ()>,
    pub start_node: NodeIndex,
    pub boss_node: NodeIndex,
    pub node_map: HashMap<usize, NodeIndex>,
}

pub struct DungeonGenerator {
    rng: StdRng,
    min_nodes: usize,
    max_nodes: usize,
    branch_probability: f64,
}

impl DungeonGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            min_nodes: 25,
            max_nodes: 40,
            branch_probability: 0.3,
        }
    }

    pub fn with_config(seed: u64, min_nodes: usize, max_nodes: usize, branch_probability: f64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            min_nodes,
            max_nodes,
            branch_probability,
        }
    }

    pub fn generate(&mut self) -> DungeonGraph {
        let target_nodes = self.rng.gen_range(self.min_nodes..=self.max_nodes);

        let mut graph = DiGraph::new();
        let mut node_map = HashMap::new();

        // Create start node
        let start_node_data = Node::new(0, NodeType::Start, 0);
        let start_node = graph.add_node(start_node_data);
        node_map.insert(0, start_node);

        // Generate main path using depth-first approach
        let mut current_id = 1;
        let mut current_idx = start_node;
        let mut current_depth = 0;
        let mut nodes_to_branch_from = Vec::new();

        // Build main spine
        let spine_length = target_nodes * 2 / 3; // Main path is 2/3 of nodes
        for _ in 0..spine_length {
            let node_data = Node::new(current_id, NodeType::Combat, current_depth + 1);
            let new_idx = graph.add_node(node_data);
            node_map.insert(current_id, new_idx);
            graph.add_edge(current_idx, new_idx, ());

            // Collect nodes for branching
            if self.rng.gen::<f64>() < self.branch_probability {
                nodes_to_branch_from.push((new_idx, current_depth + 1));
            }

            current_idx = new_idx;
            current_id += 1;
            current_depth += 1;
        }

        // Boss node at the end of main path
        let boss_node_data = Node::new(current_id, NodeType::Boss, current_depth + 1);
        let boss_node = graph.add_node(boss_node_data);
        node_map.insert(current_id, boss_node);
        graph.add_edge(current_idx, boss_node, ());
        current_id += 1;

        // Add branches
        while current_id < target_nodes && !nodes_to_branch_from.is_empty() {
            let branch_idx = self.rng.gen_range(0..nodes_to_branch_from.len());
            let (branch_source, branch_depth) = nodes_to_branch_from[branch_idx];

            // Create a short branch (1-3 nodes)
            let branch_length = self.rng.gen_range(1..=3.min(target_nodes - current_id));
            let mut branch_current = branch_source;

            for i in 0..branch_length {
                let node_data = Node::new(current_id, NodeType::Combat, branch_depth + i + 1);
                let new_idx = graph.add_node(node_data);
                node_map.insert(current_id, new_idx);
                graph.add_edge(branch_current, new_idx, ());

                branch_current = new_idx;
                current_id += 1;

                if current_id >= target_nodes {
                    break;
                }
            }

            // Maybe connect branch back to main path
            if self.rng.gen::<f64>() < 0.3 {
                let potential_targets: Vec<_> = graph
                    .node_indices()
                    .filter(|&idx| {
                        let node = &graph[idx];
                        node.depth > graph[branch_current].depth &&
                        node.node_type != NodeType::Boss &&
                        idx != branch_current
                    })
                    .collect();

                if !potential_targets.is_empty() {
                    let target_idx = self.rng.gen_range(0..potential_targets.len());
                    let target = potential_targets[target_idx];
                    graph.add_edge(branch_current, target, ());
                }
            }

            nodes_to_branch_from.remove(branch_idx);
        }

        // Assign node types based on depth and position
        self.assign_node_types(&mut graph, boss_node);

        DungeonGraph {
            graph,
            start_node,
            boss_node,
            node_map,
        }
    }

    fn assign_node_types(&mut self, graph: &mut DiGraph<Node, ()>, boss_node: NodeIndex) {
        let node_indices: Vec<_> = graph.node_indices().collect();

        for idx in node_indices {
            if graph[idx].node_type == NodeType::Start || graph[idx].node_type == NodeType::Boss {
                continue;
            }

            let depth = graph[idx].depth;
            let node_id = graph[idx].id;

            // Assign types based on depth and randomness
            let node_type = if depth <= 3 {
                // Early nodes: mostly combat
                if self.rng.gen::<f64>() < 0.1 {
                    NodeType::Treasure
                } else {
                    NodeType::Combat
                }
            } else if depth <= 8 {
                // Mid nodes: mix of types
                let roll = self.rng.gen::<f64>();
                if roll < 0.6 {
                    NodeType::Combat
                } else if roll < 0.75 {
                    NodeType::Treasure
                } else if roll < 0.85 {
                    NodeType::Rest
                } else if roll < 0.95 {
                    NodeType::Shop
                } else {
                    NodeType::Event
                }
            } else {
                // Late nodes: more variety
                let roll = self.rng.gen::<f64>();
                if roll < 0.5 {
                    NodeType::Combat
                } else if roll < 0.65 {
                    NodeType::Treasure
                } else if roll < 0.75 {
                    NodeType::Rest
                } else if roll < 0.85 {
                    NodeType::Shop
                } else {
                    NodeType::Event
                }
            };

            graph[idx].node_type = node_type;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_creates_graph_with_correct_node_count() {
        let mut gen = DungeonGenerator::with_config(42, 10, 15, 0.3);
        let dungeon = gen.generate();

        let node_count = dungeon.graph.node_count();
        assert!(node_count >= 10 && node_count <= 15);
    }

    #[test]
    fn test_generator_has_start_and_boss_nodes() {
        let mut gen = DungeonGenerator::new(42);
        let dungeon = gen.generate();

        assert_eq!(dungeon.graph[dungeon.start_node].node_type, NodeType::Start);
        assert_eq!(dungeon.graph[dungeon.boss_node].node_type, NodeType::Boss);
    }

    #[test]
    fn test_start_node_has_depth_zero() {
        let mut gen = DungeonGenerator::new(42);
        let dungeon = gen.generate();

        assert_eq!(dungeon.graph[dungeon.start_node].depth, 0);
    }

    #[test]
    fn test_boss_is_reachable_from_start() {
        let mut gen = DungeonGenerator::new(42);
        let dungeon = gen.generate();

        // BFS from start to check if boss is reachable
        let mut visited = std::collections::HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(dungeon.start_node);
        visited.insert(dungeon.start_node);

        let mut found_boss = false;
        while let Some(current) = queue.pop_front() {
            if current == dungeon.boss_node {
                found_boss = true;
                break;
            }

            for neighbor in dungeon.graph.neighbors(current) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        assert!(found_boss, "Boss node should be reachable from start node");
    }

    #[test]
    fn test_all_nodes_have_unique_ids() {
        let mut gen = DungeonGenerator::new(42);
        let dungeon = gen.generate();

        let mut ids = std::collections::HashSet::new();
        for idx in dungeon.graph.node_indices() {
            let id = dungeon.graph[idx].id;
            assert!(!ids.contains(&id), "Duplicate node ID found: {}", id);
            ids.insert(id);
        }
    }

    #[test]
    fn test_depths_increase_along_paths() {
        let mut gen = DungeonGenerator::new(42);
        let dungeon = gen.generate();

        // Check that edges generally go from lower to higher or equal depth
        for edge in dungeon.graph.edge_indices() {
            let (source, target) = dungeon.graph.edge_endpoints(edge).unwrap();
            let source_depth = dungeon.graph[source].depth;
            let target_depth = dungeon.graph[target].depth;

            // Target should be at same or greater depth
            assert!(target_depth >= source_depth,
                "Edge goes backwards in depth: {} -> {}", source_depth, target_depth);
        }
    }

    #[test]
    fn test_deterministic_generation_with_same_seed() {
        let mut gen1 = DungeonGenerator::new(12345);
        let dungeon1 = gen1.generate();

        let mut gen2 = DungeonGenerator::new(12345);
        let dungeon2 = gen2.generate();

        assert_eq!(dungeon1.graph.node_count(), dungeon2.graph.node_count());
        assert_eq!(dungeon1.graph.edge_count(), dungeon2.graph.edge_count());
    }

    #[test]
    fn test_different_seeds_produce_different_graphs() {
        let mut gen1 = DungeonGenerator::new(1);
        let dungeon1 = gen1.generate();

        let mut gen2 = DungeonGenerator::new(2);
        let dungeon2 = gen2.generate();

        // Graphs might have same node count by chance, but structure should differ
        let node_count_different = dungeon1.graph.node_count() != dungeon2.graph.node_count();
        let edge_count_different = dungeon1.graph.edge_count() != dungeon2.graph.edge_count();

        // At least one should be different (high probability)
        assert!(node_count_different || edge_count_different);
    }
}
