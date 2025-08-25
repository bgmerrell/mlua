#!/bin/bash

# Build script for mlua with WASI exception handling support
# This script handles the required wasm-opt post-processing for lua51-wasi builds

set -e

echo "Building mlua with WASI exception handling support..."

# Default values
TARGET="wasm32-wasip1"
FEATURES="lua51-wasi,macros,vendored"
BUILD_TYPE="debug"
OUTPUT_DIR=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --release)
            BUILD_TYPE="release"
            shift
            ;;
        --features)
            FEATURES="$2"
            shift 2
            ;;
        --target)
            TARGET="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --release          Build in release mode"
            echo "  --features FEAT    Specify features (default: lua51-wasi,macros,vendored)"
            echo "                     Note: async feature excluded by default due to WASI limitations"
            echo "  --target TARGET    Specify target (default: wasm32-wasip1)"
            echo "  --help            Show this help message"
            echo ""
            echo "This script builds mlua for WASI and applies the required wasm-opt"
            echo "post-processing for exception handling support."
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Determine output directory
if [[ "$BUILD_TYPE" == "release" ]]; then
    OUTPUT_DIR="target/$TARGET/release"
    CARGO_FLAGS="--release"
else
    OUTPUT_DIR="target/$TARGET/debug"
    CARGO_FLAGS=""
fi

echo "Building with features: $FEATURES"
echo "Target: $TARGET"
echo "Build type: $BUILD_TYPE"
echo "Output directory: $OUTPUT_DIR"

# Check if wasm-opt is available
if ! command -v wasm-opt &> /dev/null; then
    echo "Error: wasm-opt is not installed or not in PATH"
    echo "Please install binaryen toolkit: https://github.com/WebAssembly/binaryen"
    exit 1
fi

# Build the project
echo "Running cargo build..."
WASI_SDK="${WASI_SDK:-}" cargo build \
    --target "$TARGET" \
    --features "$FEATURES" \
    $CARGO_FLAGS

echo "Build completed successfully!"

# Find WASM files in the output directory
WASM_FILES=$(find "$OUTPUT_DIR" -name "*.wasm" 2>/dev/null || true)

if [[ -z "$WASM_FILES" ]]; then
    echo "Warning: No .wasm files found in $OUTPUT_DIR"
    echo "The wasm-opt post-processing step will need to be done manually:"
    echo "  wasm-opt <your-binary>.wasm -o <your-binary>.exnref.wasm --translate-to-exnref -O2"
else
    echo "Found WASM files for post-processing:"
    for wasm_file in $WASM_FILES; do
        echo "  $wasm_file"

        # Create output filename
        base_name=$(basename "$wasm_file" .wasm)
        dir_name=$(dirname "$wasm_file")
        output_file="$dir_name/$base_name.exnref.wasm"

        echo "  Processing with wasm-opt: $wasm_file -> $output_file"

        # Apply wasm-opt with exception handling translation
        if wasm-opt "$wasm_file" -o "$output_file" --translate-to-exnref -O2; then
            echo "  ✓ Successfully processed $wasm_file"

            # Optionally replace the original file
            if [[ "${REPLACE_ORIGINAL:-}" == "1" ]]; then
                mv "$output_file" "$wasm_file"
                echo "  ✓ Replaced original file with processed version"
            fi
        else
            echo "  ✗ Failed to process $wasm_file"
        fi
    done
fi

echo ""
echo "WASI build completed!"
echo ""
echo "To run your WASM binary with exception handling support:"
echo "  wasmtime run -W exceptions=yes your-binary.exnref.wasm"
echo ""
echo "To run tests (basic features only, async not supported):"
echo "  WASI_SDK=\"$WASI_SDK\" cargo test --target $TARGET --features lua51-wasi,macros,vendored --no-default-features"
echo "  (Note: Many tests require dependencies not available in WASI)"
echo ""
echo "For async support, use wasm32-unknown-emscripten target instead of WASI"
