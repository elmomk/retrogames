#!/bin/bash
cd "$(dirname "$0")/.."

SERVICE="${1:-}"
LINES="${2:-50}"

if [ -n "$SERVICE" ]; then
  docker compose logs --tail="$LINES" "$SERVICE"
else
  docker compose logs --tail="$LINES"
fi
