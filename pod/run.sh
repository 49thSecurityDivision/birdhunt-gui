#!/bin/sh

set -e

ROOT=$(dirname $0)/..

podman run \
	--name birdhunt-jumpbox --replace \
	--network birdhunt-net --ip 192.0.2.2 \
	-e "XDG_RUNTIME_DIR=$XDG_RUNTIME_DIR" -e "WAYLAND_DISPLAY=$WAYLAND_DISPLAY" \
	-v $XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY -dt \
	--cap-add=NET_RAW \
	birdhunt-jumpbox

$ROOT/pod/install-birdhunt.sh

podman exec -d birdhunt-jumpbox /home/ubuntu/birdhunt
