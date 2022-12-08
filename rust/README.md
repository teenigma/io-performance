# io-perf-rust

## Cargo
1. Initial cargo
    ```console
    ➜ cargo new
    ```
1. Add dependencies
    ```console
    ➜ cargo add regex
    ```

1. Compile and build
    ```console
    ➜ cargo build [--release]
    ```

1. Execute binary
    ```console
    ➜ cargo run [--release] </path/to/input.txt>
    ```

## Test file
1. Download file
    ```console
    ➜ curl -LO https://www.gutenberg.org/cache/epub/10/pg10.txt
    ```

1. Loop file for 100 rounds
    ```console
    ➜ for i in {1..100}; do cat pg10.txt >>pg10-100.txt; done
    ```
