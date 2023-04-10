#!/bin/bash
cargo boj login --bojautologin=$SUS_BOJAUTOLOGIN --onlinejudge=$SUS_ONLINEJUDGE
./test.sh $1 go