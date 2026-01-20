#!/bin/bash
# Script to create an admin user

set -e

ADMIN_USERNAME="${1:-admin}"
ADMIN_PASSWORD="${2:-changeme}"
DB_URL="${DATABASE_URL:-postgres://postgres:password@localhost:5432/portfolio}"

echo "Creating admin user: $ADMIN_USERNAME"

# Connect to the database and insert the user (for now, using plain password hash - should use bcrypt in production)
PGPASSWORD="${DB_URL##*:}" psql -U "${DB_URL##*/}" -d "portfolio" -h "${DB_URL#*@}" -c "INSERT INTO users (username, password_hash) VALUES ('$ADMIN_USERNAME', '$ADMIN_PASSWORD') ON CONFLICT DO NOTHING;"

echo "Admin user created successfully!"
echo "Username: $ADMIN_USERNAME"
echo "Password: $ADMIN_PASSWORD"
echo ""
echo "⚠️  WARNING: This is using plain text passwords for now."
echo "In production, implement proper bcrypt hashing!"
