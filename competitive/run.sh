#!/bin/bash
cargo build --bin main --tests --release
time ( cat input.txt | target/release/main; echo Exit code: $? )