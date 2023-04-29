#!/bin/bash

set -e

DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

if [[ "$1" == "-h" ]] || [[ "$1" == "--help" ]]; then
    printf "💎 Install Quill: Install the fork of quill which supports canister thresholdEcdsa signing\n\n   usage: install_quill\n\n"
    exit 1;
fi

cargo install --locked --git https://github.com/AegirFinance/quill --root "$DIR/.." "$@"
