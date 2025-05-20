#!/bin/bash

# Script to diagnose database issues and API behavior

echo "====== Database Structure Check ======"
docker-compose exec postgres psql -U postgres -d app_db -c "\d"
echo ""

echo "====== Crates Table Structure ======"
docker-compose exec postgres psql -U postgres -d app_db -c "\d crates"
echo ""

echo "====== Rustaceans Table Structure ======"
docker-compose exec postgres psql -U postgres -d app_db -c "\d rustaceans"
echo ""

echo "====== Enable Debug Logging for Diesel ======"
echo "Set RUST_LOG=debug and restart the app container"
docker-compose down app
docker-compose up -d app
echo ""

echo "====== Testing API Routes Manually ======"
echo "Creating a test rustacean..."
RUSTACEAN_RESULT=$(docker-compose exec -T app curl -s -X POST http://app:8000/rustaceans \
  -H "Content-Type: application/json" \
  -d '{"name":"Test User","email":"test@example.com"}')
echo "Response: $RUSTACEAN_RESULT"

if [ -z "$RUSTACEAN_RESULT" ]; then
  echo "Failed to create rustacean! API might not be running correctly."
  exit 1
fi

RUSTACEAN_ID=$(echo $RUSTACEAN_RESULT | grep -o '"id":[0-9]*' | head -1 | cut -d ":" -f2)
echo "Created rustacean with ID: $RUSTACEAN_ID"

echo "Attempting to create a crate..."
CRATE_RESPONSE=$(docker-compose exec -T app curl -v -X POST http://app:8000/crates \
  -H "Content-Type: application/json" \
  -d "{\"rustacean_id\":$RUSTACEAN_ID,\"code\":\"test\",\"name\":\"Test Crate\",\"version\":\"0.1\",\"description\":\"Test description\"}" 2>&1)

echo "Full crate creation response:"
echo "$CRATE_RESPONSE"

# Clean up
echo "Cleaning up test data..."
docker-compose exec -T app curl -X DELETE http://app:8000/rustaceans/$RUSTACEAN_ID > /dev/null 2>&1

echo "Debug complete!"