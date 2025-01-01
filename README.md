# Noir package manager

Noir-libs is a package manager to be used with Noir. It retrieves and manages packages.

## Package locations

The fetched packages are cached in a cache folder. Exact location depends on the used operating system:
- Linux: /home/user/.cache/noir-libs/
- macOS: /Users/user/Library/Application Support/com.walnut.noir-libs/
- Windows: C:\Users\Alice\AppData\Roaming\walnut\noir-libs

## How to test

1. Get the example package `example.tar.gz` from `tests/test_files`, extract to some totally unrelated folder
1. Make sure you have everything ready for [Aztec development](https://docs.aztec.network/guides/getting_started)
1. Try to compile the example with `aztec-nargo compile`. See that if fails miserably because of missing dependencies
1. Create an empty folder somewhere for running a local server to act as your package repository
1. Copy the package `packages` from `tests/test_files` to your empty folder
1. Extract the package: `tar -xvf packages`
1. Start a local HTTP server from that folder, for example with `python3 -m http.server 8888`
1. Compile this tool: `cargo build --release`
1. Copy the binary `noir-libs` from `target/release` folder into your example folder
1. Copy the `config.toml` file into your example folder
1. In your example folder, start adding the needed dependencies:
    1. `./noir-libs add easy_private_state@0.67.0`
    1. `./noir-libs add aztec@0.67.0`
    1. `./noir-libs add value_note@0.67.0`
1. Compile again with `aztec-nargo compile`