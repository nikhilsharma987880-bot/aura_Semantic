mod node;
mod network;
mod parser;
mod interpreter;
mod compiler;
mod cli;
mod module_loader;
mod package_manager;

use network::SemanticNetwork;
use interpreter::SymbolInterpreter;
use compiler::AuraCompiler;
use std::{env, thread, time::Duration, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let memory_file = "aura_memory.json";
cli::handle_cli();
    // यदि कोई आर्ग्यूमेंट न हो, तो डिफॉल्ट डेमो बूट और डेमन लूप चलाएं
    if args.len() < 2 {
        println!("==========================================");
        println!("   AURA: Autonomous Semantic Engine v0.1  ");
        println!("==========================================");

        let mut net = SemanticNetwork::load_from_file(memory_file)
            .unwrap_or_else(|| {
                println!("[*] Initializing baseline semantic fabric...");
                let mut network = SemanticNetwork::new();
                let mut n1 = node::SymbolNode::new("Astitva (Existence)");
                n1.link("Chetna (Consciousness)", 0.85);
                n1.link("Spandan (Vibration)", 0.70);
                network.add_node(n1);
                network
            });

        let sample_script = r#"
            # AURA Core Boot Script
            INJECT "Chetna (Consciousness)" -> "Gyan (Knowledge)" (0.8)
            INJECT "Gyan (Knowledge)" -> "Vikas (Evolution)" (0.9)
            PROPAGATE "Astitva (Existence)" DEPTH 2
        "#;
        let _ = std::fs::write("main.aura", sample_script);

        // सीधे .aura फाइल को कंपाइल करना
        if let Ok(()) = AuraCompiler::compile_file("main.aura", "main.aura_bin") {
            println!("[*] Direct .aura compilation test passed successfully!");
        }

        if let Ok(tokens) = AuraCompiler::load_bytecode("main.aura_bin") {
            let interpreter = SymbolInterpreter { token_stream: tokens };
            interpreter.execute(&mut net, memory_file);
        }

        let _ = net.save_to_file(memory_file);
        net.start_autopoietic_daemon(memory_file, 5);

        println!("\n[*] Entering Continuous Autonomous Background Loop (Press Ctrl+C to exit)...");
        loop {
            net.find_and_evolve(
                &node::SymbolNode::new("Chetna (Consciousness)"), 
                memory_file
            );
            thread::sleep(Duration::from_secs(3));
        }
    }

    // CLI कमांड हैंडलिंग
    let command = &args[1];
    match command.as_str() {
        "build" => {
            if args.len() < 3 {
                println!("[-] Error: Missing script path. Use: cargo run build <script.aura>");
                return;
            }
            let source_file = &args[2];
            let bin_file = format!("{}.aura_bin", source_file.trim_end_matches(".aura").trim_end_matches(".sym"));
            println!("[*] Compiling '{}' into '{}'...", source_file, bin_file);
            if let Ok(()) = AuraCompiler::compile_file(source_file, &bin_file) {
                println!("[*] Compilation successful!");
            }
        }
        "run" => {
            if args.len() < 3 {
                println!("[-] Error: Missing file path. Use: cargo run run <script.aura>");
                return;
            }
            let target_file = &args[2];
            let mut net = SemanticNetwork::load_from_file(memory_file).unwrap_or_else(|| {
                SemanticNetwork::new()
            });

            // यदि यूजर सीधा .aura फाइल पास करता है, तो उसे ऑन-द-फ्लाई बाइटकोड में कंपाइल कर लें
            let bin_file = if target_file.ends_with(".aura") || target_file.ends_with(".sym") {
                let generated_bin = format!("{}.aura_bin", target_file.trim_end_matches(".aura").trim_end_matches(".sym"));
                if let Ok(()) = AuraCompiler::compile_file(target_file, &generated_bin) {
                    println!("[*] Auto-compiled '{}' into bytecode.", target_file);
                } else {
                    println!("[-] Error: Failed to compile '{}'", target_file);
                    return;
                }
                generated_bin
            } else {
                target_file.clone()
            };

            if let Ok(tokens) = AuraCompiler::load_bytecode(&bin_file) {
                let interpreter = SymbolInterpreter { token_stream: tokens };
                interpreter.execute(&mut net, memory_file);
                let _ = net.save_to_file(memory_file);
                println!("[*] Execution completed and state persisted.");
            } else {
                println!("[-] Error: Could not load bytecode from '{}'", bin_file);
            }
        }
        _ => {
            println!("[-] Unknown command: '{}'. Use 'build' or 'run'.", command);
        }
    }
}
