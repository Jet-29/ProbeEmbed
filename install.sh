#!/bin/bash
cargo build --release
rsync -r --delete target/release/probe_embed Arachnid:embedded/
