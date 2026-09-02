use crate::network::SemanticNetwork;
use crate::node::SymbolNode;
use crate::parser::SymbolParser;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymbolToken {
    Inject(String, String, f64),
    Resonate(String),
    Propagate(String, usize),
    Import(String),
    ConditionalResonate(String, String),
}

pub struct SymbolInterpreter {
    pub token_stream: Vec<SymbolToken>,
}

impl SymbolInterpreter {
    pub fn new() -> Self {
        SymbolInterpreter {
            token_stream: Vec::new(),
        }
    }

    pub fn push_token(&mut self, token: SymbolToken) {
        self.token_stream.push(token);
    }

    pub fn execute(&self, net: &mut SemanticNetwork, memory_file: &str) {
        println!("\n--- [ Executing Compiled Symbolic Token Stream ] ---");

        for (index, token) in self.token_stream.iter().enumerate() {
            match token {
                SymbolToken::Inject(src, target, strength) => {
                    println!("  [{}] Injecting Link: {} -> {} ({:.1})", index, src, target, strength);
                    if let Some(node) = net.nodes.get_mut(src) {
                        node.link(target, *strength);
                    } else {
                        let mut new_node = SymbolNode::new(src);
                        new_node.link(target, *strength);
                        net.add_node(new_node);
                    }
                }
                SymbolToken::Resonate(query_id) => {
                    println!("  [{}] Triggering Resonance: {}", index, query_id);
                    let query_pattern = net.nodes.get(query_id).cloned().unwrap_or_else(|| {
                        SymbolNode::new(query_id)
                    });
                    net.find_and_evolve(&query_pattern, memory_file);
                }
                SymbolToken::Propagate(start_id, depth) => {
                    println!("  [{}] Propagating Wave from: {} (Depth: {})", index, start_id, depth);
                    net.propagate_signal(start_id, *depth);
                }
                SymbolToken::Import(filepath) => {
                    println!("  [{}] Importing Module: '{}'", index, filepath);
                    if let Ok(script_content) = std::fs::read_to_string(filepath) {
                        let sub_tokens = SymbolParser::parse_script(&script_content);
                        let sub_interpreter = SymbolInterpreter { token_stream: sub_tokens };
                        sub_interpreter.execute(net, memory_file);
                    }
                }
                SymbolToken::ConditionalResonate(cond_node, sub_script) => {
                    println!("  [{}] Evaluating Semantic Condition: IF node '{}' exists in Network", index, cond_node);
                    
                    if net.nodes.contains_key(cond_node) {
                        println!("    ├── Condition Met! Executing Block Inside IF Statement...");
                        let sub_tokens = SymbolParser::parse_script(sub_script);
                        let sub_interpreter = SymbolInterpreter { token_stream: sub_tokens };
                        sub_interpreter.execute(net, memory_file);
                    } else {
                        println!("    ├── Condition Failed. Skipping Block.");
                    }
                }
            }
        }
    }
}
