#!/bin/bash

canister() {
    dfx canister --network ic "$@"
}

KEY="$(dfx identity export default)"

oracle daily \
    --private-pem <(echo "$KEY") \
    --deposits-canister "$(canister id deposits)" \
    --signing-canister "$(canister id signing)" \
    --governance "$(canister id nns-governance)"
