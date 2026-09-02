use crate::interpreter::SymbolToken;

pub struct SymbolParser;

impl SymbolParser {
    pub fn parse_script(script: &str) -> Vec<SymbolToken> {
        let mut tokens = Vec::new();
        let mut lines = script.lines().peekable();

        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("INJECT") {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 4 {
                    let src = parts[1].to_string();
                    let target = parts[3].to_string();
                    let rest = parts[4];
                    let strength = rest
                        .chars()
                        .filter(|c| c.is_digit(10) || *c == '.')
                        .collect::<String>()
                        .parse::<f64>()
                        .unwrap_or(0.5);

                    tokens.push(SymbolToken::Inject(src, target, strength));
                }
            } else if line.starts_with("PROPAGATE") {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 2 {
                    let start_id = parts[1].to_string();
                    let depth = line
                        .split_whitespace()
                        .last()
                        .and_then(|s| s.parse::<usize>().ok())
                        .unwrap_or(1);

                    tokens.push(SymbolToken::Propagate(start_id, depth));
                }
            } else if line.starts_with("RESONATE") {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 2 {
                    let query_id = parts[1].to_string();
                    tokens.push(SymbolToken::Resonate(query_id));
                }
            } else if line.starts_with("IMPORT") {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 2 {
                    let filepath = parts[1].to_string();
                    tokens.push(SymbolToken::Import(filepath));
                }
            } else if line.starts_with("IF") {
                // सिंटैक्स: IF "NodeName" { ... मल्टी-लाइन ब्लॉक या सिंगल लाइन ... }
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 2 {
                    let condition_node = parts[1].to_string();
                    
                    let mut block_content = String::new();
                    let mut brace_count = if line.contains('{') { 1 } else { 0 };

                    if line.contains('{') && line.contains('}') {
                        // एक ही लाइन में ब्लॉक हो तो
                        if let Some(start) = line.find('{') {
                            if let Some(end) = line.rfind('}') {
                                block_content = line[start + 1..end].trim().to_string();
                                brace_count = 0;
                            }
                        }
                    } else if line.contains('{') {
                        // मल्टी-लाइन ब्लॉक को रीड करना
                        while let Some(next_line) = lines.next() {
                            let trimmed_next = next_line.trim();
                            if trimmed_next.contains('{') {
                                brace_count += 1;
                            }
                            if trimmed_next.contains('}') {
                                brace_count -= 1;
                                if brace_count == 0 {
                                    break;
                                }
                            }
                            block_content.push_str(next_line);
                            block_content.push('\n');
                        }
                    }

                    tokens.push(SymbolToken::ConditionalResonate(condition_node, block_content));
                }
            }
        }

        tokens
    }
}
