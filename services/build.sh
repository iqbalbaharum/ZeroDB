#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

# This script builds all subprojects and puts all created Wasm modules in one dir
cd ed25519
cargo update --aggressive
marine build --release

cd ../ipfs
cargo update --aggressive
marine build --release

cd ../dht
cargo update --aggressive
marine build --release

cd ../data
cargo update --aggressive
marine build --release

cd ../facade
cargo update --aggressive
marine build --release

cd ..
mkdir -p artifacts
rm -f artifacts/*.wasm
cp target/wasm32-wasi/release/fdb_ed25519.wasm artifacts/
cp target/wasm32-wasi/release/fdb_ipfs.wasm artifacts/
cp target/wasm32-wasi/release/fdb_dht.wasm artifacts/
cp target/wasm32-wasi/release/fdb_data.wasm artifacts/
cp target/wasm32-wasi/release/fdb_facade.wasm artifacts/
marine aqua artifacts/fdb_facade.wasm -s Fdb -i fdb > ../aqua/aqua/fdb.aqua

wget https://github.com/fluencelabs/sqlite/releases/download/v0.15.0_w/sqlite3.wasm
mv sqlite3.wasm artifacts/

RUST_LOG="info" mrepl --quiet Config.toml