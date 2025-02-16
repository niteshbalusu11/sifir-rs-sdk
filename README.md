# sifir-rs-sdk

Builds a universal dyanamic library for iOS and a static library for Android.

## Supported platforms

* Android through the NDK (API level 30+)
* MacOS
* iOS

### Build using nix on a Mac
- Install [nix](https://determinate.systems/nix-installer/)
- Install [direnv](https://direnv.net/)
- Run `direnv allow` to allow direnv to load the nix environment
- If you want to install xcode and xcode command line tools, simply run `setup-ios-env`.

```
# Build Android
build-android

# Build iOS
build-ios

# Build MacOS
build-macos

# Build all
build-all
```
