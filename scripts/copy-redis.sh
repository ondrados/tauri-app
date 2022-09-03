#!/bin/bash

REDIS_PATH=$(which redis-server)
TARGET_TRIPLE=$(rustc -Vv | grep host | cut -f2 -d' ')
DESTINATION_PATH="./src-tauri/bin/redis-server-$TARGET_TRIPLE"

mkdir -p "./src-tauri/bin" && cp "$REDIS_PATH" "$DESTINATION_PATH"

echo redis-server copied.”