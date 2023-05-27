#!/bin/bash

cargo build --release
status=$?
if [[ $status != 0 ]]; then
	exit $status
fi

sudo setcap CAP_NET_BIND_SERVICE=eip ./target/release/rdhcp
./target/release/rdhcp
