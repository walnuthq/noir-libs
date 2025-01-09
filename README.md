# Noir package manager

Noir-libs is a package manager for [Noir](https://noir-lang.org/). Explore available packages at [noir-libs.org](https://noir-libs.org/).

## CLI installation

Install CLI: `curl -s https://raw.githubusercontent.com/walnuthq/noir-libs/main/install.sh | bash`

Usage: `noir-libs --help`

## Features

The following features are supported:
- Add a package, from the package repository, to a project: `noir-libs add <package_name>@<package-version>`. 
  If no version is specified, the latest version will be fetched e.g. `noir-libs add aztec`
- Remove a package from a project: `noir-libs remove <package_name>`

### Supported packages

`noir-libs` is integrated with [noir-libs.org](https://noir-libs.org/). Explore available packages there. 

## Package locations

The fetched packages are stored in a cache folder. Exact location depends on the used operating system:
- Linux: `/home/user/.cache/noir-libs/`
- macOS: `/Users/user/Library/Application Support/com.walnut.noir-libs/`
- Windows: `C:\Users\Alice\AppData\Local\walnut\noir-libs`

## Building
`noir-libs` can be built with a command `cargo build --release`.

## Testing

1. Get the example package `example.tar.gz` from `tests/test_files`, extract to some totally unrelated folder
1. Make sure you have everything ready for [Aztec development](https://docs.aztec.network/guides/getting_started)
1. Try to compile the example with `aztec-nargo compile`. See that if fails miserably because of missing dependencies
1. Install this tool by following the instructions above
1. In your example folder, start adding the needed dependencies:
    1. `noir-libs add easy_private_state`
    1. `noir-libs add aztec`
    1. `noir-libs add value_note`
1. Compile again with `aztec-nargo compile`