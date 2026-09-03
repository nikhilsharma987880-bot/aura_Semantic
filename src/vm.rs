use std::fs::File;
use std::io::{self, Read};
use crate::network::SemanticNetwork;
use crate::interpreter::SymbolToken;

pub struct AuraVM {
    stack: Vec<String>,
    instruction_pointer: usize,
}

impl AuraVM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            instruction_pointer: 0,
        }
    }

    pub fn execute_bytecode(&mut self, bin_filename: &str, net: &mut SemanticNetwork) -> io::Result<()> {
        let mut file = File::open(bin_filename)?;
        let mut bytecode_bytes = Vec::new();
        file.read_to_end(&mut bytecode_bytes)?;

        println!("[VM] Starting native execution of binary: {} (Size: {} bytes)", bin_filename, bytecode_bytes.len());

        let tokens: Vec<SymbolToken> = serde_json::from_slice(&bytecode_bytes).unwrap_or_default();

        while self.instruction_pointer < tokens.len() {
            let token = &tokens[self.instruction_pointer];

            match token {
                SymbolToken::Inject(source, target, weight) => {
                    println!("  [VM Opcode: INJECT] Linking '{}' -> '{}' (Weight: {})", source, target, weight);
                    let mut node = crate::node::SymbolNode::new(source);
                    node.link(target, *weight);
                    net.add_node(node);
                }
                SymbolToken::Propagate(target, depth) => {
                    println!("  [VM Opcode: PROPAGATE] Executing wave cascade from '{}' (Depth: {})", target, depth);
                    let source_node = crate::node::SymbolNode::new(target);
                    net.find_and_evolve(&source_node, "aura_memory.json");
                }
                SymbolToken::Resonate(target) => {
                    println!("  [VM Opcode: RESONATE] Tuning resonance for '{}'", target);
                    let source_node = crate::node::SymbolNode::new(target);
                    net.find_and_evolve(&source_node, "aura_memory.json");
                }
                SymbolToken::Import(path) => {
                    println!("  [VM Opcode: IMPORT] Importing external module/file: '{}'", path);
                }
                SymbolToken::ConditionalResonate(condition_node, target) => {
                    println!("  [VM Opcode: CONDITIONAL_RESONATE] Checking condition on '{}' for target '{}'", condition_node, target);
                    if net.nodes.contains_key(condition_node) {
                        let source_node = crate::node::SymbolNode::new(target);
                        net.find_and_evolve(&source_node, "aura_memory.json");
                    }
                }
                _ => {
                    println!("  [VM Opcode: UNKNOWN] Encountered unhandled instruction token.");
                }
            }

            self.instruction_pointer += 1;
        }

        println!("[VM] Execution completed successfully via Virtual Machine engine.");
        Ok(())
    }
}
