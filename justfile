[private]
@default:
    just --list

@build: install
    bun run tauri build

@dev: install
    bun run tauri dev

@update:
    cargo update --manifest-path src-tauri/Cargo.toml
    bun update

@check:
    cargo clippy --manifest-path src-tauri/Cargo.toml
    bun check

@install:
    bun install

@clean:
    cargo clean --manifest-path src-tauri/Cargo.toml
    rm -rf node_modules dist
