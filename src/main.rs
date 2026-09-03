mod node;
mod network;
mod parser;
mod interpreter;
mod compiler;
mod module_loader;
mod repl;
mod vm;
mod standard_lib;

use network::SemanticNetwork;
use interpreter::SymbolInterpreter;
use compiler::AuraCompiler;
use vm::AuraVM;
use std::{env, thread, time::Duration, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let memory_file = "aura_memory.json";

    // यदि कोई आर्ग्यूमेंट न हो, तो सीधा इंटरेक्टिव REPL शेल खोलें
    if args.len() < 2 {
        repl::start_repl(memory_file);
        return;
    }

    // CLI कमांड हैंडलिंग
    let command = &args[1];
    match command.as_str() {
        "init" => {
            init_project();
        }
        "build" => {
            if args.len() < 3 {
                println!("[-] Error: Missing script path. Use: aura build <script.aura>");
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
                println!("[-] Error: Missing file path. Use: aura run <script.aura>");
                return;
            }
            let target_file = &args[2];
            
            // इम्पोर्ट्स प्रोसेस करें
            if let Ok(content) = std::fs::read_to_string(target_file) {
                let _processed_content = module_loader::process_imports(&content);
            }

            let mut net = SemanticNetwork::load_from_file(memory_file).unwrap_or_else(|| {
                SemanticNetwork::new()
            });

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

            // अब डेडिकेटेड VM इंजन के जरिए बाइटकोड एग्जीक्यूट करें
            let mut vm_engine = AuraVM::new();
            if let Err(e) = vm_engine.execute_bytecode(&bin_file, &mut net) {
                println!("[-] VM Execution Error: {}", e);
            }

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
            println!("[-] Unknown command: '{}'. Use 'init', 'build', or 'run'.", command);
        }
    }
}

fn init_project() {
    let config_content = "[package]\nname = \"my_aura_world\"\nversion = \"1.0.0\"\nauthor = \"Developer\"\n";
    std::fs::write("aura.toml", config_content).expect("Failed to create aura.toml");
    
    let main_content = "# AURA Main Semantic Script\nINJECT \"Srot (Source)\" -> \"Chetna (Consciousness)\" (0.95)\nPROPAGATE \"Srot (Source)\" DEPTH 2\n";
    std::fs::write("main.aura", main_content).expect("Failed to create main.aura");
    
    println!("[x] Initialized new AURA project with 'aura.toml' and 'main.aura'.");
}
