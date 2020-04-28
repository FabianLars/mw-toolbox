#!/bin/bash
cd cli
RUSTFLAGS='-C link-arg=-s' cargo build --release --features server