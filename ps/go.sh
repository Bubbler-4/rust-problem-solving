#!/bin/bash
cargo boj login --bojautologin=$BUB_BOJAUTOLOGIN --onlinejudge=$BUB_ONLINEJUDGE
./test.sh $1 go