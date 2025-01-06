#!/usr/bin/env bash
set -e

echo "Installing your CLI..."

# Variables
TOOL_NAME="noir-libs"
TOOL_HOME=${TOOL_HOME-"$HOME/.noir-libs"}
TOOL_BIN_DIR="$TOOL_HOME/bin"
GITHUB_REPO="walnuthq/noir-libs"
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Determine architecture
case "$ARCH" in
x86_64)
  ARCH="x86_64-unknown-linux-gnu"
  ;;
aarch64 | arm64)
  ARCH="aarch64-apple-darwin"
  ;;
*)
  echo "Unsupported architecture: $ARCH"
  exit 1
  ;;
esac

# Fetch the latest release tag
echo "Fetching the latest release tag from GitHub..."
LATEST_TAG=$(curl -s "https://api.github.com/repos/$GITHUB_REPO/releases/latest" | grep '"tag_name"' | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/')

if [[ -z "$LATEST_TAG" || "$LATEST_TAG" == "null" ]]; then
  echo "Failed to fetch the latest release tag. Please check the repository: $GITHUB_REPO"
  exit 1
fi

echo "Latest release tag is: $LATEST_TAG"

# Construct the download URL
BIN_URL="https://github.com/$GITHUB_REPO/releases/download/$LATEST_TAG/${TOOL_NAME}-${LATEST_TAG}-${ARCH}.tar.gz"

# Create bin directory
mkdir -p "$TOOL_BIN_DIR"

NOIR_LIBS_TOOL_BIN_FINAL_DIR="$TOOL_BIN_DIR/$TOOL_NAME-${LATEST_TAG}-${ARCH}"
NOIR_LIBS_TOOL_BIN_PATH="$NOIR_LIBS_TOOL_BIN_FINAL_DIR/bin/$TOOL_NAME"

# Download and extract the binary
echo "Downloading $TOOL_NAME from $BIN_URL..."
curl -# -L "$BIN_URL" | tar -xz -C "$TOOL_BIN_DIR"

# Make it executable
chmod +x "$NOIR_LIBS_TOOL_BIN_PATH"

# Detect user's shell and update PATH
case $SHELL in
*/zsh)
  PROFILE="$HOME/.zshrc"
  PREF_SHELL="zsh"
  ;;
*/bash)
  PROFILE="$HOME/.bashrc"
  PREF_SHELL="bash"
  ;;
*/fish)
  PROFILE="$HOME/.config/fish/config.fish"
  PREF_SHELL="fish"
  ;;
*)
  echo "Could not detect shell. Please add ${NOIR_LIBS_TOOL_BIN_FINAL_DIR} to your PATH manually."
  exit 1
  ;;
esac

# Remove old entries for NOIR_LIBS_TOOL_BIN_FINAL_DIR and PATH
echo "Cleaning up old entries from $PROFILE..."
sed -i.bak '/# noir-libs - Noir packages manager for Aztec Network/d' "$PROFILE"
sed -i.bak '/export NOIR_LIBS_TOOL_BIN_FINAL_DIR=/d' "$PROFILE"
sed -i.bak '/export PATH=.*\$NOIR_LIBS_TOOL_BIN_FINAL_DIR/d' "$PROFILE"

# Add the new entry
echo "Updating your $PREF_SHELL profile at $PROFILE..."
{
  echo "# noir-libs - Noir packages manager for Aztec Network | noir-libs.org"
  echo "export NOIR_LIBS_TOOL_BIN_FINAL_DIR=\"$NOIR_LIBS_TOOL_BIN_FINAL_DIR\""
  echo "export PATH=\"\$NOIR_LIBS_TOOL_BIN_FINAL_DIR/bin:\$PATH\""
} >>"$PROFILE"

echo
echo "Installation complete!"
echo "Installed noir-libs in $NOIR_LIBS_TOOL_BIN_FINAL_DIR"
echo "Detected your shell as $PREF_SHELL. Run 'source $PROFILE' or start a new terminal session to use $TOOL_NAME."
