//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();

    // read all public keys from config into Vec<B256>
    // read personal private key from config into B256
    stdin.write(2);

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let proof = prover.prove(&pk, sp1_stdin).plonk().run().unwrap();

    // Save solidity proof to config
    let proof = proof.bytes();
    let solidity_proof = proof
        .iter()
        .fold(String::with_capacity(proof.len() * 2), |mut acc, b| {
            write!(acc, "{:02x}", b).unwrap();
            acc
        });

    println!("successfully generated and verified proof for the program!")
}
