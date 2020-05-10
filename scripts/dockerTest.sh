  
#!/usr/bin/env bash

set -xe

LAUNCH_COMMAND=$(cat ./scripts/runTest.sh)

docker run \
--rm \
--user $(id -u):$(id -g) \
-v $(pwd):/torrentRouter \
-e TERM=xterm \
-e HOME=/torrentRouter \
rust /bin/bash -c "$LAUNCH_COMMAND"
