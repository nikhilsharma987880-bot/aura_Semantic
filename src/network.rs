use crate::node::SymbolNode;
use std::collections::HashMap;
use std::fs;
use std::thread;
use std::time::Duration;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SemanticNetwork {
    pub nodes: HashMap<String, SymbolNode>,
}

impl SemanticNetwork {
    pub fn new() -> Self {
        SemanticNetwork {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: SymbolNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string_pretty(self)?;
        fs::write(filename, serialized)?;
        println!("[*] Network memory successfully persisted to '{}'", filename);
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Option<Self> {
        if let Ok(data) = fs::read_to_string(filename) {
            if let Ok(net) = serde_json::from_str(&data) {
                println!("[*] Network memory successfully restored from '{}'", filename);
                return Some(net);
            }
        }
        None
    }

    pub fn inspect_fabric(&self) {
        println!("[*] Inspecting Semantic Network Fabric...");
        println!("[*] Active Network Nodes count: {}", self.nodes.len());
        for (id, node) in &self.nodes {
            println!("    Node: {} -> Connections: {}", id, node.connections.len());
        }
    }

    pub fn start_autonomous_heartbeat(&mut self, cycles: usize) {
        println!("[*] Starting autonomous heartbeat execution for {} cycles...", cycles);
        for i in 1..=cycles {
            println!("[Heartbeat] Cycle {}/{} running cognitive rhythm...", i, cycles);
            self.decay_and_prune();
        }
    }

    // --- कॉग्निटिव डीके और प्रूनिंग ---
    pub fn decay_and_prune(&mut self) {
        let threshold = 0.15;
        let mut edges_removed = 0;
        let mut nodes_to_remove = Vec::new();

        for (id, node) in self.nodes.iter_mut() {
            node.connections.retain(|_, weight| {
                *weight -= 0.02;
                if *weight < threshold {
                    edges_removed += 1;
                    false
                } else {
                    true
                }
            });

            if (id.starts_with("Autopoietic_Concept") || id.starts_with("Hybrid_Concept")) && node.connections.is_empty() {
                nodes_to_remove.push(id.clone());
            }
        }

        for dead_id in nodes_to_remove {
            println!("  [Pruning] Removing stale node: '{}'", dead_id);
            self.nodes.remove(&dead_id);
        }

        if edges_removed > 0 {
            println!("  [Decay] Cleaned up {} weak connections.", edges_removed);
        }
    }

    // --- ऑटोनॉमस कॉन्सेप्ट सिंथेसिस (Hybrid Concept Generation) ---
    pub fn synthesize_concepts(&mut self, memory_file: &str) -> bool {
        let mut new_nodes_to_add = Vec::new();
        let threshold = 0.75;

        let node_items: Vec<(String, SymbolNode)> = self.nodes.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

        for (parent_id, node) in &node_items {
            for (target_id, weight) in &node.connections {
                if *weight >= threshold {
                    if let Some(target_node) = self.nodes.get(target_id) {
                        if target_node.connections.contains_key(parent_id) {
                            let clean_parent = parent_id.split('(').next().unwrap_or(parent_id).trim();
                            let clean_target = target_id.split('(').next().unwrap_or(target_id).trim();
                            
                            let hybrid_id = format!("Hybrid_Concept_{}And{}", clean_parent, clean_target);
                            
                            if !self.nodes.contains_key(&hybrid_id) {
                                println!("    ✨ [Autonomous Synthesis] Merging '{}' & '{}' -> Spawning Hybrid: '{}'", clean_parent, clean_target, hybrid_id);
                                let mut hybrid_node = SymbolNode::new(&hybrid_id);
                                hybrid_node.link(parent_id, 0.90);
                                hybrid_node.link(target_id, 0.90);
                                new_nodes_to_add.push(hybrid_node);
                            }
                        }
                    }
                }
            }
        }

        let added = !new_nodes_to_add.is_empty();
        for node in new_nodes_to_add {
            self.nodes.insert(node.id.clone(), node);
        }

        if added {
            let _ = self.save_to_file(memory_file);
        }
        added
    }

    pub fn find_and_evolve(&mut self, input_pattern: &SymbolNode, memory_file: &str) {
        let mut best_match: Option<String> = None;
        let mut max_resonance = 0.0;

        for (id, existing_node) in &self.nodes {
            let mut current_resonance = 0.0;
            for (target, weight) in &input_pattern.connections {
                if let Some(existing_weight) = existing_node.connections.get(target) {
                    current_resonance += weight * existing_weight;
                }
            }
            if current_resonance > max_resonance {
                max_resonance = current_resonance;
                best_match = Some(id.clone());
            }
        }

        if let Some(ref matched_id) = best_match {
            println!("  [Resonance] Match found: '{}' (Score: {:.2})", matched_id, max_resonance);
            let mut nodes_to_spawn = Vec::new();

            if let Some(node) = self.nodes.get_mut(matched_id) {
                for (target, weight) in &input_pattern.connections {
                    let current_w = node.connections.get(target).unwrap_or(&0.0);
                    let new_w = (current_w + 0.2).min(1.0);
                    node.connections.insert(target.clone(), new_w);

                    if new_w >= 0.8 {
                        let spawned_id = format!("Autopoietic_Concept_{}", target);
                        nodes_to_spawn.push((spawned_id, target.clone()));
                    }
                }
            }

            let mut structural_changed = false;
            for (spawned_id, target) in nodes_to_spawn {
                if !self.nodes.contains_key(&spawned_id) {
                    println!("    ├── [Dynamic Autopoiesis] Spawning spontaneous node -> '{}'", spawned_id);
                    let mut new_child = SymbolNode::new(&spawned_id);
                    new_child.link(&target, 0.85);
                    new_child.link(matched_id, 0.70);
                    self.nodes.insert(spawned_id, new_child);
                    structural_changed = true;
                }
            }

            if self.synthesize_concepts(memory_file) {
                structural_changed = true;
            }

            if structural_changed {
                let _ = self.save_to_file(memory_file);
            }
        }
    }

    pub fn propagate_signal(&mut self, start_id: &str, depth: usize) {
        println!("  [Wave Cascade] Starting propagation from '{}' (Depth: {})", start_id, depth);
        let mut current_frontier = vec![start_id.to_string()];

        for d in 1..=depth {
            let mut next_frontier = Vec::new();
            println!("    ├── Wave Layer {} active nodes: {:?}", d, current_frontier);

            let mut targets_to_check = Vec::new();
            for current_id in &current_frontier {
                if let Some(node) = self.nodes.get(current_id) {
                    for target in node.connections.keys() {
                        targets_to_check.push(target.clone());
                    }
                }
            }

            for target in targets_to_check {
                if let Some(target_node) = self.nodes.get_mut(&target) {
                    let current_w = target_node.connections.values().cloned().sum::<f64>() / (target_node.connections.len() as f64).max(1.0);
                    if current_w > 0.25 {
                        next_frontier.push(target);
                    }
                }
            }

            if next_frontier.is_empty() {
                break;
            }
            current_frontier = next_frontier;
        }
    }

    pub fn start_autopoietic_daemon(&mut self, memory_file: &str, cycles: usize) {
        println!("\n--- [ Activating Self-Modifying Daemon with Synthesis & Pruning ] ---");
        for cycle in 1..=cycles {
            thread::sleep(Duration::from_millis(1000));
            println!("\n[Daemon Cycle {}] Scanning topology & applying cognitive decay...", cycle);
            
            self.decay_and_prune();
            self.synthesize_concepts(memory_file);

            let node_ids: Vec<String> = self.nodes.keys().cloned().collect();
            if !node_ids.is_empty() {
                let target_index = cycle % node_ids.len();
                let active_id = node_ids[target_index].clone();
                if let Some(pattern_node) = self.nodes.get(&active_id).cloned() {
                    println!("  ├── [Self-Dialog] Processing concept node: {}", active_id);
                    self.find_and_evolve(&pattern_node, memory_file);
                }
            }
        }
    }
}
