#/bin/bash

set -e

NETWORK="${1:-local}"

info() {
    printf "💎 Canister Info:\n\n"
    printf "Signer Principal: "
    dfx canister --network $NETWORK --no-wallet call signer get_principal
    printf "Signer Address:"
    dfx canister --network $NETWORK --no-wallet call signer address
}

tests() {
    info
}

tests
