#!/bin/bash

set -euo pipefail

GOLEM_API_HOST="http://golem-x.localhost:9006"

function get_profile() {
  curl -Ss -H "Accept: application/json" "${GOLEM_API_HOST}/users/${1?}"
}

function compare_json() {
  ACTUAL=$(echo "${1?}" | jq -S)
  EXPECTED=$(echo "${2?}" | jq -S)

  diff --color=always -u <(echo $ACTUAL) <(echo $EXPECTED) | (grep -v '^---' | grep -v '^+++' | grep -v '^@@' || true)
}

if [ "${BASH_SOURCE[0]}" == "${0}" ]; then
  if [ $# -lt 1 ]; then
    echo "Usage: $0 <get_profile> [args...]"
    exit 1
  fi

  case ${1} in
    get_profile)
      get_profile "${2?}"
      ;;
    *)
      echo "Invalid argument: ${1}"
      exit 1
      ;;
  esac
fi