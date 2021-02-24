#!/usr/bin/env bash

install_fe() {
  git clone https://github.com/substrate-developer-hub/substrate-front-end-template.git --depth 1 ./substrate-front-end-template
  cd ./substrate-front-end-template || {
    echo "missing ./substrate-front-end-template directory"
    exit 1
  }
  local state_add
  state_add=$(jq '.CUSTOM_TYPES |= . + {"State":"u32"}' <./src/config/development.json)
  yarn install
  echo "$state_add" >./src/config/development.json
}

start_fe() {
  cd ./substrate-front-end-template || {
    echo "missing ./substrate-front-end-template directory"
    exit 1
  }
  yarn start
}

case "$1" in
install) install_fe ;;
start) start_fe ;;
*) echo "valid arguments: install, start" ;;
esac
