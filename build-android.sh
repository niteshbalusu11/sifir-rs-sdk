#!/bin/bash

# Exit on error
set -e

# Unset any iOS/macOS specific variables that might interfere
unset SDKROOT
unset PLATFORM_NAME
unset IPHONEOS_DEPLOYMENT_TARGET
unset TVOS_DEPLOYMENT_TARGET
unset XROS_DEPLOYMENT_TARGET

# First, make sure we have the targets
rustup target add \
    aarch64-linux-android \
    x86_64-linux-android \
    i686-linux-android

# Create output directory
mkdir -p target/android

# Then, build the library
echo "Building for Android..."
cargo ndk \
    --manifest-path="./sifir-android/Cargo.toml" \
    --platform 30 \
    -t arm64-v8a \
    -t x86 \
    -t x86_64 \
    --output-dir "target/android" \
    build --release \

echo "Android build complete!"
