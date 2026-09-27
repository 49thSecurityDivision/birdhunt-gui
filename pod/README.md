We test Birdhunt with the Podman container runtime. We create a bunch of containers for several OSes, run them in the same pod (so they can access each other over the network), and run Birdhunt itself from a "jumpbox" container in the pod.

This folder has scripts for setting all of that up.
