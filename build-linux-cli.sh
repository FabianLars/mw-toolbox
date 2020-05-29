#!/bin/bash
cargo update
cd cli
cargo build --release --features server