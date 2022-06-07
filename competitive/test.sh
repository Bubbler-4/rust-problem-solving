#!/bin/bash
cargo build --bin main --tests --release
time ( cargo test --bin main --release -- --nocapture; echo Exit code: $? )