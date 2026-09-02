#!/bin/bash

echo "[x] Compiling AURA Native Runtime (Black-box Mode)..."
cargo build --release

if [ -f "target/release/aura" ]; then
    echo "[x] Registering 'aura' command globally..."
    sudo cp target/release/aura /usr/local/bin/aura
    sudo chmod +x /usr/local/bin/aura
    echo "[x] Success! AURA is now a standalone independent language."
    echo "You can now open any terminal and type: aura init"
else
    echo "Error: Compilation failed. Check Rust setup."
fi
