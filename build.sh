#!/usr/bin/env bash
set -e

COMMAND=${1:-build}

if [ "$COMMAND" = "dev" ]; then
    echo "=> Starting development mode with hot reloading..."
    
    # Run tailwind in watch mode in background
    pnpm exec tailwindcss -i assets/tailwind-input.css -o assets/tailwind.min.css --watch &
    TAILWIND_PID=$!
    
    # Trap exit signal to kill background tailwind process on Ctrl+C
    trap "kill $TAILWIND_PID 2>/dev/null || true" EXIT
    
    # Run dx serve for rust hot reloading
    dx serve --platform web
    
    exit 0
fi

echo "=> Cleaning old UI WASM artifacts..."
cargo clean -p happyhome

echo "=> Building Tailwind CSS..."
pnpm exec tailwindcss -i assets/tailwind-input.css -o assets/tailwind.min.css --minify

echo "=> Compiling optimized WASM and bundling UI..."
dx build --platform web --release

echo "=> Copying assets to public directory..."
mkdir -p target/dx/happyhome/release/web/public/assets
cp -R assets/* target/dx/happyhome/release/web/public/assets/

echo "=> Running wasm-opt for further size reduction..."
if command -v wasm-opt &> /dev/null; then
    find target/dx/happyhome/release/web -name "*.wasm" -type f -exec wasm-opt -Oz -o {} {} \;
else
    echo "wasm-opt not found, skipping additional optimization."
fi

echo "=> Build Complete! Blazing fast, sub-second FCP WASM bundle is ready in target/dx/happyhome/release/web/public/"
