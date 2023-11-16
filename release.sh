#!/usr/bin/env bash

# Variables
bin=target/release/html-cleaner.exe
version=$(grep '^version =' Cargo.toml | awk '{print $3}' | sed 's/"//g')
release_dir="/d/temp/rust-utils/html-cleaner/v${version}"


# Remove the target dir if exists.
rm -rf "${release_dir}" 2>/dev/null

# Make the dir.
mkdir -p "${release_dir}" 2>/dev/null

cargo build --release
cp "${bin}" "${release_dir}"
cp "${bin}" ~/bin
# Copy over all the md files.
cp *.md "${release_dir}"
