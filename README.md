# noir-libs: Noir Package Manager CLI

`noir-libs` is a CLI application for the Noir Package Manager [noir-libs.org](https://noir-libs.org/).
   
## Installation

Install the CLI: `curl -s https://raw.githubusercontent.com/walnuthq/noir-libs/main/install.sh | bash`

Usage: `noir-libs --help`

## Features

The following features are supported:
- **add** a package from the package repository to a project: `noir-libs add <package_name>@<package-version>`. 
  If no version is specified, the latest version will be fetched e.g. `noir-libs add aztec`
- **remove** a package from a project: `noir-libs remove <package_name>`
- **package** a Nargo project to a distributable tarball. Inside Noir project: `noir-libs package`
- **publish** a Nargo package to public remote package registry. Inside Noir project: `noir-libs publish`

### Supported packages

All supported packages are available at [noir-libs.org](https://noir-libs.org/).

## Package locations

Fetched packages are stored in a cache folder. The exact location depends on the operating system:
- Linux: `/home/user/.cache/noir-libs/`
- macOS: `/Users/user/Library/Application Support/com.walnut.noir-libs/`
- Windows: `C:\Users\Alice\AppData\Local\walnut\noir-libs`

## Building from source
Build `noir-libs` using the following command: `cargo build --release`.

## Testing

1. Get the example package `example.tar.gz` from `tests/test_files` and extract it to an unrelated folder.
1. Ensure you have everything set up for [Aztec development](https://docs.aztec.network/guides/getting_started).
1. Try compiling the example with: `aztec-nargo compile`. It should fail due to missing dependencies.
1. Install `noir-libs` by following the instructions in the Installation section above.
1. In your example folder, add the necessary dependencies using the following commands:
    1. `noir-libs add easy_private_state`
    1. `noir-libs add aztec`
    1. `noir-libs add value_note`
1. Compile again with `aztec-nargo compile`
2. Check that everything worked this time.

## Acknowledgments

A lot of inspiration for noir-libs was taken from [scarb](https://github.com/software-mansion/scarb), built by [Software Mansion](https://swmansion.com/).
