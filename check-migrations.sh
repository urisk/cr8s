#!/bin/bash

# This script checks and applies database migrations

echo "Checking database connection..."
docker-compose exec app diesel database setup

echo "Printing database schema structure..."
docker-compose exec app diesel print-schema

echo "Running migrations..."
docker-compose exec app diesel migration run

echo "Migration status:"
docker-compose exec app diesel migration list