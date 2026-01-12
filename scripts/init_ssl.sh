#!/bin/bash
set -e

# This script initializes SSL certificates for the first deployment
# Run this on the server after the first deployment

DOMAIN="jakewray.dev"
EMAIL="admin@jakewray.dev"

echo "Initializing SSL certificates for $DOMAIN..."

# Create dummy certificates for nginx to start
echo "Creating dummy certificates..."
mkdir -p ~/app/certbot/conf/live/$DOMAIN
openssl req -x509 -nodes -newkey rsa:2048 -days 1 \
    -keyout ~/app/certbot/conf/live/$DOMAIN/privkey.pem \
    -out ~/app/certbot/conf/live/$DOMAIN/fullchain.pem \
    -subj "/CN=$DOMAIN"

# Start nginx with dummy certificates
echo "Starting nginx..."
cd ~/app
sudo docker compose -f docker-compose.prod.yml up -d proxy

# Wait for nginx to start
echo "Waiting for nginx to start..."
sleep 5

# Delete dummy certificates
echo "Removing dummy certificates..."
sudo docker compose -f docker-compose.prod.yml exec proxy rm -rf /etc/nginx/ssl/live/$DOMAIN

# Request real certificates
echo "Requesting Let's Encrypt certificates..."
sudo docker compose -f docker-compose.prod.yml run --rm certbot certonly \
    --webroot \
    --webroot-path=/var/www/certbot \
    --email $EMAIL \
    --agree-tos \
    --no-eff-email \
    -d $DOMAIN \
    -d www.$DOMAIN

# Reload nginx to use new certificates
echo "Reloading nginx..."
sudo docker compose -f docker-compose.prod.yml exec proxy nginx -s reload

echo "SSL certificates initialized successfully!"
echo "Certificates will auto-renew via certbot service."
