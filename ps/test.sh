#!/bin/bash
cargo oj
cargo boj test $1 -p && if [ "$2" = 'go' ]; then
    cargo boj submit $1
else
    echo 'Test successful, but not submitted.'
fi
