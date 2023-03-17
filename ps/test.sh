#!/bin/bash
cargo oj
cargo build --bin main --tests --release
BOJ=$1 cargo test --test test --release -- --nocapture && cargo boj submit $1