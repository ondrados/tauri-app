#!/bin/bash

echo "Downloading redis..."
wget https://download.redis.io/redis-stable.tar.gz
tar -xzvf redis-stable.tar.gz
cd redis-stable

echo "Building redis from source..."
make

cd ..

REDIS_PATH="./redis-stable/src/redis-server"
TARGET_TRIPLE=$(rustc -Vv | grep host | cut -f2 -d' ')

if [ $1 = "windows-latest" ]
then
    DESTINATION_PATH="./src-tauri/bin/redis-server-$TARGET_TRIPLE.exe"
else
    DESTINATION_PATH="./src-tauri/bin/redis-server-$TARGET_TRIPLE"
fi


mkdir -p "./src-tauri/bin" && cp "$REDIS_PATH" "$DESTINATION_PATH"

rm -rf ./redis-stable ./redis-stable.tar.gz

echo "redis-server installed"