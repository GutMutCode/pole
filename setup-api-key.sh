#!/bin/bash
# Setup script for API keys

echo "🔑 Pole API Key Setup"
echo ""

if [ ! -f .env ]; then
    echo "Creating .env from template..."
    cp .env.example .env
    echo "✓ .env file created"
    echo ""
    echo "Please edit .env and add your API key:"
    echo "  nano .env"
    echo "  or"
    echo "  vim .env"
else
    echo "⚠ .env file already exists"
fi

echo ""
echo "After setting your API key:"
echo "  direnv allow"
echo "  or"
echo "  source .env"
