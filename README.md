> **Warning**
> This repository is a proof-of-concept only. It has not been extensively
> tested, and is not "production-ready". Use at your own risk.
>
> If you want liquid staking for your ICP, without doing these steps yourself,
> see: https://stakedicp.com

# Canister-Owned-Neurons - Beta

Proof-of-concept for a canister to directly own a neuron on the IC via Threshold ECDSA.

## Architecture

This proof-of-concept has two components.
- A fork of the [quill](https://github.com/AegirFinance/quill) command-line
  tool to create and submit messages to the NNS.
- A `signer` canister which makes Threshold ECDSA requests to sign messages
  on behalf of quill.

The quill fork uses the signer canister to sign it's messages. This means that
the private keys which actually own any created neurons belong to the canister.
Quill then forwards the signed message onto the NNS.

Flow Diagram:

```
┌───────┐ Request    ┌─────────────────┐
│       ├───────────►│                 │
│ Quill │            │ Signer Canister │
│       │◄───────────┤ (Private Key)   │
│       │ Signed Req └─────────────────┘
│       │
│       │ Signed Req ┌─────────────────┐
│       ├───────────►│                 │
│       │            │ NNS Canister    │
│       │◄───────────┤                 │
└───────┘ Response   └─────────────────┘
```

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

For the rest of this document, anytime we use: `./quill local ...`, use
`./quill ic ...` instead.

### Creating a neuron

To create a neuron, we first need to transfer some ICP to the key owned by the
signer canister. We can ask it for it's address, like:

```sh
$ ./quill local public-ids
Principal id: j2nlo-qsl7k-2dydm-2t74w-dtpe2-gbhkl-dsvon-fngvp-r3i7k-6ygiy-bae
Account id: ab15a718e7732da6d6db09bf71a4a923445f21980aca1aefd33a808d549013ff
```

Then, we can transfer some ICP to that Account ID. Note, we must transfer at least
1.0001 ICP to pay for the new neuron, and the ICP transfer fee. An example
using dfx to transfer some from our local wallet is:

```sh
$ dfx ledger transfer \
    --memo 0 \
    --amount 1.00010000 \
    "ab15a718e7732da6d6db09bf71a4a923445f21980aca1aefd33a808d549013ff"
```

At this point we are ready to stake a new neuron. Note, the amount we stake
excludes the extra 0.0001 ICP we sent above for the fee.

```sh
# Save the stake message to stake.json
$ ./quill local neuron-stake --amount 1 --name test > stake.json

# Send the stake message to the network
$ ./quill local send stake.json
```

You can also use any other quill functions.


## Future Work

- Use HTTP outcalls so the canister can call into the governance canister itself.
  - I had a look into this using the rust `ic-agent` crate. However, it has
    some dependencies which make compiling it to wasm (so it can run in a
    canister) very difficult. This approach might be the cleanest, but would
    need some heavy reverse-engineering.
  - It might be worth trying the Javascript `@dfinity/agent` library. That
    would probably be much simpler to get compiling into wasm, but would mean
    we need to use [Azle](https://github.com/demergent-labs/azle) to write the
    canister.

** Ideas and pull-requests are welcome! **
