//! # Generate ABI Call Data
//! As far as the EVM is concerned, the transaction’s input data (calldata) is
//! just a sequence of bytes. The EVM doesn’t have builtin support for
//! calling methods.
use hex::ToHex;
use tiny_keccak::{Hasher, Sha3};

/// Compute the Keccak-256 hash of input bytes.
fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut hasher = Sha3::v256();
    hasher.update(bytes);
    hasher.finalize(&mut output);
    output
}

/// Returns first 4 bytes of hex output
/// [src](https://medium.com/@hayeah/how-to-decipher-a-smart-contract-method-call-8ee980311603)
fn getter(header: &[u8]) -> Vec<u8> {
    keccak256(header)[..4].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total_supply_getter() {
        let raw_total_supply_getter = getter(b"totalSupply()");
        assert!(raw_total_supply_getter.len() == 4);
        assert_eq!(raw_total_supply_getter.encode_hex::<String>(), "1f1881f8");
    }
}

fn main() {
    // contract call https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/ERC20.sol#L91
    let total_issuance_call = getter(b"totalSupply()").encode_hex::<String>();
    println!(
        "First 4 bytes of Keccak256 of `totalSupply()`:\n{}",
        total_issuance_call
    );
}