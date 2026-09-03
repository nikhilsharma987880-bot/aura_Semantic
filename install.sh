#!/bin/bash
set -e

echo "[*] Cloning AURA Semantic Language repository..."
rm -rf /tmp/aura_Semantic
git clone https://github.com/nikhilsharma987880-bot/aura_Semantic.git /tmp/aura_Semantic

echo "[*] Entering AURA directory & compiling native runtime..."
cd /tmp/aura_Semantic
cargo build --release

echo "[*] Installing AURA binary..."
# बाइनरी को यूजर के लोकल बिन फोल्डर में कॉपी कर रहे हैं ताकि sudo की जरूरत न पड़े
mkdir -p ~/.local/bin
cp target/release/aura ~/.local/bin/aura

# पाथ सुनिश्चित करना
if [[ ":$PATH:" != ":$HOME/.local/bin:" ]]; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    export PATH="$HOME/.local/bin:$PATH"
fi

echo "[+] AURA successfully installed! Run 'aura --help' to get started."
