#!/bin/bash
docker-compose up -d

while ! nc -z 127.0.0.1 5433; do
    echo "Still waiting for pgadmin ...";
    sleep 5;
done
sleep 5;

./create-pgadmin-connection.sh
