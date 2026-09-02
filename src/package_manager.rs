use std::fs;
use std::path::Path;
use std::process::Command;

pub fn install_from_registry(pkg_name: &str) {
    println!("[x] Connecting to AURA Central Registry...");
    
    // मान ले कि पैकेजेस GitHub पर इस फॉर्मेट में होस्टेड हैं: 
    // https://github.com/nikhilsharma987880-bot/aura-packages/<pkg_name>
    let repo_url = format!("https://github.com/nikhilsharma987880-bot/aura-packages.git");
    let target_dir = format!(".aura_modules/{}", pkg_name);

    if Path::new(&target_dir).exists() {
        println!("[!] Package '{}' is already installed.", pkg_name);
        return;
    }

    // फोल्डर बनाएं जहाँ लाइब्रेरी स्टोर होगी
    fs::create_dir_all(".aura_modules").ok();

    println!("[x] Downloading package '{}'...", pkg_name);
    
    // गिट या एचटीटीपी के जरिए सीधे रिमोट पैकेज खींचना
    let status = Command::new("git")
        .args(["clone", &repo_url, &target_dir])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("[x] Successfully installed package: {}", pkg_name);
            update_aura_toml(pkg_name);
        }
        _ => {
            println!("Error: Failed to download package '{}'. Check network or package name.", pkg_name);
        }
    }
}

fn update_aura_toml(pkg_name: &str) {
    let mut config = fs::read_to_string("aura.toml").unwrap_or_default();
    config.push_str(&format!("\n[dependencies.{}]\nversion = \"latest\"\n", pkg_name));
    fs::write("aura.toml", config).ok();
    println!("[x] Updated 'aura.toml' dependencies.");
}
