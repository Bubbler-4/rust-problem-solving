#!/bin/bash
cargo oj
if cargo boj test $1 && [ "$2" = 'go' ]; then
    cargo boj submit $1
else
    echo 'Test successful, but not submitted.'
fi
