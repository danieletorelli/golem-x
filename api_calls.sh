#!/bin/bash

set -euo pipefail

GOLEM_API_HOST="http://golem-x.localhost:9006"

function get_profile() {
  curl -Ss -H "Accept: application/json" "${GOLEM_API_HOST}/users/${1?}"
}

function follow() {
  TARGET="${2?}"
  curl -Ss -H "Accept: application/json" -X POST "${GOLEM_API_HOST}/users/${1?}/follows" -d "{\"user\":\"${TARGET}\"}"
}

function unfollow() {
  TARGET="${2?}"
  curl -Ss -H "Accept: application/json" -X DELETE "${GOLEM_API_HOST}/users/${1?}/follows" -d "{\"user\":\"${TARGET}\"}"
}

function tweet() {
  CONTENT="${2?}"
  curl -Ss -H "Accept: application/json" -X POST "${GOLEM_API_HOST}/users/${1?}/tweets" -d "{\"content\":\"${CONTENT}\"}"
}

function get_tweets() {
  curl -Ss -H "Accept: application/json" "${GOLEM_API_HOST}/users/${1?}/tweets"
}

function get_timeline() {
  curl -Ss -H "Accept: application/json" "${GOLEM_API_HOST}/users/${1?}/timeline"
}

function compare_json() {
  JQ_REMOVE_TIMESTAMPS="if .tweets then .tweets |= map(del(.timestamp)) elif .tweet then .tweet |= del(.timestamp) else . end"
  ACTUAL=$(echo "${1?}" | jq -S "${JQ_REMOVE_TIMESTAMPS}")
  EXPECTED=$(echo "${2?}" | jq -S "${JQ_REMOVE_TIMESTAMPS}")

  diff --color=always -u <(echo $ACTUAL) <(echo $EXPECTED) | (grep -v '^---' | grep -v '^+++' | grep -v '^@@' || true)
}

if [ "${BASH_SOURCE[0]}" == "${0}" ]; then
  if [ $# -lt 1 ]; then
    echo "Usage: $0 <get_profile,follow,unfollow,tweet,get_tweets,get_timeline> [args...]"
    exit 1
  fi

  case ${1} in
    get_profile)
      get_profile "${2?}"
      ;;
    follow)
      follow "${2?}" "${3?}"
      ;;
    unfollow)
      unfollow "${2?}" "${3?}"
      ;;
    tweet)
      tweet "${2?}" "${3?}"
      ;;
    get_tweets)
      get_tweets "${2?}"
      ;;
    get_timeline)
      get_timeline "${2?}"
      ;;
    *)
      echo "Invalid argument: ${1}"
      exit 1
      ;;
  esac
fi