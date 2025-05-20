#!/bin/bash

# Script to check main.rs and ensure routes are registered correctly

echo "====== Checking main.rs ======"
docker-compose exec app cat /app/src/main.rs

echo ""
echo "====== Checking routes registration ======"
grep -A 20 "rocket::build" /app/src/main.rs || echo "Route registration not found in main.rs"