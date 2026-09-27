#!/bin/sh

ROOT=$(dirname $0)/..
OS=$(cat /etc/os-release | grep '^NAME=')

BIN=$ROOT/target/debug/birdhunt

cargo +nightly build --manifest-path $ROOT/Cargo.toml
if [ "$OS" == "NAME=NixOS" ]; then
	patchelf --remove-rpath --set-interpreter /lib64/ld-linux-x86-64.so.2 --output $BIN.port $BIN
	podman cp $BIN.port birdhunt-jumpbox:/home/ubuntu/birdhunt
else
	podman cp $BIN birdhunt-jumpbox:/home/ubuntu/birdhunt
fi
