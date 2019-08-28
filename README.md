# Shokunin August 2019

## Summary

This solution uses a simple Hill-Climbing algorithm to explore
the space of the potential rankings and find a match:
https://en.wikipedia.org/wiki/Hill_climbing

Given the size of the developer set and predicates in the brief,
a matching ranking is normally found within one second.

Rust was used as a way to get familiar with the language.

## Prerequisites

- Docker or,
- Cargo plus the Rust compiler.

## Building, running the tests and running the solution

    docker run -it \
        -v "$PWD:/tmp/project" \
        -w /tmp/project \
        rust:1.35-slim \
        sh -c 'cargo test && cargo run'

or:

    cargo test && cargo run
