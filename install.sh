#!/bin/bash
set -e

echo "[*] Cloning AURA Semantic Language repository..."
rm -rf /tmp/aura_Semantic
git clone https://github.com/nikhilsharma987880-bot/aura_Semantic.git /tmp/aura_Semantic

echo "[*] Entering AURA directory & compiling native runtime..."
cd /tmp/aura_Semantic
cargo build --release

echo "[*] Installing AURA binary..."
mkdir -p ~/.local/bin

# चेक करें कि कौन सी बाइनरी जनरेट हुई है और उसे 'aura' नाम से कॉपी करें
if [ -f "target/release/aura" ]; then
    cp target/release/aura ~/.local/bin/aura
elif [ -f "target/release/aura_symbolic" ]; then
    cp target/release/aura_symbolic ~/.local/bin/aura
else
    # अगर नाम कुछ और है तो रिलीज फोल्डर की पहली executable फाइल ढूंढकर कॉपी कर ले
    BINARY_NAME=$(find target/release -maxdepth 1 -type f -executable | head -n 1)
    if [ -n "$BINARY_NAME" ]; then
        cp "$BINARY_NAME" ~/.local/bin/aura
    else
        echo "Error: Could not find compiled binary to install."
        exit 1
    fi
fi

# पाथ सुनिश्चित करना
if [[ ":$PATH:" != ":$HOME/.local/bin:" ]]; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    export PATH="$HOME/.local/bin:$PATH"
fi

echo "[+] AURA successfully installed! Run 'aura --help' to get started."
