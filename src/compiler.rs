use std::fs;
use std::io::Result;
use crate::interpreter::SymbolToken;
use crate::parser::SymbolParser;

pub struct AuraCompiler;

impl AuraCompiler {
    // टेक्स्ट स्क्रिप्ट फाइल को बाइटकोड (.aura_bin) में कंपाइल करना
    pub fn compile_file(source_path: &str, output_bin_path: &str) -> Result<()> {
        println!("[*] Compiling '{}' into binary bytecode '{}'...", source_path, output_bin_path);
        let script_content = fs::read_to_string(source_path)?;
        let tokens = SymbolParser::parse_script(&script_content);

        // टोकन्स को बाइनरी फॉर्मेट में सीरियलाइज करना
        let serialized_bytecode = bincode_serialize(&tokens);
        fs::write(output_bin_path, serialized_bytecode)?;
        println!("[*] Compilation successful! Binary size: {} bytes", fs::metadata(output_bin_path)?.len());
        Ok(())
    }

    // सीधे बाइटकोड फाइल से टोकन स्ट्रीम लोड करना
    pub fn load_bytecode(bin_path: &str) -> Result<Vec<SymbolToken>> {
        let bytes = fs::read(bin_path)?;
        let tokens = bincode_deserialize(&bytes);
        println!("[*] Successfully loaded compiled bytecode from '{}'", bin_path);
        Ok(tokens)
    }
}

// लाइटवेट बाइनरी एन्कोडिंग/डिकोडिंग हेल्पर
fn bincode_serialize(tokens: &[SymbolToken]) -> Vec<u8> {
    serde_json::to_vec(tokens).unwrap_or_default()
}

fn bincode_deserialize(bytes: &[u8]) -> Vec<SymbolToken> {
    serde_json::from_slice(bytes).unwrap_or_default()
}
