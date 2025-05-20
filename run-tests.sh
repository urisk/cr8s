#!/bin/bash

# Make sure services are up and running
echo "Starting services..."
docker-compose up -d postgres redis app

# Wait for the API to be fully up
echo "Waiting for API to start..."
sleep 5

# Run the tests
echo "Running tests..."
docker-compose run --rm test cargo test -- --nocapture

# Show the result
echo "Tests completed!"