#!/usr/bin/env bash

set -e

NAME=$1

if [ -z "$NAME" ]
then
    echo "Usage:"
    echo "$0 NEW_PROJECT_NAME"
    exit 1
fi

# create the new project
cargo new --vcs none --bin $NAME

# copy in the new templates
cat templates/Cargo.toml >> $NAME/Cargo.toml
cp templates/Xargo.toml $NAME/
cp templates/avr-atmega328p.json $NAME/
cp tools/build.sh $NAME/; chmod 700 tools/build.sh
cp templates/main.rs $NAME/src/
cp templates/.gitignore $NAME/

# create the upload script
cat templates/upload.sh | sed -e "s|__NAME__|$NAME|" > $NAME/upload.sh
chmod 700 $NAME/upload.sh