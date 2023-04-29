# Canister-Owned-Neurons - Beta

Proof-of-concept for a canister to directly own a neuron on the IC via Threshold ECDSA.

## Future Work

- Use HTTP outcalls so the canister can call into the governance canister itself.
  - I had a look into this using the rust `ic-agent` crate. However, it has
    some dependencies which make compiling it to wasm (so it can run in a
    canister) very difficult.
  - It might be worth trying the Javascript `@dfinity/agent` library. That
    would probably be much simpler to get compiling into wasm, but would mean
    we need to use [Azle](https://github.com/demergent-labs/azle) to write the
    canister.
