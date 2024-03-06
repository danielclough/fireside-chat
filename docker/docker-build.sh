#!/bin/bash

# Exit on Error!
set -e

BASE_DIR=$(dirname $(realpath $0 ))

# Create a buildkit daemon with the name "multiarch"
export DOCKER_CLI_EXPERIMENTAL=enabled
docker buildx install
docker buildx create \
    --use \
    --name=multiarch \
    --node=multiarch \
    --bootstrap \
    --buildkitd-flags '--allow-insecure-entitlement network.host'

# Install QEMU
docker run --rm --privileged \
    multiarch/qemu-user-static --reset -p yes

# Run a build for the different platforms
docker buildx build --push --platform=linux/amd64 -t danielclough/fireside-chat-base - --network=host < ${BASE_DIR}/Dockerfile.base
docker buildx build --push --platform=linux/amd64 -t danielclough/fireside-chat-backend - --network=host < ${BASE_DIR}/Dockerfile.backend
docker buildx build --push --platform=linux/amd64 -t danielclough/fireside-chat-frontend - --network=host < ${BASE_DIR}/Dockerfile.frontend
docker buildx build --push --platform=linux/amd64 -t danielclough/fireside-chat-database - --network=host < ${BASE_DIR}/Dockerfile.database