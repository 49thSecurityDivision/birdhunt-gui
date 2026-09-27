#!/bin/sh

# Creates a Podman pod running several containers to emulate a CCDC environment
# The `jumpbox` box is intended to actually run Birdhunt, the others are boxes
# to add as hosts in Birdhunt and test features with

ROOT=$(dirname $0)/..

podman pod create birdhunt # will fail if the pod exists, which we just ignore

# Build container images
BASE=$ROOT/pod/containers
podman build -t birdhunt-ubuntu -f $BASE/ubuntu
podman build -t birdhunt-jumpbox -f $BASE/jumpbox

podman run --replace --pod "birdhunt" --name birdhunt-ubuntu -dt birdhunt-ubuntu
