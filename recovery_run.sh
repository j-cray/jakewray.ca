#!/bin/bash
set -e

echo "Starting recovery launch sequence..."

# Cleanup
echo "Cleaning up old containers..."
podman rm -f db portfolio nginx certbot || true

# Network
podman network create jake_net || true

# DB
echo "Starting DB..."
# Explicitly setting credentials found in .env (or implied by dev setup)
podman run -d --name db \
    --network jake_net \
    -e POSTGRES_USER=admin \
    -e POSTGRES_PASSWORD=password \
    -e POSTGRES_DB=portfolio \
    -v db_data:/var/lib/postgresql/data \
    docker.io/library/postgres:15-alpine

# Portfolio
echo "Starting Portfolio..."
# Override DATABASE_URL to use container hostname 'db' instead of 'localhost'
podman run -d --name portfolio \
    --network jake_net \
    -e LEPTOS_SITE_ADDR=0.0.0.0:3000 \
    -e DATABASE_URL="postgres://admin:password@db:5432/portfolio" \
    -e RUST_LOG=debug \
    -v $(pwd)/media_mount:/app/media \
    jakewray_portfolio

# Nginx
echo "Starting Nginx..."
podman run -d --name nginx \
    --network jake_net \
    -p 8080:80 -p 8443:443 \
    -v $(pwd)/nginx/nginx_http_only.conf:/etc/nginx/conf.d/default.conf \
    -v $(pwd)/certbot/conf:/etc/letsencrypt \
    -v $(pwd)/certbot/www:/var/www/certbot \
    docker.io/library/nginx:stable-alpine

echo "All containers started."
podman ps
