#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

RUNE_PI_HOST=${RUNE_PI_HOST:-user1@rune-proto1.local}
RUNE_PI_DIR=${RUNE_PI_DIR:-/home/user1/src/rune/firmware/userspace/rune-ui}
RUNE_PI_BIN=${RUNE_PI_BIN:-/usr/local/bin/rune-ui}
RUNE_PI_SERVICE=${RUNE_PI_SERVICE:-rune-ui.service}

case "$RUNE_PI_DIR" in
  /Users/*)
    echo "RUNE_PI_DIR is set to a macOS path: $RUNE_PI_DIR" >&2
    echo "Use a path on the Pi, for example:" >&2
    echo "  RUNE_PI_DIR=/home/user1/src/rune/firmware/userspace/rune-ui ./scripts/pi-deploy.sh" >&2
    exit 2
    ;;
esac

"$SCRIPT_DIR/pi-sync.sh"

echo "building and restarting $RUNE_PI_SERVICE on $RUNE_PI_HOST"

ssh "$RUNE_PI_HOST" "
set -eu
cd '$RUNE_PI_DIR'
cargo build --release
sudo install -m 0755 target/release/rune-ui-demo '$RUNE_PI_BIN'
sudo cp systemd/rune-ui.service '/etc/systemd/system/$RUNE_PI_SERVICE'
sudo systemctl daemon-reload
sudo systemctl restart '$RUNE_PI_SERVICE'
"

echo "deploy complete"
