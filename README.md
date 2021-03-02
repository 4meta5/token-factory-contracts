# token-factory-contracts

The Solidity contract gives the caller the `MINTER_ROLE` and `BURNER_ROLE` to mint and burn ERC20 tokens (and delegate this right to other accounts).

The Rust code computes the format for calls to a local EVM instance.

References:
* https://docs.soliditylang.org/en/v0.8.1/abi-spec.html
* https://medium.com/@hayeah/how-to-decipher-a-smart-contract-method-call-8ee980311603
* https://emn178.github.io/online-tools/keccak_256.html