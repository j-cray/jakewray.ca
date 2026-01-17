#!/bin/bash
set -e

# Configuration
DOCKER_CONFIG_DIR="$HOME/.docker/cli-plugins"
# Determine architecture
ARCH=$(uname -m)
if [ "$ARCH" = "x86_64" ]; then
    BINARY_ARCH="linux-x86_64"
elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
    BINARY_ARCH="linux-aarch64"
else
    echo "Unknown architecture: $ARCH"
    exit 1
fi

PLUGIN_URL="https://github.com/docker/compose/releases/download/v2.24.6/docker-compose-$BINARY_ARCH"
PLUGIN_PATH="$DOCKER_CONFIG_DIR/docker-compose"

echo "Installing Docker Compose Plugin for $ARCH..."
echo "Downloading binary from $PLUGIN_URL..."
curl -SL "$PLUGIN_URL" -o "$PLUGIN_PATH"

# 3. Make executable
chmod +x "$PLUGIN_PATH"

echo "Docker Compose Plugin installed successfully at $PLUGIN_PATH"
docker compose version
