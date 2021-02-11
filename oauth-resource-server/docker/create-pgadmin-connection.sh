#!/bin/bash

docker cp ./servers.json pgadmin:/tmp/servers.json
docker exec pgadmin python /pgadmin4/setup.py --load-servers /tmp/servers.json --user "administr@t.r"
