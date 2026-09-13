#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${BASE_URL:-http://localhost:3000}"
USERNAME="${USERNAME:-aniket}"

echo "Logging in as: ${USERNAME}"

curl -sS -X POST "${BASE_URL}/api/auth/login" \
  -H 'content-type: application/json' \
  -d "{\"username\":\"${USERNAME}\"}"

echo
