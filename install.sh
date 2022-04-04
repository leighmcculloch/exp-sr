#!/bin/sh


installdir="${SJC_INSTALL:-$HOME/.bin}"
installloc="$installdir/sjc"
version="0.2.0"
sys="$(uname -sm)"
bin=
case "$sys" in
  "Linux x86_64") bin=sjc-linux-amd64 ;;
  "Darwin x86_64") bin=sjc-macos-amd64;;
  "Darwin arm64") bin=sjc-macos-arm64 ;;
  *) echo "Unrecognized operating system or architecture: $sys"; exit 1 ;;
esac

mkdir -p "$installdir"
curl -sSL -o "$installloc" "https://github.com/leighmcculloch/sjc/releases/download/$version/$bin"
chmod +x "$installloc"

echo "Installed to $installloc."
echo "Add $installdir to your PATH."
