binary := "tolist"

# Both mac architectures cross-compile from Apple Silicon with no extra
# tooling: rustup ships the std for each, and Apple's linker handles both.
mac_targets := "aarch64-apple-darwin x86_64-apple-darwin"

# List the available recipes.
default:
    @just --list

# Build the release binary.
build:
    cargo build --release

# Run the release binary, forwarding any arguments.
run *args:
    cargo run --release -- {{ args }}

# Run the test suite, optionally filtered.
test:
    cargo test

# Format the sources.
fmt:
    cargo fmt

# Check formatting without rewriting anything.
fmt-check:
    cargo fmt --check

# Lint with warnings promoted to errors.
lint:
    cargo clippy --all-targets -- -D warnings

# Everything CI should gate on.
check: fmt-check lint test

# Install into ~/.cargo/bin, honouring [profile.release].
install:
    cargo install --path . --force

# Remove the installed binary.
uninstall:
    cargo uninstall {{ binary }}

# Release binaries for every mac architecture, with checksums.
dist: clean-dist
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p dist
    for target in {{ mac_targets }}; do
        echo "building $target"
        rustup target add "$target" >/dev/null 2>&1 || true
        cargo build --release --target "$target"
        cp "target/$target/release/{{ binary }}" "dist/{{ binary }}-$target"
    done
    (cd dist && shasum -a 256 * > SHA256SUMS)
    ls -l dist

clean-dist:
    rm -rf dist

clean: clean-dist
    cargo clean
