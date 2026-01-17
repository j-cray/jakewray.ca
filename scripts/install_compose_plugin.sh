#!/bin/bash
set -e

# This script installs/fixes Docker Compose as both a standalone binary and a plugin.
# It handles architecture detection and ensures lowercase names for URLs.

COMPOSE_VERSION="v2.24.6"
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Normalize architecture names
if [ "$ARCH" = "x86_64" ]; then
    BINARY_ARCH="x86_64"
elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
    BINARY_ARCH="aarch64"
else
    echo "Unknown architecture: $ARCH"
    exit 1
fi

BINARY_URL="https://github.com/docker/compose/releases/download/${COMPOSE_VERSION}/docker-compose-${OS}-${BINARY_ARCH}"

echo "Installing Docker Compose ${COMPOSE_VERSION} for ${OS}-${BINARY_ARCH}..."
echo "Downloading from ${BINARY_URL}..."

# 1. Download to /usr/local/bin
sudo curl -SL "${BINARY_URL}" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose

# 2. Link as a CLI plugin in multiple standard paths
sudo mkdir -p /usr/local/lib/docker/cli-plugins
sudo ln -sf /usr/local/bin/docker-compose /usr/local/lib/docker/cli-plugins/docker-compose

sudo mkdir -p /usr/lib/docker/cli-plugins
sudo ln -sf /usr/local/bin/docker-compose /usr/lib/docker/cli-plugins/docker-compose

echo "Docker Compose installed successfully!"
docker-compose version
docker compose version
