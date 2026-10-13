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

if [ "$COMMAND" = "clean" ]; then
    echo "=> Full clean: WASM artifacts + bundled output + build cache..."
    cargo clean -p happyhome
    rm -rf target/dx
    echo "=> Clean done. Next build starts from scratch (deps still cached)."
    exit 0
fi

echo "=> Cleaning old UI WASM artifacts..."
cargo clean -p happyhome

echo "=> Wiping previous bundle output (stale hashed files, .br, leftovers)..."
# cargo clean does NOT touch target/dx: old hashed bundles (styles-dxh*.css,
# happyhome-dxh*.js, their .br twins) accumulate there across builds and are
# easy to mistake for the live bundle when inspecting output by hand.
rm -rf target/dx/happyhome/release/web

echo "=> Building Tailwind CSS..."
pnpm exec tailwindcss -i assets/tailwind-input.css -o assets/tailwind.min.css --minify

echo "=> Generating SEO artifacts from config (robots, sitemap, llms.txt, JSON-LD)..."
cargo run --bin seo_artifacts

echo "=> Compiling optimized WASM and bundling UI..."
BASE_ARG=""
if [ -n "$BASE_PATH" ]; then
    BASE_ARG="--base-path $BASE_PATH"
fi
dx build --platform web --release $BASE_ARG

echo "=> Copying assets to public directory..."
mkdir -p target/dx/happyhome/release/web/public/assets
cp -R assets/* target/dx/happyhome/release/web/public/assets/
# Tailwind source input is a build artefact, never served.
rm -f target/dx/happyhome/release/web/public/assets/tailwind-input.css{,.br}

echo "=> Running wasm-opt for further size reduction..."
if command -v wasm-opt &> /dev/null; then
    find target/dx/happyhome/release/web -name "*.wasm" -type f -exec wasm-opt -Oz --enable-bulk-memory --strip-debug -o {} {} \;
    find target/dx/happyhome/release/web -name "*.wasm.br" -delete
else
    echo "wasm-opt not found, skipping additional optimization."
fi

echo "=> Staging standalone distribution folder for GitHub Pages..."
rm -rf dist
mkdir -p dist
cp -R target/dx/happyhome/release/web/public/* dist/
# GitHub Pages SPA fallback: 404.html serves index.html for client-side routing
cp dist/index.html dist/404.html
# Bypass Jekyll processing so GitHub Pages serves raw wasm and underscore assets
touch dist/.nojekyll

echo "=> Pre-compressing copied assets (brotli)..."
# dx pre-compresses its own hashed output during the build; the raw assets we
# cp afterwards are the only ones left uncompressed. Skip files already paired.
if command -v brotli &> /dev/null; then
    find target/dx/happyhome/release/web/public dist -type f ! -name "*.br" -print0 |
        while IFS= read -r -d '' f; do
            [ -f "$f.br" ] || brotli -q 11 -o "$f.br" "$f"
        done
else
    echo "brotli not found, skipping pre-compression."
fi

echo "=> Output report..."
du -sh target/dx/happyhome/release/web/public dist
echo "   Largest files in dist/:"
find dist -type f ! -name "*.br" -exec du -k {} + \
    | sort -rn | head -5 | awk '{print $1" KB  "$2}'

echo "=> Build Complete! Standalone GitHub Pages distribution ready in dist/"
