[private]
@default:
    just --list

@build: install
    pnpm run tauri build

@dev: install
    pnpm run tauri dev

@update:
    cargo update --manifest-path src-tauri/Cargo.toml
    pnpm update

@check:
    cargo check --manifest-path src-tauri/Cargo.toml
    pnpm check

@install:
    pnpm install

@clean:
    cargo clean --manifest-path src-tauri/Cargo.toml
    rm -rf node_modules dist
