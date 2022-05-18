#!/bin/bash
cargo build --bin interactive --release
target/release/interactive; echo Exit code: $?