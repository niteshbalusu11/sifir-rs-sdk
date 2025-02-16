#!/bin/bash

# Exit on error
set -e

# Set up macOS-specific environment
unset SDKROOT
unset PLATFORM_NAME
unset DEVELOPER_DIR
export DEVELOPER_DIR="$(xcode-select -p)"
export SDKROOT="$(xcrun --sdk macosx --show-sdk-path)"
# export MACOSX_DEPLOYMENT_TARGET="11.0"

# First, make sure we have the targets
rustup target add \
    aarch64-apple-darwin \
    x86_64-apple-darwin

# Create output directory
mkdir -p target/macos

# Build for Apple Silicon
echo "Building for Apple Silicon (arm64)..."
cargo build --release \
    --manifest-path="./sifir-ios/Cargo.toml" \
    --target aarch64-apple-darwin \
    --target-dir "target/macos"

# Build for Intel
echo "Building for Intel (x86_64)..."
cargo build --release \
    --manifest-path="./sifir-ios/Cargo.toml" \
    --target x86_64-apple-darwin \
    --target-dir "target/macos"

# Create universal binary (optional)
echo "Creating universal binary..."
if [ -f "target/macos/aarch64-apple-darwin/release/libsifir_ios.a" ] && \
   [ -f "target/macos/x86_64-apple-darwin/release/libsifir_ios.a" ]; then
    mkdir -p target/macos/universal
    lipo -create \
        target/macos/aarch64-apple-darwin/release/libsifir_ios.a \
        target/macos/x86_64-apple-darwin/release/libsifir_ios.a \
        -output target/macos/universal/libsifir_ios.a
    echo "Universal binary created at target/macos/universal/libsifir_ios.a"
else
    echo "Warning: Could not create universal binary. One or both architecture builds are missing."
fi

echo "macOS build complete!"
