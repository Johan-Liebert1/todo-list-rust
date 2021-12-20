#!/bin/bash

echo "Building the project...";
cargo build;

sudo mv ./target/debug/todo-list /usr/local/bin

mkdir -p /home/$USER/.todo-data

FILE_PATH=/home/$USER/.todo-data/data.json

if [[ ! -f $FILE_PATH ]]; then
    touch $FILE_PATH
fi
