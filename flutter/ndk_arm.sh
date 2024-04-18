#!/usr/bin/env bash
cargo ndk --platform 24 --target armv7-linux-androideabi build --release --features flutter,hwcodec
