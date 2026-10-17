#!/bin/bash
set -e
echo "Removing old distribution package..."
rm -rf "./dist"
mkdir -p "./dist/SonicAudioTools-rs/licenses"

echo "Copying new distribution files..."

cp ./target/release/cri-table2json ./dist/SonicAudioTools-rs/cri-table2json
cp ./target/release/csb-editor-rs ./dist/SonicAudioTools-rs/csb-editor-rs

cp ./LICENSE.md ./dist/SonicAudioTools-rs/licenses/LICENSE
cargo install -f copydeps cargo-bundle-licenses
cargo bundle-licenses --format json --output "./dist/SonicAudioTools-rs/licenses/Rust-THIRDPARTY.json"

echo "Creating SHA256SUMS..."
cd dist
find * -type f -exec sha256sum {} \; >> "SHA256SUMS_linux"

7z a "./SonicAudioTools-rs_linux.zip" ./* -mx=9