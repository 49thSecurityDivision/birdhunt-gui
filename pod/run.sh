#!/bin/sh

ROOT=$(dirname $0)/..

podman run --pod birdhunt \
	--name birdhunt-jumpbox --replace \
	-e "XDG_RUNTIME_DIR=$XDG_RUNTIME_DIR" -e "WAYLAND_DISPLAY=$WAYLAND_DISPLAY" \
	-v $XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY -dt \
	birdhunt-jumpbox

$ROOT/pod/install-birdhunt.sh

podman exec -d birdhunt-jumpbox /home/ubuntu/birdhunt
