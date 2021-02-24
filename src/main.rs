//! # Compute Method Ids
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
fn method_id(header: &[u8]) -> Vec<u8> {
    keccak256(header)[..4].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total_supply_getter() {
        let raw_total_supply_getter = method_id(b"totalSupply()");
        assert!(raw_total_supply_getter.len() == 4);
        assert_eq!(raw_total_supply_getter.encode_hex::<String>(), "1f1881f8");
    }
    #[test]
    fn balance_of_getter() {
        let raw_total_supply_getter = method_id(b"balanceOf(address)");
        assert!(raw_total_supply_getter.len() == 4);
        assert_eq!(raw_total_supply_getter.encode_hex::<String>(), "1d7976f3");
    }
    #[test]
    fn mint_setter() {
        let raw_total_supply_getter = method_id(b"mint(address,uint256)");
        assert!(raw_total_supply_getter.len() == 4);
        assert_eq!(raw_total_supply_getter.encode_hex::<String>(), "9cff1ade");
    }
    #[test]
    fn burn_setter() {
        let raw_total_supply_getter = method_id(b"burn(address,uint256)");
        assert!(raw_total_supply_getter.len() == 4);
        assert_eq!(raw_total_supply_getter.encode_hex::<String>(), "4f10869a");
    }
}

fn main() {
    let total_issuance_call = method_id(b"totalSupply()").encode_hex::<String>();
    println!(
        "First 4 bytes of Keccak256 of `totalSupply()`:\n{}",
        total_issuance_call
    );
    let balance_of_call = method_id(b"balanceOf(address)").encode_hex::<String>();
    println!(
        "First 4 bytes of Keccak256 of `balanceOf(address)`:\n{}",
        balance_of_call
    );
    let mint_call = method_id(b"mint(address,uint256)").encode_hex::<String>();
    println!(
        "First 4 bytes of Keccak256 of `mint(address,uint256)`:\n{}",
        mint_call
    );
    let burn_call = method_id(b"burn(address,uint256)").encode_hex::<String>();
    println!(
        "First 4 bytes of Keccak256 of `burn(address,uint256)`:\n{}",
        burn_call
    );
}