use std::fs;
use std::path::Path;

pub fn process_imports(file_content: &str) -> String {
    let mut combined_content = String::new();
    
    for (line_idx, line) in file_content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("IMPORT") {
            let parts: Vec<&str> = trimmed.split('"').collect();
            if parts.len() >= 2 {
                let import_file = parts[1];
                let module_content = load_external_module(import_file, line_idx + 1);
                combined_content.push_str(&module_content);
                combined_content.push('\n');
            } else {
                println!("AuraError [ImportError] on line {}: Invalid IMPORT syntax.", line_idx + 1);
                println!("  = help: Correct syntax is: IMPORT \"filename.aura\"\n");
            }
        } else {
            combined_content.push_str(line);
            combined_content.push('\n');
        }
    }
    
    combined_content
}

fn load_external_module(filename: &str, line_no: usize) -> String {
    // पहले लोकल फोल्डर में चेक करें, फिर /packages/ डायरेक्टरी में देखें
    let primary_path = Path::new(filename);
    let fallback_path = Path::new("packages").join(filename);

    let target_path = if primary_path.exists() {
        primary_path
    } else if fallback_path.exists() {
        &fallback_path
    } else {
        println!("AuraError [ModuleNotFound]: Import file '{}' not found on line {}.", filename, line_no);
        println!("  = help: Verify that the package exists in the root or /packages/ directory.\n");
        return String::new();
    };

    match fs::read_to_string(&target_path) {
        Ok(content) => {
            println!("[x] Successfully merged semantic module: {:?} (at line {})", target_path, line_no);
            content
        }
        Err(_) => {
            println!("AuraError [ReadError]: Failed to read module file {:?} on line {}.", target_path, line_no);
            String::new()
        }
    }
}
