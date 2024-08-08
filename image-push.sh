# echo $GHCR_TOKEN | podman login ghcr.io -u $USER_NAME --password-stdin
podman push ghcr.io/contextfreeinfo/taca-dev:latest
