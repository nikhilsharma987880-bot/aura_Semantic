use std::fs;
use std::path::Path;
use std::process::Command;

pub fn install_from_registry(pkg_name: &str) {
    println!("[x] Connecting to AURA Central Registry...");

    // यदि रिमोट रिपॉजिटरी से क्लोन करना है या लोकल फॉールबैक इस्तेमाल करना है
    let repo_url = format!("https://github.com/nikhilsharma987880-bot/aura-packages.git");
    let target_dir = format!("packages/{}", pkg_name);

    let packages_dir = Path::new("packages");
    if !packages_dir.exists() {
        fs::create_dir_all(packages_dir).expect("Failed to create packages directory");
    }

    if Path::new(&target_dir).exists() {
        println!("[!] Package '{}' is already present locally.", pkg_name);
        return;
    }

    println!("[x] Fetching package '{}' from registry...", pkg_name);

    // गिट क्लोन के जरिए पैकेज खींचने की कोशिश करें, अगर नेटवर्क न हो तो लोकल टेम्पलेट बना दें
    let status = Command::new("git")
        .args(["clone", &repo_url, &target_dir])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("[x] Successfully installed package: {}", pkg_name);
            update_aura_toml(pkg_name);
        }
        _ => {
            println!("[!] Network clone failed. Creating localized semantic package template for '{}'...", pkg_name);
            let target_file = packages_dir.join(format!("{}.aura", pkg_name));
            let pkg_content = format!("# AURA Auto-linked Package: {}\nINJECT \"{}_Root\" -> \"Core_Resonance\" (0.90)\n", pkg_name, pkg_name);
            fs::write(&target_file, pkg_content).expect("Failed to write package file");
            println!("[+] Successfully generated and linked package into /packages/{}.aura", pkg_name);
            update_aura_toml(pkg_name);
        }
    }
}

fn update_aura_toml(pkg_name: &str) {
    let mut config = fs::read_to_string("aura.toml").unwrap_or_else(|_| {
        "[package]\nname = \"my_aura_world\"\nversion = \"1.0.0\"\nauthor = \"Developer\"\n".to_string()
    });
    
    if !config.contains(pkg_name) {
        config.push_str(&format!("\n[dependencies.{}]\nversion = \"latest\"\n", pkg_name));
        fs::write("aura.toml", config).ok();
        println!("[x] Updated 'aura.toml' with new dependency.");
    }
}
