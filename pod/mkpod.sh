#!/bin/sh

set -e

# Creates a Podman pod running several containers to emulate a CCDC environment
# The `jumpbox` box is intended to actually run Birdhunt, the others are boxes
# to add as hosts in Birdhunt and test features with

ROOT=$(dirname $0)/..

# Make a network for containers to use
podman network exists birdhunt-net
if [ ! $? ]; then
	podman network create --subnet 192.0.2.0/24 birdhunt-net
fi

# Make the pod
# The pod lets containers share a networking namespace. So they can all connect
# to each other with IP addresses.
# podman pod create \
# 	--name birdhunt --replace \
# 	--network birdhunt-net \
# 	--infra-name birdhunt-infra

# Build container images
BASE=$ROOT/pod/containers
podman build -t birdhunt-ubuntu -f $BASE/ubuntu
podman build -t birdhunt-jumpbox -f $BASE/jumpbox

podman run --network birdhunt-net --replace --name birdhunt-ubuntu --ip 192.0.2.3 -dt birdhunt-ubuntu
