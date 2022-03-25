#!/bin/bash
cargo build --bin main --release
time ( cat input.txt | target/release/main; echo Exit code: $? )