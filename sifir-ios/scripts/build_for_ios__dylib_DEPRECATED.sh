#!/bin/bash
echo "---Sifir.io----";
echo "---------------";
echo "|              |";
echo "|      0       |";
echo "|              |";
echo "---------------";
echo "Will build a universal IOS dylib !!";
echo "---------------";
echo "---------------";

# Always build release now since ssl throws error on anything else now
target="release";
#if [[ "$1" == "release" ]]; then
#	target=$1;
#	echo "### Building Release ###"
#fi

# Build local (+ FFI)
cargo  build -p sifir-ios --"$target";

export IPHONEOS_DEPLOYMENT_TARGET="11.0"

#cargo +nightly build -p sifir-ios --target x86_64-apple-ios --"$target";
#cargo +nightly build -p sifir-ios --target aarch64-apple-ios --"$target";
cargo  build -p sifir-ios --target x86_64-apple-ios --"$target";
retVal=$?
[ ! $retVal -eq 0 ] && exit 1;
cargo  build -p sifir-ios --target aarch64-apple-ios --"$target";
retVal=$?
[ ! $retVal -eq 0 ] && exit 1;

mkdir -p ../output/"$target"/{universal,aarch64-apple-ios,x86_64-apple-ios};

# copy indiviual arch libs  for testing
\cp -f ../../target/aarch64-apple-ios/"$target"/libsifir_ios.dylib ../output/"$target"/aarch64-apple-ios/libsifir_ios.dylib
\cp -f ../../target/x86_64-apple-ios/"$target"/libsifir_ios.dylib ../output/"$target"/x86_64-apple-ios/libsifir_ios.dylib

# create universal lb
lipo -create ../../target/aarch64-apple-ios/"$target"/libsifir_ios.dylib ../../target/x86_64-apple-ios/"$target"/libsifir_ios.dylib -output ../output/"$target"/universal/libsifir_ios.dylib

[ ! $retVal -eq 0 ] && exit 1;

# Update dylib rpath
install_name_tool -id "@rpath/libsifir_ios.dylib" ../output/"$target"/universal/libsifir_ios.dylib

# Output sizes
du -d 1 -h ../output/"$target"

echo "Done!":

