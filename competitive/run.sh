#!/bin/bash
cargo build --bin main --tests --release
cargo build --bin gen_input --tests --release
time ( target/release/gen_input | target/release/main; echo Exit code: $? )