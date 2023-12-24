#!/bin/sh

BINARY=propr
PLATFORM=$(uname)

case $PLATFORM in
Darwin)
    PLATFORM_BINARY="${BINARY}-macos"
    ;;
*)
    echo "Unsupported platform: $PLATFORM"
    exit 0
    ;;
esac

# Determine where on the filesystem the script is located, since it is most likely symlinked
LOCATION_ON_FILE_SYSTEM=$(dirname $([ -L $0 ] && readlink -f $0 || echo $0))
EXEC_BINARY=$LOCATION_ON_FILE_SYSTEM/../bin/$PLATFORM_BINARY

# Ensure the binary is executable
chmod +x $EXEC_BINARY

exec $EXEC_BINARY "$@"
