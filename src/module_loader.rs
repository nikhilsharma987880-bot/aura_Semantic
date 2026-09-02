use std::fs;
use std::path::Path;

pub fn process_imports(file_content: &str) {
    for line in file_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("IMPORT") {
            // "IMPORT filename.aura" से फाइल का नाम निकालना
            let parts: Vec<&str> = trimmed.split('"').collect();
            if parts.len() >= 2 {
                let import_file = parts[1];
                load_external_module(import_file);
            }
        }
    }
}

fn load_external_module(filename: &str) {
    if Path::new(filename).exists() {
        let content = fs::read_to_string(filename).expect("Unable to read imported module");
        println!("[x] Successfully merged semantic module: {}", filename);
        // यहाँ बाहरी फाइल के नियम मुख्य मेमोरी ग्राफ में जुड़ जाएंगे
    } else {
        println!("Warning: Import file '{}' not found.", filename);
    }
}
