#!/bin/bash

canister() {
    dfx canister --network ic "$@"
}

echo Exporting identity
IDENTITY_PEM="$(mktemp)"
dfx identity export default > "$IDENTITY_PEM"
echo pem: "$IDENTITY_PEM"

oracle daily \
    --private-pem "$IDENTITY_PEM" \
    --deposits-canister "$(canister id deposits)" \
    --signing-canister "$(canister id signing)" \
    --governance "$(canister id nns-governance)"
