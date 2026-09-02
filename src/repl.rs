use std::io::{self, Write};
use crate::network::SemanticNetwork;
use crate::interpreter::SymbolInterpreter;
use crate::parser::SymbolParser;

pub fn run_repl(net: &mut SemanticNetwork, memory_file: &str) {
    println!("\n--- [ Aura Symbolic Interactive REPL ] ---");
    println!("Type your symbolic commands below:");
    println!("  INJECT \"Astitva (Existence)\" -> \"Chetna (Consciousness)\" (0.85)");
    println!("  PROPAGATE \"Astitva (Existence)\" DEPTH 2");
    println!("  RESONATE \"Astitva (Existence)\"");
    println!("  HEARTBEAT 3");
    println!("  EXIT\n");

    let mut interpreter = SymbolInterpreter::new();

    loop {
        print!("aura-shell> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("  [!] Error reading input.");
            break;
        }

        let command = input.trim();
        if command.eq_ignore_ascii_case("EXIT") || command.eq_ignore_ascii_case("QUIT") {
            let _ = net.save_to_file(memory_file);
            println!("[*] State persisted. Exiting runtime.");
            break;
        }

        if command.is_empty() {
            continue;
        }

        // स्वतंत्र हार्टबेट ट्रिगर करने के लिए शेल कमांड
        if command.starts_with("HEARTBEAT") {
            let cycles = command
                .split_whitespace()
                .last()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(2);
            net.start_autonomous_heartbeat(cycles);
            let _ = net.save_to_file(memory_file);
            continue;
        }

        // कस्टम पार्सर के जरिए कमांड को टोकन में बदलना
        let tokens = SymbolParser::parse_script(command);
        if tokens.is_empty() {
            println!("  [!] Unrecognized symbolic structure. Use INJECT, PROPAGATE, RESONATE, or HEARTBEAT.");
        } else {
            interpreter.token_stream = tokens;
            interpreter.execute(net);
        }

        // हर कमांड के बाद स्टेट सुरक्षित करना
        let _ = net.save_to_file(memory_file);
    }
}
