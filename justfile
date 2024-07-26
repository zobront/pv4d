set fallback := true
set dotenv-load

default:
  @just --list

generate-registration-ids num_voters:
  #!/usr/bin/env bash
  # for i in {1..num_voters}:
  #   registration_password = random
  #   registration_id=$(cast keccak $registration_password)
  # add ids to config, print list of passwords

shared-wallet:
  #!/usr/bin/env bash
  cast wallet new
  # save address and pk to env
  # print address & print estimate of how much eth to send for gas

deploy:
  #!/usr/bin/env bash
  # probably set up a forge deployment script and just call that from here
  # save contract address to env / config

register registration_password password:
  #!/usr/bin/env bash
  priv_key=$(cast keccak {{password}})
  pub_key=$(cast keccak $priv_key)

  cast send --rpc-url $RPC_URL --private-key $SHARED_PRIVATE_KEY \
    $DEPLOYED_CONTRACT "addPublicKey(string memory,bytes32)" \
    {{registration_password}} $pub_key

vote password vote:
    #!/usr/bin/env bash
    priv_key=$(cast keccak {{password}})
    # nullifier is sha2 of priv key, should go into config for proof

    # cast call to get all public keys and add to config for zkvm run
    cargo run --bin pv4d-script --release
    # above should add proof and stuff to config

    # below should read proof from config
    cast send --rpc-url $RPC_URL --private-key $SHARED_PRIVATE_KEY \
        $DEPLOYED_CONTRACT "vote(bytes32,uint256,bytes memory)" \
        $nullifier {{vote}} $proof

tally:
    #!/usr/bin/env bash
    # vote options is from config
    cast call --rpc-url $RPC_URL $DEPLOYED_CONTRACT "tally(uint256)" $vote_options
    # this returns an array, pretty print it
