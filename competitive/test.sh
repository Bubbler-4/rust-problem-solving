#!/bin/bash
time ( cargo test --bin boj -- --nocapture; echo Exit code: $? )