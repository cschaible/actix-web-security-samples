#!/bin/bash

set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE USER actix WITH PASSWORD 'secret';
    CREATE DATABASE actix;
    GRANT ALL PRIVILEGES ON DATABASE actix TO actix;
    CREATE USER keycloak WITH PASSWORD 'secret';
    CREATE DATABASE keycloak;
    GRANT ALL PRIVILEGES ON DATABASE keycloak TO keycloak;
EOSQL