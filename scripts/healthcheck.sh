#/bin/bash

set -e

NETWORK="${1:-local}"

info() {
    printf "💎 Canister Info:\n\n"
    printf "Owner: "
    dfx canister --network $NETWORK --no-wallet call neuron owner
    printf "Neuron ID: "
    dfx canister --network $NETWORK --no-wallet call neuron neuronId
}

tests() {
    info
}

tests
