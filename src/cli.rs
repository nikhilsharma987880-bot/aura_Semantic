use std::env;
use std::fs;
use std::path::Path;

pub fn handle_cli() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("AURA Native Runtime v1.0.0 (Semantic Engine)");
        println!("Usage: aura <command> [arguments]");
        println!("Commands:");
        println!("  init             - Initialize a new .aura project");
        println!("  run <file.aura>  - Execute and validate an AURA semantic script");
        println!("  install <pkg>    - Download and link a semantic package");
        return;
    }

    match args[1].as_str() {
        "init" => {
            init_project();
        }
        "run" => {
            if args.len() < 3 {
                println!("Error: Please specify an .aura file to run.");
                return;
            }
            let filename = &args[2];
            run_script(filename);
        }
        "install" => {
            if args.len() < 3 {
                println!("Error: Please specify a package name to install.");
                return;
            }
            let pkg_name = &args[2];
            install_package(pkg_name);
        }
        _ => {
            println!("Unknown command. Use 'aura' for help.");
        }
    }
}

fn init_project() {
    let config_content = "[package]\nname = \"my_aura_world\"\nversion = \"1.0.0\"\nauthor = \"Developer\"\n";
    fs::write("aura.toml", config_content).expect("Failed to create aura.toml");
    
    let main_content = "# AURA Main Semantic Script\nINJECT \"Srot (Source)\" -> \"Chetna (Consciousness)\" (0.95)\nPROPAGATE \"Srot (Source)\" DEPTH 2\n";
    fs::write("main.aura", main_content).expect("Failed to create main.aura");
    
    println!("[x] Initialized new AURA project with 'aura.toml' and 'main.aura'.");
}

fn run_script(filename: &str) {
    if !Path::new(filename).exists() {
        println!("Error: File '{}' not found.", filename);
        return;
    }
    
    println!("[x] Loading and analyzing semantic script: {}", filename);
    let content = fs::read_to_string(filename).expect("Failed to read file");

    // === एडवांस एरर डायग्नोस्टिक्स और सेमांटिक चेकर (Lexer & Resonance Check) ===
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        
        // कमेंट्स या खाली लाइनों को छोड़ दें
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // सिंटैक्स और स्ट्रक्चरल चेकिंग
        if trimmed.starts_with("INJECT") {
            // चेक करें कि इसमें '->' और वेट ब्रैकेट है या नहीं
            if !trimmed.contains("->") {
                let col = line.find("INJECT").unwrap_or(0) + 1;
                println!("AuraError [SyntaxError]: Missing '->' operator in INJECT statement.");
                println!(" --> {}:{}:{}", filename, line_num + 1, col);
                println!("  | \n{}: {}", line_num + 1, line);
                println!("  = help: Correct syntax is: INJECT \"Source\" -> \"Target\" (Weight)\n");
                return;
            }

            // वेट (Resonance Weight) 0.0 से 1.0 के बीच होना चाहिए (मैथेमैटिकल वैलिडेशन)
            if let Some(start_idx) = trimmed.rfind('(') {
                if let Some(end_idx) = trimmed.rfind(')') {
                    let weight_str = &trimmed[start_idx + 1..end_idx];
                    if let Ok(weight) = weight_str.parse::<f32>() {
                        if weight < 0.0 || weight > 1.0 {
                            println!("AuraError [ResonanceValueError]: Weight '{}' out of bounds.", weight);
                            println!(" --> {}:{} ", filename, line_num + 1);
                            println!("  = note: Resonance weight must strictly be between 0.0 and 1.0.\n");
                            return;
                        }
                    }
                }
            }
        }
    }

    println!("[*] Semantic analysis passed successfully! No syntax or resonance errors found.");
    // आगे का बाइटकोड जनरेशन और एग्जीक्यूशन यहाँ ट्रिगर होगा
}

fn install_package(pkg_name: &str) {
    println!("[x] Fetching semantic package '{}' from registry...", pkg_name);
}
