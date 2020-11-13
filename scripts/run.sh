#!/bin/sh

RUST_LOG="app=debug,wishlist=debug" cargo run --release --bin app
