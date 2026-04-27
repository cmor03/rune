#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
UI_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)

if ! command -v fswatch >/dev/null 2>&1; then
    echo "fswatch is required for watch mode."
    echo "On macOS: brew install fswatch"
    exit 1
fi

echo "watching $UI_DIR"
echo "press Ctrl-C to stop"

"$SCRIPT_DIR/pi-deploy.sh"

while fswatch -1 \
    --exclude '/target/' \
    --exclude '/\\.git/' \
    "$UI_DIR" >/dev/null
do
    "$SCRIPT_DIR/pi-deploy.sh"
done
