[private]
@default:
    just --list

@build: install
    npm run tauri build

@dev: install
    npm run tauri dev

@update:
    cargo update --manifest-path src-tauri/Cargo.toml
    npm update

@install:
    npm install

@clean:
    cargo clean --manifest-path src-tauri/Cargo.toml
    rm -rf node_modules dist
