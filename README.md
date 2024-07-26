# private voting for dummies

a group of people wants to decide something, but doesn't want to share their preferences with each other. they know about zk proofs, and might have even played with them, but... they're looking for a quick result, not a coding project.

# usage (admin)

1. fork this repo into a private repo that will be only for voters.

2. fill in the .env file:
- SP1_PRIVATE_KEY (if you don't have one, follow [these steps](https://docs.succinct.xyz/prover-network/setup.html))
- RPC_URL (testnet is fine, just make sure it's for a chain that has the [SP1VerifierGateway](https://docs.succinct.xyz/onchain-verification/contract-addresses.html) deployed)
- NUM_OPTIONS (the number of options that will be voted on)

3. `just shared-wallet` will generate a new wallet, adding the address and private key to the .env file. it will also print the address along with an estimated amount of ETH to send for gas to cover all txs.

4. `just generate-registration-ids <num_voters>` will add registration ids for each voter to the config to be included in deployment. it will also print a list of secret registration passwords. send one registration password to each voter (privately, so each voter can only register once).

5. deploy the contract using `just deploy`.

# usage (voter)

1. when you receive your registration password, run `just register <registration_password> <made_up_secret_password>` to register to vote.

2. after all users have performed this action, run `just vote <made_up_secret_password> <vote>` to privately cast your vote.

3. at any point, run `just tally` to see the results.

# how does it work?

...
