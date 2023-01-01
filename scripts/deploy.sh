#!/bin/bash

set -e

if [[ -z $1 ]]; then
    printf "💎 Deploy Script:\n\n   usage: deploy <local|ic|other> [reinstall|upgrade]\n\n"
    exit 1;
fi

NETWORK="${1:-local}"
MODE=$2

if [ -n "$MODE" ]; then
  MODE="--mode $MODE"
fi

dfx deploy --no-wallet --network $NETWORK neuron \
	--argument="(principal \"$(dfx identity get-principal)\")" \
    $MODE
