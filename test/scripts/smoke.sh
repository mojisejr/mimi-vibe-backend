#!/usr/bin/env bash
set -euo pipefail
HOST="${1:-http://localhost:8080}"
HEALTH="${HOST%/}/health"

echo "Running smoke checks against: $HOST"

echo "Checking GET $HEALTH"
status=$(curl -s -o /dev/null -w "%{http_code}" "$HEALTH" || true)
if [ "$status" != "200" ]; then
  echo "Health check failed: HTTP $status"
  [ -f server.log ] && echo "----- server.log -----" && tail -n 200 server.log || true
  exit 1
fi
echo "Health OK (200)"

echo "Smoke tests passed"
