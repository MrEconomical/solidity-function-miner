# Solidity Function Miner

A fast and lightweight tool for mining Solidity function selector hashes with zero bytes. Useful for micro-optimizing gas usage in Solidity smart contracts by saving gas with zero bytes in function calldata.

## Usage

Running the built and optimized executable in `target/release` is much faster than running with `cargo run --release`.

```
solidity-function-miner
    <function name>         Name of Solidity function to mine selectors for
    <function parameters>   Parameter list of function without spaces or abbreviations
    <zero byte target>      Number of zero bytes to mine
    <thread count>          Number of threads to mine on
```