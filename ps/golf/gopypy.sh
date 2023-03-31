#!/bin/bash
cargo boj test $1 --cmd='python main.py' && cargo boj submit $1 --path=main.py --lang=73 --code-open=n
