#!/bin/bash
# Local development setup script

set -e

echo "🚀 Setting up local development environment..."

# Check dependencies
command -v cargo &> /dev/null || { echo "❌ cargo not found. Install Rust from https://rustup.rs/"; exit 1; }
command -v docker &> /dev/null || { echo "❌ docker not found. Please install Docker"; exit 1; }
command -v docker-compose &> /dev/null || { echo "❌ docker-compose not found. Please install Docker Compose"; exit 1; }

echo "✅ All dependencies found"
echo ""

# Start database
echo "📦 Starting PostgreSQL database..."
docker-compose up -d db
sleep 3

echo ""
echo "⏳ Running database migrations..."
cargo sqlx database create || true
cargo sqlx migrate run -D "postgres://admin:password@127.0.0.1:5432/portfolio" || true

echo ""
echo "👤 Creating default admin user..."
PGPASSWORD=password psql -U admin -h 127.0.0.1 -d portfolio -c "INSERT INTO users (username, password_hash) VALUES ('admin', 'admin123') ON CONFLICT (username) DO NOTHING;" || echo "⚠️ Could not create user (may already exist)"

echo ""
echo "✅ Setup complete!"
echo ""
echo "🎯 To run the development server:"
echo ""
echo "   Install cargo-leptos if you haven't:"
echo "   cargo install cargo-leptos"
echo ""
echo "   Then run:"
echo "   cargo leptos watch"
echo ""
echo "📍 Access at:"
echo "   - Frontend: http://localhost:3000"
echo "   - Admin login: http://localhost:3000/admin/login"
echo ""
echo "🔐 Default credentials:"
echo "   Username: admin"
echo "   Password: admin123"
echo ""
echo "🛑 To stop the database:"
echo "   docker-compose down"
