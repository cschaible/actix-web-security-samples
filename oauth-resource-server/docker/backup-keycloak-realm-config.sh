#!/bin/bash

docker exec -it docker_keycloak_1 /opt/jboss/keycloak/bin/standalone.sh \
-v /tmp:/tmp/keycloak-export\
-Djboss.socket.binding.port-offset=100 -Dkeycloak.migration.action=export \
-Dkeycloak.migration.provider=singleFile \
-Dkeycloak.migration.realmName=app \
-Dkeycloak.migration.usersExportStrategy=REALM_FILE \
-Dkeycloak.migration.file=/tmp/app-realm.json

docker cp docker_keycloak_1:/tmp/app-realm.json ./app-realm-backup.json