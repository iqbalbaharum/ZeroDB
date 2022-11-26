
marine build --release

mkdir -p artifacts
rm artifacts/*
cp curl_adapter/target/wasm32-wasi/release/curl_adapter.wasm artifacts/                                   
cp target/wasm32-wasi/release/zero_db.wasm artifacts/

wget https://github.com/fluencelabs/sqlite/releases/download/v0.15.0_w/sqlite3.wasm
mv sqlite3.wasm artifacts/

mrepl Config.toml