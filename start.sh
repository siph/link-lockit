#!/bin/bash
export IP="0.0.0.0"
export DATABASE_URL="postgresql://postgres:password@database/postgres"
export PORT=8080

export DATASOURCE_USERNAME=postgres
export DATASOURCE_PASSWORD=password
export DATASOURCE_DATABASE=postgres

docker-compose up -d
