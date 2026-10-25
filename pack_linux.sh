#!/bin/bash
set -e
echo "Removing old distribution package..."
rm -rf "./dist"
mkdir -p "./dist/CriTableTools/licenses"

echo "Copying new distribution files..."

cp ./target/release/cri-table2json ./dist/CriTableTools/cri-table2json
cp ./target/release/csb-rs ./dist/CriTableTools/csb-rs
cp ./target/release/recreate-table-tester ./dist/CriTableTools/recreate-table-tester
cp ./target/release/recreate-tester ./dist/CriTableTools/recreate-tester

cp ./LICENSE.md ./dist/CriTableTools/licenses/LICENSE
cargo install -f copydeps cargo-bundle-licenses
cargo bundle-licenses --format json --output "./dist/CriTableTools/licenses/Rust-THIRDPARTY.json"

echo "Creating SHA256SUMS..."
cd dist
find * -type f -exec sha256sum {} \; >> "SHA256SUMS_linux"

7z a "./CriTableTools_linux.zip" ./* -mx=9