use std::env;
use std::fs;
use std::path::Path;

pub fn handle_cli() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("AURA Native Runtime v1.0.0");
        println!("Usage: aura <command> [arguments]");
        println!("Commands:");
        println!("  init             - Initialize a new .aura project");
        println!("  run <file.aura>  - Execute an AURA semantic script");
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
    println!("[x] Loading and executing semantic script: {}", filename);
    // यहाँ पार्सर और एग्जीक्यूटर कॉल होगा जो .aura कोड को प्रोसेस करेगा
}

fn install_package(pkg_name: &str) {
    println!("[x] Fetching semantic package '{}' from registry...", pkg_name);
    // पैकेज डाउनलोड करके लोकल फोल्डर में जोड़ने का लॉजिक
}
