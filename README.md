# Canister-Owned-Neurons - Beta

Proof-of-concept for a canister to directly own a neuron on the IC via Threshold ECDSA.

## Usage

### Install Dependencies

This repository depends on a special fork of quill, which adds a new CLI flag.
The new cli flag is called `--canister-id`, and enables quill to use a remote
canister to sign messages via Threshold ECDSA.

You can download the needed version of quill by running:

```sh
./scripts/install_quill.sh
```

### Local Setup

To test this repository locally, first we need dfx running.

```sh
$ dfx start
```

Then, we need to deploy the signer canister, which will own the private key.

```sh
$ ./scripts/deploy.sh local
```

Then, we can use the provided quill wrapper script:

```sh
$ ./quill local public-ids
```

### IC Setup

First, we need to deploy the signer canister, which will own the private key.

```sh
$ ./scripts/deploy.sh ic reinstall key_1
```

Then, we can use the provided quill wrapper script:

```sh
$ ./quill ic public-ids
```

### Creating a neuron

To create a neuron, we first need to transfer some ICP to the key owned by the signer canister. We can ask it for it's address, like:

```sh
$ dfx canister call signer address
("c3d69b64bc40e92e1554fb5c0fb289d72d7faa7207dbc423fe68236566e1f581")
```

Then, we can transfer some ICP to that address. Note, we must transfer at least
1.0001 ICP to pay for the new neuron, and the ICP transfer fee.

```sh
$ dfx ledger transfer --memo 0 --amount 1.00010000 "c3d69b64bc40e92e1554fb5c0fb289d72d7faa7207dbc423fe68236566e1f581"
```

At this point we are ready to stake a new neuron. Note, the amount here
excludes the extra 0.0001 ICP we sent above for the fee.

```sh
# Save the stake message to stake.json
$ ./quill local neuron-stake --amount 1 --name test > stake.json

# Send the stake message to the network
$ ./quill local send stake.json
```



## Future Work

- Use HTTP outcalls so the canister can call into the governance canister itself.
  - I had a look into this using the rust `ic-agent` crate. However, it has
    some dependencies which make compiling it to wasm (so it can run in a
    canister) very difficult.
  - It might be worth trying the Javascript `@dfinity/agent` library. That
    would probably be much simpler to get compiling into wasm, but would mean
    we need to use [Azle](https://github.com/demergent-labs/azle) to write the
    canister.
