#!/bin/bash
time ( cargo test --bin main -- --nocapture; echo Exit code: $? )