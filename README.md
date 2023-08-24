# Jupiter DCA CPI Integration Example

This repository contains a program that composes Jupiter's DCA program via CPI.

## Dependencies
```
anchor-lang = { version = "0.28.0", features = ["init-if-needed"] }
anchor-spl = { version = "0.28.0" }
jupiter-dca = { git = "https://github.com/jup-ag/dca-cpi", rev = "4c8fcdf" }
```

This repo works out-of-the-box but if you are building a new project from scratch referencing the code here and using Solana v1.16, you may get errors like
```
Invoked an instruction with data that is too large
```

This is a known issue and not specific to Jupiter DCA program or the `jupiter-dca` crate. Read more for details:
https://github.com/solana-labs/solana/issues/31960#issuecomment-1668682153

Every project will have different dependencies so there is no specific guide that can resolve the error.

The approach to take is to take, as of August 2023, is to downgrade your Solana CLI to 1.14 (we built this project against 1.14.19). You may face issues with building. You can resolve the issue by repeatedly running:

```sh
anchor build
cargo update -p <dependency> --precise <lower-version> # based on the error, downgrade the dependencies' versions
# e.g. cargo update -p solana-zk-token-sdk --precise 1.14.19
```

Do this until you build successfully while making sure all your tests still passes.

## To run
```sh
anchor build # dont forget to change the program id if necessary
anchor deploy # or anchor run deploy-mainnet (see `Anchor.toml` to know what it does)
```

## Client code
There are examples on
- Setting up DCA via your program's escrow vault
- Closing DCA Escrow (and getting the tokens back)
- Querying data on the DCA progress (this is more specific to Jupiter's DCA itself and you may refer to https://www.npmjs.com/package/@jup-ag/dca-sdk)

You'll need to make changes to the code
1. Set environment variables in .env for `RPC` and `USER_PRIVATE_KEY`
2. Set the input and output mint as well as amount of input mint
3. DCA parameters like cycle frequencies as well as amount of input mint per cycle

```sh
cd app
npm run example
```
