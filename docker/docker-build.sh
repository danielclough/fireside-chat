#!/bin/bash

# Exit on Error!
set -e

BASE_DIR=$(dirname $(realpath $0 ))
GITHUB_REF=main

# Create a buildkit daemon with the name "multiarch"
export DOCKER_CLI_EXPERIMENTAL=enabled

# Run a build for the different platforms
podman build \
    --build-arg CACHEBUST=`git rev-parse ${GITHUB_REF}` \
    --build-arg FIRESIDE_BACKEND_URL="chat-backend.danielc.us" \
    --build-arg FIRESIDE_DATABASE_URL="chat-database.danielc.us" \
    -t danielclough/fireside-chat-base - \
    --network=host < ${BASE_DIR}/Dockerfile.base
podman build \
    --build-arg CACHEBUST=`git rev-parse ${GITHUB_REF}` \
    -t danielclough/fireside-chat-backend - \
    --network=host < ${BASE_DIR}/Dockerfile.backend
podman build \
    --build-arg CACHEBUST=`git rev-parse ${GITHUB_REF}` \
    -t danielclough/fireside-chat-frontend - \
    --network=host < ${BASE_DIR}/Dockerfile.frontend
podman build \
    --build-arg CACHEBUST=`git rev-parse ${GITHUB_REF}` \
    -t danielclough/fireside-chat-database - \
    --network=host < ${BASE_DIR}/Dockerfile.database