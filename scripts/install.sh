#!/bin/sh

OWNER=segersniels
REPOSITORY=propr-cli
BINARY=propr
PLATFORM=$(uname)
BIN_DIRECTORY=/usr/local/bin

function determine_platform_binary() {
    case $PLATFORM in
    Linux)
        if [ $(uname -m) = "aarch64" ]; then
        PLATFORM_BINARY="${BINARY}-aarch64-linux"
        else
        PLATFORM_BINARY="${BINARY}-amd64-linux"
        fi
        ;;
    Darwin)
        PLATFORM_BINARY="${BINARY}-macos"
        ;;
    *)
        echo "Unsupported platform: $PLATFORM"
        exit 0
        ;;
    esac
}

function download_binary() {
    url="https://github.com/${OWNER}/${REPOSITORY}/releases/latest/download/${PLATFORM_BINARY}"
    path="${BIN_DIRECTORY}/${BINARY}"

    echo "Downloading ${PLATFORM_BINARY}..."

    if which wget >/dev/null ; then
        wget --quiet -O $path $url
    elif which curl >/dev/null ; then
        curl -s -L $url -o $path
    else
        echo "Unable to download, neither `wget` nor `curl` are available."
    fi

    chmod +x $path

    echo "Installed at ${path}"
}

# Check if we are running as root; if not, try to rerun with sudo.
if [ "$EUID" -ne 0 ] && command -v sudo &>/dev/null; then
    exec sudo -- "$0" "$@"
    exit 0
fi

determine_platform_binary
download_binary
