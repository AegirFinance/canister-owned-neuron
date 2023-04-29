#!/bin/bash

set -e

if [[ "$1" == "-h" ]] || [[ "$1" == "--help" ]]; then
    printf "💎 Deploy Script:\n\n   usage: deploy <local|ic|other> [reinstall|upgrade] [dfx_test_key|test_key_1|key_1]\n\n"
    exit 1;
fi

NETWORK="${1:-local}"
MODE="${2:-reinstall}"
KEY_ID="${3:-dfx_test_key}"
OWNER="$(dfx identity get-principal)"

if [ -n "$MODE" ]; then
  MODE="--mode $MODE"
fi

dfx deploy --no-wallet --network $NETWORK signer \
	--argument="(record { owners=vec {principal \"$OWNER\"}; key_id=\"$KEY_ID\";})" \
    $MODE
