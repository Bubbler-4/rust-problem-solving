#!/bin/bash
cargo boj test $1 --cmd='ruby golfscript.rb main.gs' && cargo boj submit $1 --path=main.gs --lang=79 --code-open=n
