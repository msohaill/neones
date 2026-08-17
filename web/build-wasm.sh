#!/usr/bin/env bash

if ! command -v rustup &> /dev/null
then
  echo "Installing Rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  . "$HOME/.cargo/env"
  echo "Rustup installed successfully."
fi

if ! command -v wasm-pack &> /dev/null
then
  echo "Installing wasm-pack..."
  curl https://wasm-bindgen.github.io/wasm-pack/installer/init.sh -sSf | sh
  echo "wasm-pack installed successfully."
fi

rustup --version
wasm-pack --version

rustup target add wasm32-unknown-unknown
wasm-pack build ./neones-web --target web
