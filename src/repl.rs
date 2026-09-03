use std::io::{self, Write};
use crate::network::SemanticNetwork;
use crate::interpreter::SymbolInterpreter;
use crate::parser::SymbolParser;

pub fn start_repl(memory_file: &str) {
    println!("==================================================");
    println!("   AURA Interactive Semantic Shell v1.0         ");
    println!("   Type 'help' for commands or 'exit' to quit.    ");
    println!("==================================================");

    let mut net = SemanticNetwork::load_from_file(memory_file).unwrap_or_else(|| {
        SemanticNetwork::new()
    });

    let mut interpreter = SymbolInterpreter::new();

    loop {
        print!("aura> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("[-] Error reading input.");
            break;
        }

        let command = input.trim();
        if command.is_empty() {
            continue;
        }

        match command {
            "exit" | "quit" => {
                println!("[*] Saving state and exiting REPL...");
                let _ = net.save_to_file(memory_file);
                break;
            }
            "help" => {
                println!("Available REPL Commands:");
                println!("  INJECT \"Source\" -> \"Target\" (weight) - Create a semantic link");
                println!("  PROPAGATE \"Node\" DEPTH n               - Trigger wave propagation");
                println!("  HEARTBEAT n                            - Run autonomous background cycles");
                println!("  status                                 - View active network nodes");
                println!("  clear                                  - Clear session memory");
                println!("  exit / quit                            - Exit the AURA shell");
            }
            "status" => {
                println!("[*] Active Semantic Network State loaded from memory.");
                net.inspect_fabric();
            }
            "clear" => {
                net = SemanticNetwork::new();
                println!("[x] Network memory cleared for this session.");
            }
            cmd if cmd.starts_with("HEARTBEAT") => {
                let cycles = cmd
                    .split_whitespace()
                    .last()
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(2);
                net.start_autonomous_heartbeat(cycles);
                let _ = net.save_to_file(memory_file);
            }
            _ => {
                // कंपाइलर और पार्सर के जरिए ऑथेंटिक सेमांटिक कमांड प्रोसेस करना
                let tokens = SymbolParser::parse_script(command);
                if tokens.is_empty() {
                    println!("[-] Unknown command or syntax error. Type 'help' for options.");
                } else {
                    interpreter.token_stream = tokens;
                    interpreter.execute(&mut net, memory_file);
                    let _ = net.save_to_file(memory_file);
                }
            }
        }
    }
}
