#!/bin/sh

docker run -it \
        -v "$PWD:/tmp/project" \
        -w /tmp/project \
        rust:1.35-slim \
        sh -c 'cargo test && cargo run'

