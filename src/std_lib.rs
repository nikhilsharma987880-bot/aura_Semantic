use crate::network::SemanticNetwork;
use crate::node::SymbolNode;
use std::fs;
use std::io;

pub struct SemanticStdLib;

impl SemanticStdLib {
    // 1. किसी टेक्स्ट डॉक्यूमेंट या डेटा को पढ़कर उसके शब्दों/विचारों को ऑटोमैटिकली नेटवर्क से जोड़ना
    pub fn ingest_text_to_graph(net: &mut SemanticNetwork, text: &str, default_weight: f64) {
        let concepts: Vec<&str> = text
            .split(|c: char| c.is_whitespace() || c == '.' || c == ',' || c == '\n')
            .filter(|s| !s.is_empty() && s.len() > 3)
            .collect();

        println!("[StdLib] Ingesting {} concepts into semantic fabric...", concepts.len());

        for window in concepts.windows(2) {
            let src = window[0].to_string();
            let target = window[1].to_string();

            if let Some(node) = net.nodes.get_mut(&src) {
                let current_w = node.connections.get(&target).unwrap_or(&0.0);
                node.link(&target, (current_w + default_weight).min(1.0));
            } else {
                let mut new_node = SymbolNode::new(&src);
                new_node.link(&target, default_weight);
                net.add_node(new_node);
            }
        }
    }

    // 2. किसी बाहरी डेटा या JSON फाइल से सेमांटिक मेमोरी लोड करना
    pub fn import_external_knowledge(net: &mut SemanticNetwork, filepath: &str) -> io::Result<()> {
        let content = fs::read_to_string(filepath)?;
        Self::ingest_text_to_graph(net, &content, 0.4);
        println!("[StdLib] Successfully mapped external file '{}' to semantic network.", filepath);
        Ok(())
    }

    // 3. इंटरनेट से लाइव डेटा फेच करके सीधे सेमांटिक नोड्स में बदलना
    pub fn fetch_and_inject_web(net: &mut SemanticNetwork, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("[StdLib] Fetching live data from URL: '{}'", url);
        
        // ureq HTTP क्लाइंट का उपयोग करके वेब डेटा फेच करना
        let response = ureq::get(url).call()?;
        let response_text = response.into_string()?;

        println!("[StdLib] Processing and mapping web response into semantic nodes...");
        
        // टेक्स्ट को शब्दों/कॉन्सेप्ट्स में तोड़कर नोड्स से जोड़ना
        let concepts: Vec<&str> = response_text
            .split(|c: char| c.is_whitespace() || c == '"' || c == '{' || c == '}' || c == ':')
            .filter(|s| !s.is_empty() && s.len() > 4)
            .take(50) // शुरुआती 50 मुख्य कॉन्सेप्ट्स लेना
            .collect();

        for window in concepts.windows(2) {
            let src = window[0].to_string();
            let target = window[1].to_string();

            if let Some(node) = net.nodes.get_mut(&src) {
                let current_w = node.connections.get(&target).unwrap_or(&0.0);
                node.link(&target, (current_w + 0.3).min(1.0));
            } else {
                let mut new_node = SymbolNode::new(&src);
                new_node.link(&target, 0.3);
                net.add_node(new_node);
            }
        }

        println!("[StdLib] Live web data successfully integrated into the semantic network!");
        Ok(())
    }
}
