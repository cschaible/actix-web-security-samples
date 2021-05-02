#!/bin/bash

docker cp ./servers.json pgadmin:/tmp/servers.json
docker exec pgadmin /venv/bin/python3 /pgadmin4/setup.py --load-servers /tmp/servers.json --user "administr@to.rs"
