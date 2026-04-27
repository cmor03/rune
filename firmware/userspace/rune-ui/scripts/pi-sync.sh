#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
UI_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)

RUNE_PI_HOST=${RUNE_PI_HOST:-user1@rune-proto1.local}
RUNE_PI_DIR=${RUNE_PI_DIR:-/home/user1/src/rune/firmware/userspace/rune-ui}

case "$RUNE_PI_DIR" in
  /Users/*)
    echo "RUNE_PI_DIR is set to a macOS path: $RUNE_PI_DIR" >&2
    echo "Use a path on the Pi, for example:" >&2
    echo "  RUNE_PI_DIR=/home/user1/src/rune/firmware/userspace/rune-ui ./scripts/pi-deploy.sh" >&2
    exit 2
    ;;
esac

echo "syncing $UI_DIR -> $RUNE_PI_HOST:$RUNE_PI_DIR"

ssh "$RUNE_PI_HOST" "mkdir -p '$RUNE_PI_DIR'"
rsync -az --delete \
  --exclude target \
  --exclude .git \
  "$UI_DIR/" \
  "$RUNE_PI_HOST:$RUNE_PI_DIR/"

echo "sync complete"
