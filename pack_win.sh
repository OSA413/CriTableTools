#!/bin/bash
set -e
echo "Removing old distribution package..."
rm -rf "./dist"
mkdir -p "./dist/CriTableTools/licenses"

echo "Copying new distribution files..."

cp ./target/release/cri-table2json.exe ./dist/CriTableTools/cri-table2json.exe
cp ./target/release/csb-editor-rs.exe ./dist/CriTableTools/csb-editor-rs.exe

cp ./LICENSE.md ./dist/CriTableTools/licenses/LICENSE
cargo install -f copydeps cargo-bundle-licenses
cargo bundle-licenses --format json --output "./dist/CriTableTools/licenses/Rust-THIRDPARTY.json"

echo "Creating SHA256SUMS..."
cd dist
find * -type f -exec sha256sum {} \; >> "SHA256SUMS_win"

7z a "./CriTableTools_win.zip" ./* -mx=9