#!/bin/bash

# Exit on error
set -e

# Set up iOS-specific environment
unset PLATFORM_NAME
unset DEVELOPER_DIR
unset SDKROOT
# export PLATFORM_NAME=iphoneos
export DEVELOPER_DIR="$(xcode-select -p)"
# export SDKROOT="$DEVELOPER_DIR/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk"

# First, make sure we have the targets
rustup target add \
    x86_64-apple-ios \
    aarch64-apple-ios \
    aarch64-apple-ios-sim

# Then, build the library
mkdir -p target/ios

echo "Building for iOS (arm64)..."
cargo build --release \
    --manifest-path="./sifir-ios/Cargo.toml" \
    --target aarch64-apple-ios \
    --target-dir "target/ios"

echo "Building for iOS (x86_64)..."
cargo build --release \
    --manifest-path="./sifir-ios/Cargo.toml" \
    --target x86_64-apple-ios \
    --target-dir "target/ios"

echo "Building for iOS (aarch64-sim)..."
cargo build --release \
    --manifest-path="./sifir-ios/Cargo.toml" \
    --target aarch64-apple-ios-sim \
    --target-dir "target/ios"

# Create universal binary (optional)
echo "Creating universal binary..."
if [ -f "target/ios/aarch64-apple-ios/release/libsifir_ios.a" ] && \
   [ -f "target/ios/x86_64-apple-ios/release/libsifir_ios.a" ]; then
    mkdir -p target/ios/universal
    lipo -create \
        target/ios/aarch64-apple-ios/release/libsifir_ios.a \
        target/ios/x86_64-apple-ios/release/libsifir_ios.a \
        -output target/ios/universal/libsifir_ios.a
    echo "Universal binary created at target/ios/universal/libsifir_ios.a"
else
    echo "Warning: Could not create universal binary. One or both architecture builds are missing."
fi

echo "iOS build complete!"
