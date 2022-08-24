#!/bin/bash
cargo build --bin main --tests --release
time ( BOJ=$1 cargo test --test test --release -- --nocapture; echo Exit code: $? )