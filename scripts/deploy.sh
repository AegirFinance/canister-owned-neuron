#!/bin/bash

set -e

if [[ -z $1 ]]; then
    printf "💎 Deploy Script:\n\n   usage: deploy <local|ic|other> [install|reinstall|upgrade] [local_test_key|test_key_1|key_1]\n\n"
    exit 1;
fi

NETWORK="${1:-local}"
MODE="${2:-install}"
KEY_ID="${3:-test_key_1}"

if [ -n "$MODE" ]; then
  MODE="--mode $MODE"
fi

dfx deploy --no-wallet --network $NETWORK signer \
	--argument="(record { owners=vec {principal \"$(dfx identity get-principal)\"}; key_id=\"$KEY_ID\")" \
    --mode=$MODE
