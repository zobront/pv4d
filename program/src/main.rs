//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_primitives::B256;
use sha2_v0_10_8::{Sha256, Digest};
use tiny_keccak::Sha3;

pub fn main() {
    // read the list of public keys and the private key
    let pubkeys = sp1_zkvm::io::read::<Vec<B256>>();
    let private_key = sp1_zkvm::io::read::<B256>();

    // check that the keccak of the private key is in the list of public keys
    let mut sha3 = Sha3::v256();
    sha3.update(private_key);
    let mut output = [0u8; 32];
    sha3.finalize(&mut output);
    assert(pubkeys.contains(&B256::from(output)));

    // check that the passed nullifier is the sha2 of the private key
    let mut hasher = Sha256::new();
    let data = private_key.as_bytes();
    hasher.update(data);
    let nullifier = hasher.finalize();

    // return the list of public keys and the nullifier
    sp1_zkvm::io::commit::<Vec<B256>>(&pubkeys);
    sp1_zkvm::io::commit(&nullifier);
}
